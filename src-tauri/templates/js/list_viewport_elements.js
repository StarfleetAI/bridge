// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

let idCounter = 1
const resultArray = []

// Helper function to check if an element is visible and within the viewport
function isVisibleAndInViewport(element) {
  const rect = element.getBoundingClientRect()
  const inViewport =
    rect.top < window.innerHeight && rect.left < window.innerWidth && rect.bottom > 0 && rect.right > 0

  const style = window.getComputedStyle(element)
  const visible =
    style.display !== 'none' && style.visibility !== 'hidden' && element.offsetWidth > 0 && element.offsetHeight > 0

  return visible && inViewport
}

function processElement(element) {
  if (!element.hasAttribute('data-sfai')) {
    element.setAttribute('data-sfai', idCounter++)
  }

  const elementTag = element.tagName.toLowerCase()
  const elementType = element.getAttribute('type')
  let type = null
  let content = ''

  // Determine the type and content based on the element
  if (elementTag === 'a') {
    type = 'link'
    content = element.textContent.trim() || element.getAttribute('title')
  } else if (
    elementTag === 'button' ||
    (elementTag === 'input' && (elementType === 'button' || elementType === 'submit'))
  ) {
    type = 'button'
    content = element.textContent.trim() || element.value.trim()
  } else if (elementTag === 'input') {
    type = 'input'

    content = element.value.trim() ||
      element.getAttribute('aria-label') ||
      element.getAttribute('placeholder') ||
      element.getAttribute('name')
  } else if (elementTag === 'textarea') {
    type = 'input'

    content = element.value.trim() ||
      element.getAttribute('placeholder') ||
      element.getAttribute('aria-label') ||
      element.getAttribute('name')
  }

  // If it's a specified type, add it to the result array
  if (type) {
    if (isVisibleAndInViewport(element)) {
      resultArray.push({
        id: parseInt(element.getAttribute('data-sfai')),
        type,
        content,
      })
    }
  } else {
    // Check if the element has text child nodes
    const hasTextNodes = Array.from(element.childNodes).some(
      (node) => node.nodeType === Node.TEXT_NODE && node.textContent.trim().length > 0,
    )

    if (hasTextNodes && isVisibleAndInViewport(element)) {
      // Add it as a text node
      resultArray.push({
        id: parseInt(element.getAttribute('data-sfai')),
        type: 'text',
        content: element.textContent.trim(),
      })
    }
  }

  Array.from(element.children).forEach((child) => {
    processElement(child)
  })
}

processElement(document.body)

return resultArray
