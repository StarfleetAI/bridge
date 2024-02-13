import { createChat } from '../api'

export const useCreateChat = (agent_id: number) => {
  const router = useRouter()
  const createNewChat = async () => {
    const newChat = await createChat({ agent_id })
    router.push({ name: 'chats-id', params: { id: newChat.id } })
    return this
  }

  return {
    createNewChat
  }
}
