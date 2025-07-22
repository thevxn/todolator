<template>
  <!-- Modal overlay -->
  <div
    class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed"
    v-if="displayTaskModal || displayConfirmationModal"
    @click="closeModals"
  ></div>

  <main class="flex flex-col items-center justify-center p-4">
    <ConfirmationModal
      :display="displayConfirmationModal"
      @submit="handleTaskDelete(currentTaskDefinition?.id)"
      @close="closeModals"
      :name="currentTaskInstance?.name"
      :id="currentTaskInstance?.definition_id"
    />
    <TaskForm
      submit-text="Save"
      :display="displayTaskModal"
      @create="handleTaskCreate"
      @update="handleTaskUpdate"
      @close="toggleTaskModal"
      :error-text="taskCreationError"
      :current-task="currentTaskDefinition"
    />

    <div
      class="flex flex-row w-full items-center mt-10"
      :class="tasks.length > 0 ? 'justify-end' : 'justify-center'"
    >
      <!-- New Task Button -->
      <button tabindex="-1" @click="openCreateUpdateModal(null)">New Task</button>
    </div>

    <!-- Main tasks table -->
    <div
      v-if="tasks.length > 0"
      id="tasks-table"
      class="w-full mt-2 mb-10 rounded-lg overflow-hidden border border-gray-500 min-h-[200px]"
    >
      <div class="flex flex-col h-full">
        <div
          class="grid grid-cols-4 bg-secondary font-bold text-primary flex-shrink-0 border border-b-4 border-secondary"
        >
          <div class="p-2">Title</div>
          <div class="p-2">Description</div>
          <div class="p-2">Remind At</div>
          <div class="p-2"></div>
        </div>

        <div class="flex-1 min-h-0 overflow-y-auto" ref="tableRef">
          <div
            v-for="(task, i) in tasks"
            :key="i"
            :class="[
              'grid grid-cols-4 border-t border-gray-500 custom-row',
              selectedIndex === i ? 'bg-secondary border-secondary text-primary font-bold' : ''
            ]"
            :ref="
              (el) => {
                if (el) rowRefs[i] = el as HTMLElement
              }
            "
          >
            <div class="p-2 flex flex-row gap-x-2 items-center">
              {{ task.name }}
            </div>
            <div class="p-2 flex flex-row gap-x-2 items-center">{{ task.desc || '-' }}</div>
            <div class="p-2 flex flex-row gap-x-2 items-center">
              {{ new Date(task.timestamp).toLocaleString() }}
            </div>
            <div class="p-2 flex flex-row gap-x-2 items-center justify-center">
              <PIcon
                :icon="'mingcute:edit-2-line'"
                class="hover:border-warning hover:border-2 border-2 border-[#ffffff00] p-1 active:bg-warning active:text-primary text-warning rounded-md outline-none"
                @clicked="openCreateUpdateModal(i)"
              />
              <PIcon
                :icon="'mingcute:delete-2-line'"
                class="hover:border-error hover:border-2 border-2 border-[#ffffff00] p-1 active:bg-error active:text-primary text-error rounded-md outline-none"
                @clicked="openDeleteConfirmation(i)"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
  <PHotkeys screen-code="MAIN" />
</template>

<script setup lang="ts">
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import TaskForm from './TaskForm.vue'
import PHotkeys from './PHotkeys.vue'
import {
  DateTimeString,
  TaskDefinition,
  TaskInstance,
  useTasks,
  UuidString
} from '../composables/useTasks'
import { useRowSelect } from '../composables/useRowSelect'
import { toDatetimeLocalValue } from '../helpers/datetime'
import ConfirmationModal from './ConfirmationModal.vue'
import PIcon from './PIcon.vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

const {
  tasks,
  displayTaskModal,
  taskCreationError,
  loadTasks,
  createTask,
  updateTask,
  deleteTask,
  toggleTaskModal,
  toggleConfirmationModal,
  displayConfirmationModal
} = useTasks()

const { selectedIndex, resetSelectedIndex } = useRowSelect(
  () => tasks.value.length,
  () => displayTaskModal.value
)

const rowRefs = ref<HTMLElement[]>([])
const tableRef = ref<HTMLElement | null>(null)

const currentTaskDefinition = ref<TaskDefinition | undefined>()
const currentTaskInstance = ref<TaskInstance | undefined>()

const setCurrentDefinitionAndInstance = async (taskIndex: number) => {
  currentTaskInstance.value = {
    definition_id: tasks.value[taskIndex].definition_id,
    name: tasks.value[taskIndex].name,
    desc: tasks.value[taskIndex].desc,
    timestamp: toDatetimeLocalValue(tasks.value[taskIndex].timestamp) as DateTimeString
  }

  // TODO: Could be moved
  currentTaskDefinition.value = (await invoke('get_task_definition', {
    id: currentTaskInstance.value.definition_id
  })) as TaskDefinition

  // Convert timestamps to client's local time zone
  currentTaskDefinition.value.start = toDatetimeLocalValue(
    currentTaskDefinition.value.start as DateTimeString
  ) as DateTimeString
  if (currentTaskDefinition.value.recurrence) {
    currentTaskDefinition.value.recurrence.last_recurrence = toDatetimeLocalValue(
      currentTaskDefinition.value.recurrence?.last_recurrence as DateTimeString
    ) as DateTimeString
  }
}

const resetCurrentDefinitionAndInstance = () => {
  currentTaskDefinition.value = undefined
  currentTaskInstance.value = undefined
}

// Watcher for selected task index to enable visual task select using hotkeys
// TODO: Fix weird behavior after messing around in the GUI (view being dragged every time arrow up/down is pressed)
watch(selectedIndex, (newIndex) => {
  if (newIndex === null || !tableRef.value) return

  const container = tableRef.value
  const rowElement = rowRefs.value[newIndex]

  if (!rowElement) {
    return
  }

  const containerRect = container.getBoundingClientRect()
  const rowRect = rowElement.getBoundingClientRect()

  if (rowRect.top < containerRect.top) {
    // Row is above visible area
    container.scrollBy({
      top: rowRect.top - containerRect.top - 10,
      behavior: 'smooth'
    })
  } else if (rowRect.bottom > containerRect.bottom) {
    // Row is below visible area
    container.scrollBy({
      top: rowRect.bottom - containerRect.bottom + 10,
      behavior: 'smooth'
    })
  }
})

const openCreateUpdateModal = async (taskIndex: number | null) => {
  console.log('called with taskIndex: ', taskIndex)
  // TODO: Check where the task is being reset, find all places, refactor to be unified..
  resetCurrentDefinitionAndInstance()
  if (taskIndex !== null) {
    await setCurrentDefinitionAndInstance(taskIndex)
  }

  toggleTaskModal()
}

const openDeleteConfirmation = async (taskIndex: number | null) => {
  if (taskIndex !== null) {
    await setCurrentDefinitionAndInstance(taskIndex)

    toggleConfirmationModal()
  }
}

const closeModals = () => {
  nextTick(() => {
    if (!displayConfirmationModal.value && !displayTaskModal.value) {
      resetSelectedIndex()
    }
    if (displayTaskModal.value) {
      toggleTaskModal()
    }
    if (displayConfirmationModal.value) {
      toggleConfirmationModal()
    }
  })
  resetCurrentDefinitionAndInstance()
}

const handleTaskCreate = async (task: TaskDefinition) => {
  console.log('Creating task: ', task)
  await createTask(task)

  resetCurrentDefinitionAndInstance()
}

const handleTaskUpdate = async (task: TaskDefinition) => {
  try {
    await updateTask(task)
  } catch (e) {
    throw new Error(`Failed to update task: ${e}`)
  }
  resetCurrentDefinitionAndInstance()
}

const handleTaskDelete = async (id: UuidString | undefined) => {
  if (!id) {
    return
  }

  try {
    await deleteTask(id)
    resetSelectedIndex()
    resetCurrentDefinitionAndInstance()
    toggleConfirmationModal()
  } catch (e) {
    console.log(`Failed to delete task: ${e}`)
  }
}

onMounted(async () => {
  await loadTasks()

  const handler = async (e: KeyboardEvent) => {
    switch (e.key) {
      case 'n':
        // If no modal is active, open New Task modal
        if (!displayTaskModal.value && !displayConfirmationModal.value) {
          openCreateUpdateModal(null)
          e.stopPropagation()
          e.preventDefault()
        }
        // If Delete Confirmation modal is active, close it without deleting
        if (displayConfirmationModal.value && selectedIndex.value !== null) {
          toggleConfirmationModal()
          e.stopPropagation()
          e.preventDefault()
        }
        break

      case 'Escape':
        e.stopPropagation()
        e.preventDefault()
        closeModals()
        break

      case 'Enter':
        if (
          !displayTaskModal.value &&
          !displayConfirmationModal.value &&
          selectedIndex.value !== null
        ) {
          e.stopPropagation()
          e.preventDefault()
          openCreateUpdateModal(selectedIndex.value)
        }
        break

      case 'Backspace':
        if (!displayTaskModal.value && !displayConfirmationModal.value) {
          e.stopPropagation()
          e.preventDefault()
          openDeleteConfirmation(selectedIndex.value)
        }
        break

      case 'y':
        if (displayConfirmationModal.value && selectedIndex.value !== null) {
          e.stopPropagation()
          e.preventDefault()

          await handleTaskDelete(tasks.value[selectedIndex.value].definition_id as UuidString)
        }
        break

      default:
        break
    }
  }
  window.addEventListener('keydown', handler)
  onUnmounted(() => window.removeEventListener('keydown', handler))
})

type Payload = {
  url: string
}

const l = await listen<Payload>('state-changed', async () => {
  console.log('Received state-changed event, reloading tasks')
  await loadTasks()
})

onUnmounted(() => {
  l()
})
</script>

<style>
/* width */
::-webkit-scrollbar {
  width: 6px;
}

/* Track */
::-webkit-scrollbar-track {
  background: #f3f4f6c0;
  border-radius: 1rem;
}

/* Handle */
::-webkit-scrollbar-thumb {
  background: #55555583;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
  background: #555555d2;
}
</style>
