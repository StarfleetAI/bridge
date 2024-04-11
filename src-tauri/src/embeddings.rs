// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Context;
use candle_core::{
    utils::{cuda_is_available, metal_is_available},
    Device, Tensor,
};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::tokio::Api, Repo, RepoType};
use regex::Regex;
use tokenizers::{PaddingParams, Tokenizer};
use tracing::{debug, error, info, instrument};

use crate::types::Result;

const CONFIG_FILENAME: &str = "config.json";
const TOKENIZER_FILENAME: &str = "tokenizer.json";
const WEIGHTS_FILENAME: &str = "model.safetensors";

const MARKDOWN_SEPARATORS: [&str; 9] = [
    "\n#{1,6} ",
    "```\n",
    "\n\\*\\*\\*+\n",
    "\n---+\n",
    "\n___+\n",
    "\n\n",
    "\n",
    " ",
    "",
];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error from `candle`: {0}")]
    Candle(#[from] candle_core::Error),
    #[error("error from `tokenizers`: {0}")]
    Tokenizer(#[from] tokenizers::Error),
    #[error("error from `hf_hub::api`: {0}")]
    HfHubApi(#[from] hf_hub::api::tokio::ApiError),
    #[error("unexpected split level for markdown document: {0}")]
    UnexpectedMarkdownSplitLevel(usize),
    #[error("cannot initialize regex: {0}")]
    RegexInit(#[from] regex::Error),
}

pub struct Embeddings {
    pub model_name: String,
    pub max_length: usize,
    device: Device,
    model: BertModel,
    tokenizer: Tokenizer,
}

impl Embeddings {
    /// Initializes the embeddings model.
    ///
    /// # Errors
    ///
    /// Will return an error if the model can't be initialized.
    pub async fn init(model_name: impl Into<String>, max_length: usize) -> Result<Self> {
        let model_name = model_name.into();
        let device = Self::device()?;
        info!(
            "Initializing embeddings with model: `{}` on device: `{:?}`",
            model_name, device
        );

        // TODO: support revisions via the `Repo::with_revision`
        let repo = Repo::new(model_name.clone(), RepoType::Model);

        let (config_filename, tokenizer_filename, weights_filename) =
            Self::model_files(repo).await?;

        let config = std::fs::read_to_string(config_filename)?;
        let config: Config = serde_json::from_str(&config).context("Failed to parse config")?;

        let mut tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(Error::Tokenizer)?;

        let pp = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            ..Default::default()
        };
        tokenizer.with_padding(Some(pp));

        let tp = tokenizers::TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer
            .with_truncation(Some(tp))
            .map_err(Error::Tokenizer)?;

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)
                .map_err(Error::Candle)?
        };

        let model = BertModel::load(vb, &config).map_err(Error::Candle)?;

        Ok(Self {
            model_name,
            max_length,
            device,
            model,
            tokenizer,
        })
    }

    /// Embeds a piece of text.
    ///
    /// # Errors
    ///
    /// Will return an error if the text can't be split into sentences or if the text can't be embedded.
    #[instrument(skip(self, text))]
    pub fn embed<'a>(&'a self, text: &'a str) -> Result<HashMap<&'a str, Vec<f32>>> {
        self.embed_sentences(self.split_text(text, 0)?)
    }

    /// Embeds a list of sentences.
    ///
    /// # Errors
    ///
    /// Will return an error if the sentences can't be tokenized or if the embeddings can't be generated.
    #[instrument(skip(self, sentences))]
    pub fn embed_sentences<'a>(
        &self,
        sentences: Vec<&'a str>,
    ) -> Result<HashMap<&'a str, Vec<f32>>> {
        debug!("Embedding {} sentences", sentences.len());

        let mut results: HashMap<_, _> = HashMap::new();

        // TODO: Configure `chunk_size` via [`Settings`]
        for chunk in sentences.chunks(32) {
            let token_ids = self.tokenize_batch(chunk)?;
            let token_type_ids = token_ids.zeros_like().map_err(Error::Candle)?;

            let embeddings = self
                .model
                .forward(&token_ids, &token_type_ids)
                .map_err(Error::Candle)?;

            // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
            let (_n_sentences, n_tokens, _hidden_size) =
                embeddings.dims3().map_err(Error::Candle)?;

            #[allow(clippy::cast_precision_loss)]
            let embeddings = (embeddings.sum(1).map_err(Error::Candle)? / (n_tokens as f64))
                .map_err(Error::Candle)?;

            let embeddings = Self::normalize_l2(&embeddings)?;

            for (i, sentence) in chunk.iter().enumerate() {
                let sentence_emb = embeddings
                    .get(i)
                    .map_err(Error::Candle)?
                    .to_vec1()
                    .map_err(Error::Candle)?;

                results.insert(*sentence, sentence_emb);
            }
        }

        Ok(results)
    }

    // TODO: this `split_level` thing is a bit hacky, we should probably use a more robust approach
    //       to catch any possible errors at compile time instead of having a runtime check.
    //
    // TODO: this function will break if there is any code block in the text with a Markdown inside.
    //       The solution would be to use a Markdown parser to extract code blocks and process them
    //       separately.
    fn split_text<'a>(&'a self, text: &'a str, split_level: usize) -> Result<Vec<&'a str>> {
        if split_level >= MARKDOWN_SEPARATORS.len() {
            return Ok(vec![text]);
        }

        let re = Regex::new(MARKDOWN_SEPARATORS[split_level]).map_err(Error::RegexInit)?;

        let sentences: Vec<_> = re
            .split(text)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        // TODO: on level 6 and above, we should collapse the sentences to fit in the max_length
        //       to avoid too many small sentences

        Ok(sentences
            .into_iter()
            .flat_map(|sentence| {
                let length = self.sentence_tokens_len(sentence);

                if length > self.max_length {
                    match self.split_text(sentence, split_level + 1) {
                        Ok(sentences) => sentences,
                        Err(err) => {
                            error!("Failed to split sentence `{}`: {}", sentence, err);
                            vec![sentence]
                        }
                    }
                } else {
                    vec![sentence]
                }
            })
            .collect())
    }

    fn sentence_tokens_len(&self, sentence: &str) -> usize {
        match self.tokenize(sentence) {
            Ok(token_ids) => match token_ids.to_vec1::<f32>() {
                Ok(tokens) => tokens.len(),
                Err(_) => sentence.len(),
            },
            Err(_) => sentence.len(),
        }
    }

    fn tokenize(&self, sentence: &str) -> Result<Tensor> {
        let tokens = self
            .tokenizer
            .encode(sentence, true)
            .map_err(Error::Tokenizer)?;
        let token_ids = Tensor::new(tokens.get_ids(), &self.device).map_err(Error::Candle)?;

        Ok(token_ids)
    }

    fn tokenize_batch(&self, sentences: &[&str]) -> Result<Tensor> {
        let tokens = self
            .tokenizer
            .encode_batch(sentences.to_vec(), true)
            .map_err(Error::Tokenizer)?;
        let token_ids = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_ids().to_vec();
                Ok(Tensor::new(tokens.as_slice(), &self.device).map_err(Error::Candle)?)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Tensor::stack(&token_ids, 0).map_err(Error::Candle)?)
    }

    fn normalize_l2(v: &Tensor) -> Result<Tensor> {
        Ok(v.broadcast_div(
            &v.sqr()
                .map_err(Error::Candle)?
                .sum_keepdim(1)
                .map_err(Error::Candle)?
                .sqrt()
                .map_err(Error::Candle)?,
        )
        .map_err(Error::Candle)?)
    }

    async fn model_files(repo: Repo) -> Result<(PathBuf, PathBuf, PathBuf)> {
        let api = Api::new().map_err(Error::HfHubApi)?;
        let api = api.repo(repo);
        let config = api.get(CONFIG_FILENAME).await.map_err(Error::HfHubApi)?;
        let tokenizer = api.get(TOKENIZER_FILENAME).await.map_err(Error::HfHubApi)?;
        let weights = api.get(WEIGHTS_FILENAME).await.map_err(Error::HfHubApi)?;

        Ok((config, tokenizer, weights))
    }

    fn device() -> Result<Device> {
        if cuda_is_available() {
            Ok(Device::new_cuda(0).map_err(Error::Candle)?)
        } else if metal_is_available() {
            Ok(Device::new_metal(0).map_err(Error::Candle)?)
        } else {
            Ok(Device::Cpu)
        }
    }
}
