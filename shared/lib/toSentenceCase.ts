export const toSentenceCase = (str: string) => {
  return str
    .replace(/([A-Z])/g, ' $1')
    .replace(/_/g, ' ')
    .trim()
    .replace(/\s\s+/g, ' ')
    .toLowerCase()
    .replace(/(^\s*\w|[.!?]\s*\w)/g, function (firstLetter) {
      return firstLetter.toUpperCase()
    })
}
