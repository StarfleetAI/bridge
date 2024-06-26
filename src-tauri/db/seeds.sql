-- Copyright 2024 StarfleetAI
-- SPDX-License-Identifier: Apache-2.0

INSERT INTO companies (
    id, auth_id, name, slug, created_at, updated_at
) VALUES
    (0, 'personal', 'Personal', 'personal', '2024-03-19T04:20:00.230289+00:00', '2024-03-19T04:20:00.230289+00:00')
ON CONFLICT (id) DO UPDATE SET
    auth_id = excluded.auth_id,
    name = excluded.name,
    slug = excluded.slug,
    updated_at = '2024-03-19T04:20:00.230289+00:00';

INSERT INTO users (
    company_id, first_name, last_name, created_at, updated_at
) VALUES
    (0, 'User', 'Bridge', '2024-03-19T04:20:00.230289+00:00', '2024-03-19T04:20:00.230289+00:00')
ON CONFLICT DO NOTHING;

INSERT INTO agents (
    id, company_id, name, description, system_message, is_enabled, is_code_interpreter_enabled,
    created_at, updated_at
) VALUES
    (1, 0, 'Bridge', 'Your helpful assistant', 'You are an assistant for the "Bridge" - autonomous AI agents IDE, developed by StarfleetAI. Your role is to help user with his tasks.', TRUE, TRUE, '2024-03-19T04:20:00.230289+00:00', '2024-03-19T04:20:00.230289+00:00')
ON CONFLICT (id) DO UPDATE SET
    name = excluded.name,
    description = excluded.description,
    system_message = excluded.system_message,
    is_enabled = excluded.is_enabled,
    is_code_interpreter_enabled = excluded.is_code_interpreter_enabled,
    updated_at = '2024-03-29T04:20:00.230289+00:00';

INSERT INTO models (
    company_id, provider, name, context_length, max_tokens,
    text_in, text_out, image_in, image_out, audio_in, audio_out, function_calling,
    created_at, updated_at
) VALUES
    (0, 'OpenAI', 'gpt-4', 8192, 4096, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'OpenAI', 'gpt-4-turbo-preview', 128000, 4096, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'OpenAI', 'gpt-4-vision-preview', 128000, 4096, TRUE, TRUE, TRUE, FALSE, FALSE, FALSE, TRUE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'OpenAI', 'gpt-3.5-turbo', 16385, 4096, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, TRUE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'OpenAI', 'dall-e-3', 0, 0, TRUE, FALSE, FALSE, TRUE, FALSE, FALSE, FALSE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'Groq', 'llama2-70b-4096', 4096, 4096, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'Groq', 'mixtral-8x7b-32768', 32768, 32768, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, '2024-03-14T01:13:28.672978+00:00', '2024-03-14T01:13:28.672978+00:00'),
    (0, 'Groq', 'gemma-7b-it', 8192, 8192, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, '2024-03-23T01:13:28.672978+00:00', '2024-03-23T01:13:28.672978+00:00'),
    (0, 'Groq', 'llama3-8b-8192', 8192, 8192, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, '2024-03-23T01:13:28.672978+00:00', '2024-03-23T01:13:28.672978+00:00'),
    (0, 'Groq', 'llama3-70b-8192', 8192, 8192, TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, '2024-03-23T01:13:28.672978+00:00', '2024-03-23T01:13:28.672978+00:00')
ON CONFLICT (company_id, provider, name) DO UPDATE SET
    context_length = excluded.context_length,
    max_tokens = excluded.max_tokens,
    text_in = excluded.text_in,
    text_out = excluded.text_out,
    image_in = excluded.image_in,
    image_out = excluded.image_out,
    audio_in = excluded.audio_in,
    audio_out = excluded.audio_out,
    function_calling = excluded.function_calling,
    updated_at = '2024-03-23T01:13:28.672978+00:00';
