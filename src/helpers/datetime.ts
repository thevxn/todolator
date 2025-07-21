export const toDatetimeLocalValue = (timestamp: string) => {
  console.log('received timestamp: ', timestamp)
  const date = new Date(timestamp)
  const pad = (n: number) => n.toString().padStart(2, '0')

  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
    date.getDate(),
  )}T${pad(date.getHours())}:${pad(date.getMinutes())}`
}

export const convertMinutesToHighestUnit = (minutes: number) => {
  const units = {
    hour: 60,
    day: 24 * 60,
    week: 24 * 60 * 7,
    month: 24 * 60 * 7,
  }

  // try % starting from the highest unit, when you get 0, thats the unit to return
}
