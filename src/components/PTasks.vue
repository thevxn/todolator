<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed " v-if="displayNewTaskModal"
        @click="toggleCreateModal"></div>

    <main class="flex flex-col items-center justify-center p-4">
        <TaskForm submit-text="Save" :display="displayNewTaskModal" @submit="addTask" @close="toggleCreateModal" />
        <h1 class="text-secondary">Todolator</h1>
        <div class="flex flex-row w-full items-center justify-end">
            <button tabindex="-1" @click="toggleCreateModal">New Task</button>
        </div>
        <table>
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
    </main>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TaskForm from './TaskForm.vue'

const tasks = ref() as Ref<Array<ITask>>;
// const name = ref();
// const desc = ref();
// const timestamp = ref();

const displayNewTaskModal = ref(false);

try {
    tasks.value = await invoke("get_tasks");
    console.log("tasks loaded");
} catch (e) {
    console.log(e);
}

console.log(tasks.value);

type DateTimeString = string;

interface ITask {
    name: string;
    desc?: string;
    timestamp: DateTimeString;
}

async function addTask(e: ITask) {
    if (!e.name || !e.timestamp) {
        console.log("missing required attributes");
        return;
    }
    try {
        console.log("Adding task:")
        console.log(e);
        const task = { name: e.name, desc: e.desc, timestamp: new Date(e.timestamp).toISOString() };

        await invoke("create_task", task);
        console.log("task created!");

        tasks.value.push(task)

        toggleCreateModal();
    }
    catch (e) {
        console.log(e);
    }
}

function toggleCreateModal() {
    displayNewTaskModal.value = !displayNewTaskModal.value
}

onMounted(() => {
    const handler = (e: KeyboardEvent) => {
        if (e.key === "n") {
            displayNewTaskModal.value = !displayNewTaskModal.value;
            e.stopPropagation();
            e.preventDefault();
        }
    };
    window.addEventListener("keydown", handler);
    onUnmounted(() => window.removeEventListener("keydown", handler));
});

</script>

<style></style>