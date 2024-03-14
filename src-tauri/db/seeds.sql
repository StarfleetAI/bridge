-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

INSERT INTO models (
    provider, name, context_length, max_tokens,
    text_in, text_out, image_in, image_out, audio_in, audio_out, function_calling,
    is_system, created_at, updated_at
) VALUES
    ('OpenAI', 'gpt-4', 8192, 4096, 1, 1, 0, 0, 0, 0, 1, 1, strftime('%Y-%m-%dT%H:%M:%S', 'now'), strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    ('OpenAI', 'gpt-4-turbo-preview', 128000, 4096, 1, 1, 0, 0, 0, 0, 1, 1, strftime('%Y-%m-%dT%H:%M:%S', 'now'), strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    ('OpenAI', 'gpt-4-vision-preview', 128000, 4096, 1, 1, 1, 0, 0, 0, 1, 1, strftime('%Y-%m-%dT%H:%M:%S', 'now'), strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    ('OpenAI', 'gpt-3.5-turbo', 16385, 4096, 1, 1, 0, 0, 0, 0, 1, 1, strftime('%Y-%m-%dT%H:%M:%S', 'now'), strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    ('OpenAI', 'dall-e-3', 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, strftime('%Y-%m-%dT%H:%M:%S', 'now'), strftime('%Y-%m-%dT%H:%M:%S', 'now')),
    ('Groq', 'llama2-70b-4096', 4096, 4096, 1, 1, 0, 0, 0, 0, 0, 1, strftime('%Y-%m:%dT%H:%M:%S', 'now'), strftime('%Y-%m:%dT%H:%M:%S', 'now')),
    ('Groq', 'mixtral-8x7b-32786', 32768, 32768, 1, 1, 0, 0, 0, 0, 0, 1, strftime('%Y-%m:%dT%H:%M:%S', 'now'), strftime('%Y-%m:%dT%H:%M:%S', 'now'))
ON CONFLICT (provider, name) DO UPDATE SET
    context_length = excluded.context_length,
    max_tokens = excluded.max_tokens,
    text_in = excluded.text_in,
    text_out = excluded.text_out,
    image_in = excluded.image_in,
    image_out = excluded.image_out,
    audio_in = excluded.audio_in,
    audio_out = excluded.audio_out,
    function_calling = excluded.function_calling,
    updated_at = strftime('%Y-%m-%dT%H:%M:%S', 'now')
WHERE models.is_system = 1;
