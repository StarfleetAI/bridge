// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import hljs from 'highlight.js'
import 'highlight.js/styles/atom-one-dark.min.css'
export const highlightCode = (container: HTMLElement) => {
  container.querySelectorAll('a').forEach((el) => {
    el.setAttribute('target', '_blank')
    el.setAttribute('rel', 'noopener noreferrer')
  })

  container.querySelectorAll('pre code').forEach((el) => {
    if (el.getAttribute('data-highlighted') !== 'yes') {
      // Add data-language attribute to show it in the highlighter
      const lang = el.className
        .split(' ')
        .find((item) => item.startsWith('language-'))
        ?.slice(9)

      if (lang) {
        if (!hljs.getLanguage(lang)) {
          el.classList.value = 'language-html'
        }
        el.parentElement?.setAttribute('data-language', lang)

        hljs.highlightElement(el as HTMLElement)
      }
    }
  })
}
