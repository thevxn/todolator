import { Ref } from 'vue'
import { DateTimeString, TaskDefinition, TaskInstance } from '../composables/useTasks'
import { toDatetimeLocalValue } from './datetime'

export function isTaskInstance(task: TaskDefinition | TaskInstance): task is TaskInstance {
  return 'definition_id' in task
}

export const getDefaultTaskDefinition = (): TaskDefinition => {
  return {
    name: '',
    desc: undefined,
    start: toDatetimeLocalValue(new Date(Date.now()).toISOString()) as DateTimeString,
    recurrence: undefined,
  }
}
export const resetTask = (task: Ref<TaskInstance | undefined>) => {
  if (task) {
    task.value = undefined
  }
}
