import type { Dayjs } from 'dayjs'

export const utcToLocalTime = (createdAt: string): Dayjs => {
  let dateString = createdAt
  if (createdAt.at(-1) !== 'Z') {
    dateString += 'Z'
  }

  const dayjs = useDayjs()

  return dayjs(dateString)
}
