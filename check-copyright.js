// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import fs from 'fs'
import path from 'path'

const rootDirectory = path.resolve('.')
const excludedFolders = ['node_modules', '.github', '.husky', '.idea', '.nuxt', '.vscode', 'assets', 'src-tauri/target']
const excludedFullPaths = new Set(excludedFolders.map((folder) => path.resolve(rootDirectory, folder)))

const commentPatterns = {
  '.js': /\/\/ Copyright 2024 StarfleetAI[\s\S]*\/\/ SPDX-License-Identifier: Apache-2.0/,
  '.ts': /\/\/ Copyright 2024 StarfleetAI[\s\S]*\/\/ SPDX-License-Identifier: Apache-2.0/,
  '.rs': /\/\/ Copyright 2024 StarfleetAI[\s\S]*\/\/ SPDX-License-Identifier: Apache-2.0/,
  '.vue': /<!-- Copyright 2024 StarfleetAI -->[\s\S]*<!-- SPDX-License-Identifier: Apache-2.0 -->/,
}

const filesWithoutComments = []

function searchDirectory(directory) {
  const filesAndFolders = fs.readdirSync(directory)

  for (const fileOrFolder of filesAndFolders) {
    const fullPath = path.resolve(directory, fileOrFolder)
    if (excludedFullPaths.has(fullPath)) {
      continue
    }

    const stat = fs.statSync(fullPath)
    if (stat.isDirectory()) {
      searchDirectory(fullPath)
    } else {
      checkFile(fullPath)
    }
  }
}

function checkFile(filePath) {
  const fileExtension = path.extname(filePath)
  if (commentPatterns[fileExtension]) {
    const fileContent = fs.readFileSync(filePath, 'utf8')
    const pattern = commentPatterns[fileExtension]
    if (!pattern.test(fileContent)) {
      filesWithoutComments.push(filePath)
    }
  }
}

searchDirectory(rootDirectory)

if (filesWithoutComments.length > 0) {
  console.log('The following files are missing the required copyright notices:')
  filesWithoutComments.forEach((file) => {
    console.log(`- ${file}`)
  })
  throw 'Missing copyright notices'
} else {
  console.log('All files contain the required copyright notices.')
}
