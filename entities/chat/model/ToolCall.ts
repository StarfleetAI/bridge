export interface ToolCall {
  id: number
  function: {
    name: string
    arguments: string
  }
  type: string
}
