import { invoke } from '@tauri-apps/api/core'
import { Ref, ref } from 'vue'

export type UuidString = Branded<string, 'Uuidstring'>
export type DateTimeString = Branded<string, 'DateTimeString'>

export interface CreatedTaskDefinition {
  name: string
  desc: string | null
  start: DateTimeString
  recurrence: {
    last_recurrence: DateTimeString | null
    minutes: number
    exceptions: Array<DateTimeString> | null
  } | null
}

export type TaskDefinition = CreatedTaskDefinition & { id: UuidString }
export type TaskInstance = {
  definition_id: UuidString
  name: string
  desc?: string
  timestamp: DateTimeString
  // window_spawned?
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

  const createTask = async (taskDefinition: CreatedTaskDefinition) => {
    if (!taskDefinition.name || !taskDefinition.start) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }

    try {
      const taskDefinitionToSave: CreatedTaskDefinition = {
        name: taskDefinition.name,
        desc: taskDefinition.desc,
        start: new Date(taskDefinition.start).toISOString() as DateTimeString,
        recurrence: taskDefinition.recurrence
          ? {
              minutes: taskDefinition.recurrence.minutes,
              exceptions: taskDefinition.recurrence.exceptions,
              last_recurrence: new Date(
                taskDefinition.recurrence.last_recurrence as DateTimeString,
              ).toISOString() as DateTimeString,
            }
          : null,
      }
      console.log('Adding task: ', taskDefinitionToSave)

      // If the task does not have an ID, it's a create
      await invoke('create_task', { ...taskDefinitionToSave })
      console.log('task created!')

      toggleTaskModal()
      await loadTasks()
    } catch (e) {
      console.log(e)
    }
  }

  const updateTask = async (taskDefinition: TaskDefinition) => {
    if (!taskDefinition.name || !taskDefinition.start || !taskDefinition.id) {
      console.log('missing required attributes')
      taskCreationError.value = 'Missing required attributes!'
      return
    }
    const taskDefinitionToSave: TaskDefinition = {
      id: taskDefinition.id,
      name: taskDefinition.name,
      desc: taskDefinition.desc,
      start: new Date(taskDefinition.start).toISOString() as DateTimeString,
      recurrence: taskDefinition.recurrence
        ? {
            minutes: taskDefinition.recurrence.minutes,
            exceptions: taskDefinition.recurrence.exceptions,
            last_recurrence: new Date(
              taskDefinition.recurrence.last_recurrence as DateTimeString,
            ).toISOString() as DateTimeString,
          }
        : null,
    }
    console.log('Updating task: ', taskDefinitionToSave)

    await invoke('update_task', { task: taskDefinitionToSave })
    console.log('task updated!')

    toggleTaskModal()
    await loadTasks()
  }

  const deleteTask = async (id: UuidString) => {
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
