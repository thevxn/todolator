import {
  CreatedTaskDefinition,
  DateTimeString,
  TaskDefinition,
  UuidString,
} from '../composables/useTasks'
import { toDatetimeLocalValue } from './datetime'

export function isRecurringDefinition(task: TaskDefinition | CreatedTaskDefinition) {
  console.log('recurrence' in task && task.recurrence !== null)
  return 'recurrence' in task && task.recurrence !== null
}

export function isExistingDefinition(
  task: TaskDefinition | CreatedTaskDefinition,
): task is CreatedTaskDefinition {
  return 'id' in task && task.id !== ''
}

export const getDefaultTaskDefinition = (): TaskDefinition => {
  return {
    id: '' as UuidString,
    name: '',
    desc: null,
    start: toDatetimeLocalValue(new Date(Date.now()).toISOString()) as DateTimeString,
    recurrence: null,
  }
}
