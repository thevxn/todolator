import { invoke } from '@tauri-apps/api/core'
import { Ref, ref } from 'vue'

export type DateTimeString = Branded<string, 'DateTimeString'>

export interface INewTask {
  name: string
  desc?: string
  timestamp: DateTimeString
  recurrence_minutes?: DateTimeString
}
export type Task = INewTask & {
  id: string
}

export const useTasks = () => {
  const tasks = ref([]) as Ref<Array<Task>>
  const displayTaskModal = ref(false)
  const taskCreationError = ref(undefined) as Ref<string | undefined>
  const displayConfirmationModal = ref(false)

  const loadTasks = async () => {
    try {
      tasks.value = await invoke('get_tasks')
      console.log('tasks loaded')

      console.log(tasks.value)
    } catch (e) {
      console.log(e)
    }
  }

  const createTask = async (task: INewTask) => {
    if (!task.name || !task.timestamp) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }

    try {
      const taskToSave: INewTask = {
        name: task.name,
        desc: task.desc,
        timestamp: new Date(task.timestamp).toISOString() as DateTimeString,
        recurrence_minutes: task.recurrence_minutes,
      }
      console.log('Adding task: ', taskToSave)

      // If the task does not have an ID, it's a create
      await invoke('create_task', { ...taskToSave })
      console.log('task created!')

      toggleTaskModal()
      await loadTasks()
    } catch (e) {
      console.log(e)
    }
  }

  const updateTask = async (task: Task) => {
    if (!task.name || !task.timestamp) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }
    try {
      const taskToSave = {
        id: task.id,
        name: task.name,
        desc: task.desc,
        timestamp: new Date(task.timestamp).toISOString(),
      }
      console.log('Updating task: ', taskToSave)

      await invoke('update_task', { task: taskToSave })
      console.log('task updated!')

      toggleTaskModal()
      await loadTasks()
    } catch (e) {
      console.log(e)
    }
  }

  const deleteTask = async (id: string) => {
    try {
      console.log('Deleting task with ID ', id)
      await invoke('delete_task', { id })
      await loadTasks()
    } catch (e) {
      console.log(e)
    }
  }

  const toggleTaskModal = () => {
    taskCreationError.value = undefined
    displayTaskModal.value = !displayTaskModal.value
  }

  const toggleConfirmationModal = () => {
    // taskCreationError.value = undefined;
    displayConfirmationModal.value = !displayConfirmationModal.value
  }

  return {
    tasks,
    displayTaskModal,
    taskCreationError,
    displayConfirmationModal,
    loadTasks,
    toggleTaskModal,
    toggleConfirmationModal,
    createTask,
    updateTask,
    deleteTask,
  }
}
