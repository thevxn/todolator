export const toDatetimeLocalValue = (timestamp: string) => {
  const date = new Date(timestamp)
  const pad = (n: number) => n.toString().padStart(2, '0')

  const currentDateTime = new Date(Date.now())

  const displayDate = new Date(date.getTime()) // Copy the date
  switch (true) {
    case currentDateTime.getTimezoneOffset() > date.getTimezoneOffset():
      displayDate.setHours(date.getHours() - 1)
      break

    case currentDateTime.getTimezoneOffset() < date.getTimezoneOffset():
      displayDate.setHours(date.getHours() + 1)
      break
  }

  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
    date.getDate()
  )}T${pad(displayDate.getHours())}:${pad(date.getMinutes())}`
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
