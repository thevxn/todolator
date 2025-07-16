import { Ref } from 'vue'
import { DateTimeString, INewTask } from '../composables/useTasks'
import { toDatetimeLocalValue } from './datetime'

export const getDefaultTask = (): INewTask => {
  return {
    name: '',
    desc: undefined,
    timestamp: toDatetimeLocalValue(new Date(Date.now()).toISOString()) as DateTimeString,
  }
}
export const resetTask = (task: Ref<INewTask | undefined>) => {
  if (task) {
    task.value = getDefaultTask()
    console.log('Task reset: ', task)
  } else {
    console.log('No task')
  }
}
