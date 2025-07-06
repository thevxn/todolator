<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed " v-if="displayNewTaskModal"
        @click="toggleTaskModal"></div>

    <main class="flex flex-col items-center justify-center p-4">
        <TaskForm submit-text="Save" :display="displayNewTaskModal" @submit="saveTask" @close="toggleTaskModal"
            :error-text="taskCreationError" :current-task="currentTask" />
        <div class="flex flex-row w-full items-center mt-10"
            :class="tasks.length > 0 ? 'justify-end' : 'justify-center'">
            <button tabindex="-1" @click="toggleTaskModal">New Task</button>
        </div>

        <div v-if="tasks.length > 0" class="w-full mt-2 mb-10">
            <div class="pr-[6px] bg-secondary border border-gray-500 ">

                <table class="w-full table-fixed">
                    <thead>
                        <tr class="border-none">
                            <th class="text-left border-gray-500 border border-t-0 border-b-0 border-l-0">Title</th>
                            <th class="text-left border-gray-500 border border-t-0 border-b-0 border-l-0">Description
                            </th>
                            <th class="text-left border-r-secondary">Remind At</th>
                        </tr>
                    </thead>
                </table>
            </div>

            <div ref="tableRef" class="overflow-y-auto max-h-[400px]">
                <table class="w-full table-fixed">
                    <tbody>
                        <tr v-for="(task, i) in tasks" :key="i"
                            :class="selectedIndex === i ? 'bg-secondary text-primary' : ''"
                            :ref="el => (rowRefs[i] = el as HTMLElement)">
                            <td>{{ task.name }}</td>
                            <td>{{ task.desc || '-' }}</td>
                            <td>{{ new Date(task.timestamp).toLocaleString() }}</td>
                        </tr>
                    </tbody>
                </table>
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

const {
    tasks,
    displayNewTaskModal,
    taskCreationError,
    loadTasks,
    saveTask,
    toggleTaskModal
} = useTasks();

const { selectedIndex, resetSelectedIndex } = useRowSelect(() => tasks.value.length, () => displayNewTaskModal.value);

const rowRefs = ref<HTMLElement[]>([]);
const tableRef = ref<HTMLElement | null>(null);

const currentTask = ref() as Ref<ITask>;

watch(selectedIndex, (newIndex) => {
    const el = rowRefs.value[newIndex as number];
    if (el) {
        if (newIndex === 0 && tableRef.value) {
            tableRef.value.scrollTo({ behavior: "smooth", top: 0 });
            return
        }
        el.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }
});

onMounted(async () => {
    await loadTasks();

    const handler = (e: KeyboardEvent) => {
        if (e.key === "n" && !displayNewTaskModal.value) {
            resetTask(currentTask);
            toggleTaskModal();
            e.stopPropagation();
            e.preventDefault();
        }

        if (e.key === "Escape") {
            e.stopPropagation();
            e.preventDefault();
            if (displayNewTaskModal.value) {
                toggleTaskModal();
                resetTask(currentTask);
                return
            }
            resetSelectedIndex();
        }

        if (e.key === "Enter") {
            if (!displayNewTaskModal.value && selectedIndex.value !== null) {
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