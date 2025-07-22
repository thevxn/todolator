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

  interface TimeUnit {
    name: string
    minutes: number
  }

  const hour: TimeUnit = { name: 'hour', minutes: 60 }
  const day: TimeUnit = { name: 'day', minutes: hour.minutes * 24 }
  const week: TimeUnit = { name: 'week', minutes: day.minutes * 7 }
  const month: TimeUnit = { name: 'month', minutes: week.minutes * 4 }
  const year: TimeUnit = { name: 'year', minutes: month.minutes * 12 }

  const timeUnits: TimeUnit[] = [year, month, week, day, hour]

  let foundUnit = 'minute'
  let foundAmount = minutes
  for (const unit of timeUnits) {
    const unitMinutes = unit.minutes
    if (minutes % unitMinutes === 0) {
      foundUnit = unit.name
      foundAmount = minutes / unitMinutes

      break
    }
  }

  if (foundAmount > 1) {
    foundUnit += 's'
  }

  console.log('Found unit: ', foundUnit)

  return [foundAmount, foundUnit.toLowerCase()]
}
