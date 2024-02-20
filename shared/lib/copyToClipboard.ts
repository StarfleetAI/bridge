import { ClipboardItem as ClipboardItemPolyfill, write as writeToClipboard } from 'clipboard-polyfill'

/**
 * Copies the assignment details from an HTML element to the clipboard.
 *
 * @param { HTMLDivElement } element - The HTML element containing the assignment details.
 * @return {Promise<boolean>} - A promise that resolves to true if the assignment details were successfully copied to the clipboard, otherwise false.
 *
 * **Note:** This function uses a polyfill to support copying to the clipboard in old browsers.
 */
export const copyTextToClipboard = async (content: string) => {
  const tempDiv = document.createElement('div')
  tempDiv.style.display = 'none'
  tempDiv.innerHTML = content
  document.body.appendChild(tempDiv)
  const selection = window.getSelection()
  const range = document.createRange()
  range.selectNodeContents(tempDiv)
  selection?.removeAllRanges()
  selection?.addRange(range)

  let result = false
  try {
    const clipboardItem = new ClipboardItemPolyfill({
      'text/html': new Blob([content], { type: 'text/html' }),
      'text/plain': new Blob([content], { type: 'text/plain' }),
    })
    await writeToClipboard([clipboardItem])

    result = true
  } catch (err) {
    console.log(err)
    result = false
  } finally {
    selection?.removeAllRanges()
    document.body.removeChild(tempDiv)
  }

  return result
}
