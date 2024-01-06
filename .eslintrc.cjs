// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

module.exports = {
  extends: [
    '@nuxtjs/eslint-config-typescript'
  ],
  ignorePatterns: [
    'src-tauri/**/*'
  ],
  overrides: [
    {
      files: ['*.js', '*.ts', '*.vue'],
      rules: {
        'space-before-function-paren': 0
      }
    }
  ]
}
