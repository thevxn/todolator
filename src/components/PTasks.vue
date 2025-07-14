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
      @submit="handleTaskDelete(currentTask.definition_id)"
      @close="closeModals"
      :name="currentTask.name"
      :id="currentTask.definition_id"
    />
    <TaskForm
      submit-text="Save"
      :display="displayTaskModal"
      @submit="handleSave"
      @close="toggleTaskModal"
      :error-text="taskCreationError"
      :current-task="currentTask"
    />
    <div
      class="flex flex-row w-full items-center mt-10"
      :class="tasks.length > 0 ? 'justify-end' : 'justify-center'"
    >
      <button tabindex="-1" @click="toggleTaskModal">New Task</button>
    </div>

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
              selectedIndex === i ? 'bg-secondary border-secondary text-primary font-bold' : '',
            ]"
            :ref="
              (el) => {
                if (el) rowRefs[i] = el as HTMLElement
              }
            "
          >
            <div class="p-2 flex flex-row gap-x-2 items-center">{{ task.name }}</div>
            <div class="p-2 flex flex-row gap-x-2 items-center">{{ task.desc || '-' }}</div>
            <div class="p-2 flex flex-row gap-x-2 items-center">
              {{ new Date(task.timestamp).toLocaleString() }}
            </div>
            <div class="p-2 flex flex-row gap-x-2 items-center justify-center">
              <PIcon
                :icon="'mingcute:edit-2-line'"
                class="hover:border-warning hover:border-2 border-2 border-[#ffffff00] p-1 active:bg-warning active:text-primary text-warning rounded-md outline-none"
                @clicked="openEditModal(i)"
              />
              <PIcon
                :icon="'mingcute:delete-2-line'"
                class="hover:border-error hover:border-2 border-2 border-[#ffffff00] p-1 active:bg-error active:text-primary text-error rounded-md outline-none"
                @clicked="openDeleteModal(i)"
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
import { onMounted, onUnmounted, Ref, ref, watch } from 'vue'
import TaskForm from './TaskForm.vue'
import PHotkeys from './PHotkeys.vue'
import { DateTimeString, Task, useTasks } from '../composables/useTasks'
import { useRowSelect } from '../composables/useRowSelect'
import { toDatetimeLocalValue } from '../helpers/datetime'
import { resetTask } from '../helpers/task'
import ConfirmationModal from './ConfirmationModal.vue'
import PIcon from './PIcon.vue'
import { listen } from '@tauri-apps/api/event'

const {
  tasks,
  displayTaskModal,
  taskCreationError,
  loadTasks,
  createTask,
  deleteTask,
  toggleTaskModal,
  toggleConfirmationModal,
  displayConfirmationModal,
} = useTasks()

const { selectedIndex, resetSelectedIndex } = useRowSelect(
  () => tasks.value.length,
  () => displayTaskModal.value,
)

const rowRefs = ref<HTMLElement[]>([])
const tableRef = ref<HTMLElement | null>(null)

const currentTask = ref({}) as Ref<Task>

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
      behavior: 'smooth',
    })
  } else if (rowRect.bottom > containerRect.bottom) {
    // Row is below visible area
    container.scrollBy({
      top: rowRect.bottom - containerRect.bottom + 10,
      behavior: 'smooth',
    })
  }
})

const openEditModal = (taskIndex: number | null) => {
  if (taskIndex !== null) {
    resetTask(currentTask)
    currentTask.value = {
      definition_id: tasks.value[taskIndex].definition_id,
      name: tasks.value[taskIndex].name,
      desc: tasks.value[taskIndex].desc,
      timestamp: toDatetimeLocalValue(tasks.value[taskIndex].timestamp) as DateTimeString,
    }
    toggleTaskModal()
  }
}

const openDeleteModal = (taskIndex: number | null) => {
  if (taskIndex !== null) {
    resetTask(currentTask)
    currentTask.value = {
      definition_id: tasks.value[taskIndex].definition_id,
      name: tasks.value[taskIndex].name,
      desc: tasks.value[taskIndex].desc,
      timestamp: toDatetimeLocalValue(tasks.value[taskIndex].timestamp) as DateTimeString,
    }

    console.log(currentTask)
    toggleConfirmationModal()
  }
}

const closeModals = () => {
  if (!displayConfirmationModal.value && !displayTaskModal.value) {
    resetSelectedIndex()
  }
  if (displayTaskModal.value) {
    toggleTaskModal()
    resetTask(currentTask)
  }
  if (displayConfirmationModal.value) {
    toggleConfirmationModal()
  }
}

const handleSave = async (task: Task) => {
  console.log('Saving task: ', task)
  await createTask(task)
  resetTask(currentTask)
}

const handleTaskDelete = async (id: string | undefined) => {
  if (!id) {
    return
  }

  try {
    await deleteTask(id)
  } catch (e) {
    console.log(`Failed to delete task: ${e}`)
  }

  resetTask(currentTask)
  toggleConfirmationModal()
}

onMounted(async () => {
  await loadTasks()

  // TODO: Move away
  const handler = async (e: KeyboardEvent) => {
    switch (e.key) {
      case 'n':
        if (!displayTaskModal.value && !displayConfirmationModal.value) {
          resetTask(currentTask)
          toggleTaskModal()
          e.stopPropagation()
          e.preventDefault()
        }
        if (displayConfirmationModal.value && selectedIndex.value !== null) {
          e.stopPropagation()
          e.preventDefault()
          // resetTask(currentTask);
          toggleConfirmationModal()
        }
        break

      case 'Escape':
        e.stopPropagation()
        e.preventDefault()
        closeModals()
        break

      case 'Enter':
        if (!displayTaskModal.value && !displayConfirmationModal.value) {
          e.stopPropagation()
          e.preventDefault()
          openEditModal(selectedIndex.value)
        }
        break

      case 'Backspace':
        if (!displayTaskModal.value && !displayConfirmationModal.value) {
          e.stopPropagation()
          e.preventDefault()
          openDeleteModal(selectedIndex.value)
        }
        break

      case 'y':
        if (displayConfirmationModal.value && selectedIndex.value !== null) {
          e.stopPropagation()
          e.preventDefault()
          try {
            await deleteTask(tasks.value[selectedIndex.value].definition_id as string)
          } catch (e) {
            console.log(`Failed to delete task: ${e}`)
          }
          resetTask(currentTask)
          toggleConfirmationModal()
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

// const l = await listen<Payload>('state-changed', (event) => {
//   console.log(`Received event: ${event}`)
// })

listen<Payload>('state-changed', (event) => {
  console.log(`Received event: ${event}`)
})

// onUnmounted(() => {
//   l()
// })
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
