export const toDatetimeLocalValue = (timestamp: string) => {
  console.log('received timestamp: ', timestamp)
  const date = new Date(timestamp)
  const pad = (n: number) => n.toString().padStart(2, '0')

  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(
    date.getDate()
  )}T${pad(date.getHours())}:${pad(date.getMinutes())}`
}

export const convertMinutesToHighestUnit = (minutes: number | undefined) => {
  if (!minutes) return
  enum UnitMinutes {
    HOUR = 60,
    DAY = HOUR * 24,
    WEEK = DAY * 7,
    MONTH = WEEK * 4,
    YEAR = MONTH * 12,
  }

  let foundUnit = 'minutes'
  let foundAmount = minutes
  for (const unit in UnitMinutes) {
    const unitMinutes = UnitMinutes[unit as keyof typeof UnitMinutes]
    if (minutes % unitMinutes === 0) {
      foundUnit = unit
      foundAmount = minutes / unitMinutes

      if (foundAmount > 1) {
        foundUnit += 's'
      }

      break
    }
  }

  console.log('Found unit: ', foundUnit)

  return [foundAmount, foundUnit.toLowerCase()]
}
