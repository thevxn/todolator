import { invoke } from '@tauri-apps/api/core'
import { Ref, ref } from 'vue'

export type DateTimeString = Branded<string, 'DateTimeString'>

export interface TaskDefinition {
  name: string
  desc?: string
  start: DateTimeString
  recurrence?: {
    last_recurrence?: DateTimeString
    minutes: number
    exceptions?: Array<DateTimeString>
  }
}
export type TaskInstance = {
  definition_id: string
  name: string
  desc?: string
  timestamp: DateTimeString
}

export const useTasks = () => {
  const tasks = ref([]) as Ref<Array<TaskInstance>>
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

  const createTask = async (task: TaskDefinition) => {
    if (!task.name || !task.start) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }

    try {
      const taskToSave: TaskDefinition = {
        name: task.name,
        desc: task.desc,
        start: new Date(task.start).toISOString() as DateTimeString,
        recurrence: task.recurrence
          ? {
              minutes: task.recurrence?.minutes,
              exceptions: task.recurrence?.exceptions,
              last_recurrence: task.recurrence?.last_recurrence,
            }
          : undefined,
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

  const updateTask = async (task: TaskInstance) => {
    if (!task.name || !task.timestamp) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }
    const taskToSave = {
      id: task.definition_id,
      name: task.name,
      desc: task.desc,
      timestamp: new Date(task.timestamp).toISOString(),
    }
    console.log('Updating task: ', taskToSave)

    await invoke('update_task', { task: taskToSave })
    console.log('task updated!')

    toggleTaskModal()
    await loadTasks()
  }

  const deleteTask = async (id: string) => {
    console.log('Deleting task with ID ', id)
    await invoke('delete_task', { id })
    console.log('Task deleted')
    await loadTasks()
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
