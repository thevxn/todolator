export const toDatetimeLocalValue = (timestamp: string) => {
  const date = new Date(timestamp)
  const pad = (n: number) => n.toString().padStart(2, '0')

  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
    date.getDate()
  )}T${pad(date.getHours())}:${pad(date.getMinutes())}`
}

const minute = 1
const hour = minute * 60
const day = hour * 24
const week = day * 7
const month = week * 4
const year = month * 12

export const timeUnitToMinutesMap = {
  minute,
  hour,
  day,
  week,
  month,
  year
} as const

export const pluralizeUnit = (unit: string) => unit + 's'
