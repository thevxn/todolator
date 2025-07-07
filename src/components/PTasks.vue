<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed "
        v-if="displayTaskModal || displayConfirmationModal" @click="toggleTaskModal"></div>

    <!-- TODO: The submit and close events in ConfirmationModal will be used for clicking on yes/no buttons (instead of using hotkeys) -->
    <main class="flex flex-col items-center justify-center p-4">
        <ConfirmationModal :display="displayConfirmationModal" @submit="console.log(' submitted!!!')"
            @close="console.log('closed!!!')" :name="currentTask.name" />
        <TaskForm submit-text="Save" :display="displayTaskModal" @submit="saveTask" @close="toggleTaskModal"
            :error-text="taskCreationError" :current-task="currentTask" />
        <div class="flex flex-row w-full items-center mt-10"
            :class="tasks.length > 0 ? 'justify-end' : 'justify-center'">
            <button tabindex="-1" @click="toggleTaskModal">New Task</button>
        </div>

        <div v-if="tasks.length > 0"
            class="w-full mt-2 mb-10 rounded-lg overflow-hidden border border-gray-500 min-h-[200px]">
            <div class="flex flex-col h-full">
                <!-- Header always visible -->
                <div class="grid grid-cols-3 bg-secondary font-bold text-primary flex-shrink-0">
                    <div class="p-2">Title</div>
                    <div class="p-2">Description</div>
                    <div class="p-2">Remind At</div>
                </div>

                <!-- Body scrolls -->
                <div class="flex-1 min-h-0 overflow-y-auto" ref="tableRef">
                    <div v-for="(task, i) in tasks" :key="i" :class="[
                        'grid grid-cols-3 border-t border-gray-500',
                        selectedIndex === i ? 'bg-secondary text-primary' : ''
                    ]" :ref="el => { if (el) rowRefs[i] = el as HTMLElement }">
                        <div class="p-2">{{ task.name }}</div>
                        <div class="p-2">{{ task.desc || '-' }}</div>
                        <div class="p-2">{{ new Date(task.timestamp).toLocaleString() }}</div>
                    </div>
                </div>
            </div>
        </div>

    </main>
    <PHotkeys screen-code="MAIN" />
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, Ref, ref, watch } from "vue";
import TaskForm from './TaskForm.vue'
import PHotkeys from './PHotkeys.vue'
import { ITask, useTasks } from "../composables/useTasks";
import { useRowSelect } from "../composables/useRowSelect";
import { toDatetimeLocalValue } from "../helpers/datetime";
import { resetTask } from "../helpers/task";
import ConfirmationModal from './ConfirmationModal.vue'

const {
    tasks,
    displayTaskModal,
    taskCreationError,
    loadTasks,
    saveTask,
    deleteTask,
    toggleTaskModal,
    toggleConfirmationModal,
    displayConfirmationModal
} = useTasks();

const { selectedIndex, resetSelectedIndex } = useRowSelect(() => tasks.value.length, () => displayTaskModal.value);

const rowRefs = ref<HTMLElement[]>([]);
const tableRef = ref<HTMLElement | null>(null);

const currentTask = ref({}) as Ref<ITask>;

watch(selectedIndex, (newIndex) => {
    if (newIndex === null || !tableRef.value) return;

    const container = tableRef.value;
    const rowElement = rowRefs.value[newIndex];

    if (!rowElement) {
        return;
    }

    const containerRect = container.getBoundingClientRect();
    const rowRect = rowElement.getBoundingClientRect();

    if (rowRect.top < containerRect.top) {
        // Row is above visible area
        container.scrollBy({
            top: rowRect.top - containerRect.top - 10,
            behavior: 'smooth'
        });
    } else if (rowRect.bottom > containerRect.bottom) {
        // Row is below visible area
        container.scrollBy({
            top: rowRect.bottom - containerRect.bottom + 10,
            behavior: 'smooth'
        });
    }
});

onMounted(async () => {
    await loadTasks();

    const handler = async (e: KeyboardEvent) => {

        switch (e.key) {
            case "n":
                if (!displayTaskModal.value && !displayConfirmationModal.value) {
                    resetTask(currentTask);
                    toggleTaskModal();
                    e.stopPropagation();
                    e.preventDefault();
                }
                if (displayConfirmationModal.value && selectedIndex.value !== null) {
                    e.stopPropagation();
                    e.preventDefault();
                    // resetTask(currentTask);
                    toggleConfirmationModal();
                }
                break;

            case "Escape":
                e.stopPropagation();
                e.preventDefault();
                if (!displayConfirmationModal.value && !displayTaskModal.value) {
                    resetSelectedIndex();
                }
                if (displayTaskModal.value) {
                    toggleTaskModal();
                    resetTask(currentTask);
                }
                if (displayConfirmationModal.value) {
                    toggleConfirmationModal();
                }
                break;

            case "Enter":
                if (!displayTaskModal.value && !displayConfirmationModal.value && selectedIndex.value !== null) {
                    e.stopPropagation();
                    e.preventDefault();
                    resetTask(currentTask);
                    currentTask.value = {
                        id: tasks.value[selectedIndex.value].id,
                        name: tasks.value[selectedIndex.value].name,
                        desc: tasks.value[selectedIndex.value].desc,
                        timestamp: toDatetimeLocalValue(tasks.value[selectedIndex.value].timestamp),
                    }
                    toggleTaskModal();
                }
                break;

            case "Backspace":
                if (!displayTaskModal.value && !displayConfirmationModal.value && selectedIndex.value !== null) {
                    e.stopPropagation();
                    e.preventDefault();
                    resetTask(currentTask);
                    currentTask.value = {
                        id: tasks.value[selectedIndex.value].id,
                        name: tasks.value[selectedIndex.value].name,
                        desc: tasks.value[selectedIndex.value].desc,
                        timestamp: toDatetimeLocalValue(tasks.value[selectedIndex.value].timestamp),
                    }
                    toggleConfirmationModal();
                }
                break;

            case "y":
                if (displayConfirmationModal.value && selectedIndex.value !== null) {
                    e.stopPropagation();
                    e.preventDefault();
                    await deleteTask(tasks.value[selectedIndex.value].id as string);
                    resetTask(currentTask);
                    toggleConfirmationModal();
                }
                break;

            default:
                break;
        }
    };
    window.addEventListener("keydown", handler);
    onUnmounted(() => window.removeEventListener("keydown", handler));
});
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