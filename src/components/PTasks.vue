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
        <div class="overflow-y-scroll max-h-[300px] w-full my-2">
            <table v-if="tasks.length > 0">
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Description</th>
                        <th>Timestamp</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="task in tasks">
                        <td>{{ task.name }}</td>
                        <td>{{ task.desc ? task.desc : '-' }}</td>
                        <td>{{ task.timestamp }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </main>
    <PHotkeys screen-code="MAIN" />
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import TaskForm from './TaskForm.vue'
import PHotkeys from './PHotkeys.vue'
import { useTasks } from "../composables/useTasks";

// const name = ref();
// const desc = ref();
// const timestamp = ref();

const {
    tasks,
    displayNewTaskModal,
    taskCreationError,
    loadTasks,
    addTask,
    toggleCreateModal
} = useTasks();

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