import { Ref } from 'vue'
import { ITask } from '../composables/useTasks'

export const resetTask = (task: Ref<ITask | undefined>) => {
  if (task) {
    task.value = {
      name: '',
      desc: undefined,
      timestamp: '',
    }
  }
}
