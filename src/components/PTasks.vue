<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed " v-if="displayNewTaskModal"
        @click="toggleCreateModal"></div>

    <main class="flex flex-col items-center justify-center p-4">
        <TaskForm submit-text="Save" :display="displayNewTaskModal" @submit="addTask" @close="toggleCreateModal"
            :error-text="taskCreationError" />
        <!-- <h1 class="text-secondary">Todolator</h1> -->
        <div class="flex flex-row w-full items-center mt-10"
            :class="tasks.length > 0 ? 'justify-end' : 'justify-center'">
            <button tabindex="-1" @click="toggleCreateModal">New Task</button>
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
import { onMounted, onUnmounted, ref, watch } from "vue";
import TaskForm from './TaskForm.vue'
import PHotkeys from './PHotkeys.vue'
import { useTasks } from "../composables/useTasks";
import { useRowSelect } from "../composables/useRowSelect";

const {
    tasks,
    displayNewTaskModal,
    taskCreationError,
    loadTasks,
    addTask,
    toggleCreateModal
} = useTasks();

const { selectedIndex } = useRowSelect(() => tasks.value.length, () => displayNewTaskModal.value);
const rowRefs = ref<HTMLElement[]>([]);
const tableRef = ref<HTMLElement | null>(null);

watch(selectedIndex, (newIndex) => {
    const el = rowRefs.value[newIndex as number];
    if (el) {
        if (newIndex === 0 && tableRef.value) {
            console.log("here!!!");
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
            toggleCreateModal();
            e.stopPropagation();
            e.preventDefault();
        }

        if (e.key === "Escape") {
            e.stopPropagation();
            e.preventDefault();
            if (displayNewTaskModal.value) {
                toggleCreateModal();
                return
            }
            selectedIndex.value = null;
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
    background:  #55555583;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
    background: #555555d2;
}
</style>