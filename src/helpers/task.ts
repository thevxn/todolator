import { Ref } from 'vue'
import { DateTimeString, INewTask } from '../composables/useTasks'
import { toDatetimeLocalValue } from './datetime'

export const resetTask = (task: Ref<INewTask | undefined>) => {
  if (task) {
    task.value = {
      name: '',
      desc: undefined,
      timestamp: toDatetimeLocalValue(new Date(Date.now()).toISOString()) as DateTimeString,
    }
    console.log('Task reset: ', task)
  }
}
