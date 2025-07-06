<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed " v-if="displayNewTaskModal"
        @click="toggleCreateModal"></div>

    <main class="flex flex-col items-center justify-center p-4">
        <TaskForm submit-text="Save" :display="displayNewTaskModal" @submit="addTask" @close="toggleCreateModal"
            :error-text="taskCreationError" />
        <h1 class="text-secondary">Todolator</h1>
        <div class="flex flex-row w-full items-center" :class="tasks.length > 0 ? 'justify-end' : 'justify-center'">
            <button tabindex="-1" @click="toggleCreateModal">New Task</button>
        </div>
        <div class="overflow-y-scroll max-h-full w-full mt-2 mb-10" ref="tableRef">
            <table v-if="tasks.length > 0">
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Description</th>
                        <th>Remind At</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(task, i) in tasks" :key="i" :class="{ 'bg-black': selectedIndex === i }"
                        :ref="el => (rowRefs[i] = el as HTMLElement)">

                        <td>{{ task.name }}</td>
                        <td>{{ task.desc ? task.desc : '-' }}</td>
                        <td>{{ new Date(task.timestamp).toLocaleString() }}</td>
                    </tr>
                </tbody>
            </table>
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

const { selectedIndex } = useRowSelect(() => tasks.value.length);
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
    background: #f3f4f6;
}

/* Handle */
::-webkit-scrollbar-thumb {
    background: #58bc82;
}

/* Handle on hover */
::-webkit-scrollbar-thumb:hover {
    background: #555;
}
</style>