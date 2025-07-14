import { Ref } from 'vue'
import { DateTimeString, INewTask } from '../composables/useTasks'

export const resetTask = (task: Ref<INewTask | undefined>) => {
  if (task) {
    task.value = {
      name: '',
      desc: undefined,
      timestamp: '' as DateTimeString,
    }
  }
}
