<template>
    <!-- Modal overlay -->
    <div class="min-w-screen w-screen min-h-screen opacity-50 bg-black z-1 fixed " v-if="displayNewTaskModal"></div>

    <main class="flex flex-col items-center justify-center p-4">
        <TaskForm submit-text="Save" :display="displayNewTaskModal"
            @submit="displayNewTaskModal = !displayNewTaskModal" />
        <h1 class="text-secondary">Todolator</h1>
        <div class="flex flex-row w-full items-center justify-end">
            <button @click="displayNewTaskModal = !displayNewTaskModal">New Task</button>
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
import { nextTick, onMounted, onUnmounted, Ref, ref, watch } from "vue";
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

// async function addTask() {
//     try {
//         const task = { name: name.value, desc: desc.value, timestamp: new Date(timestamp.value).toISOString() };

//         console.log(`name: ${name.value}, description: ${desc.value}, timestamp: ${timestamp.value}`)

//         await invoke("create_task", task);
//         console.log("task created!");

//         // const newTasks = await invoke("get_tasks");
//         // console.log(`new tasks:`);
//         // console.log(newTasks);

//         tasks.value.push(task)
//     }
//     catch (e) {
//         console.log(e);
//     }
// }

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