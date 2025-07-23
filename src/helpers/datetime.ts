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

export const enum TimeUnitEnum {
  NAME,
  MINUTES
}
export const timeUnitToMinutesMap = {
  minute,
  hour,
  day,
  week,
  month,
  year
} as const

export const timeUnits = [year, month, week, day, hour, minute] as const
export const convertMinutesToHighestUnit = (minutes: number | undefined) => {
  if (!minutes) return

  let foundUnit: keyof typeof timeUnitToMinutesMap | undefined
  let foundAmount
  for (const unitAmount of timeUnits) {
    if (minutes % unitAmount === 0) {
      const foundUnitObj = Object.entries(timeUnitToMinutesMap).find(
        ({ 1: minutes }) => minutes === unitAmount
      )

      if (foundUnitObj) {
        foundUnit = foundUnitObj[TimeUnitEnum.NAME] as keyof typeof timeUnitToMinutesMap
      }

      foundAmount = minutes / unitAmount

      break
    }
  }

  if (foundUnit && foundAmount && foundAmount > 1) {
    pluralizeUnit(foundUnit)
  }

  console.log('Found unit: ', foundUnit)

  return [foundAmount, foundUnit?.toLowerCase()]
}

export const pluralizeUnit = (unit: string) => unit + 's'
