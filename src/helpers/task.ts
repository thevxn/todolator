import {
  CreatedTaskDefinition,
  DateTimeString,
  TaskDefinition,
  UuidString
} from '../composables/useTasks'
import { toDatetimeLocalValue } from './datetime'

export function isRecurringDefinition(task: TaskDefinition | CreatedTaskDefinition) {
  console.log('recurrence' in task && task.recurrence !== null)
  return 'recurrence' in task && task.recurrence !== null
}

export function isExistingDefinition(
  task: TaskDefinition | CreatedTaskDefinition
): task is CreatedTaskDefinition {
  return 'id' in task && task.id !== ''
}

export const getDefaultTaskDefinition = (): TaskDefinition => {
  return {
    id: '' as UuidString,
    name: '',
    desc: null,
    start: toDatetimeLocalValue(
      new Date(Date.now()).toLocaleString('sv').replace(' ', 'T') as DateTimeString
    ) as DateTimeString,
    recurrence: null
  }
}

export const getDefaultRecurrence = (): TaskDefinition['recurrence'] => {
  return {
    exceptions: null,
    last_recurrence: null,
    interval: {
      Daily: 1
    }
  }
}
