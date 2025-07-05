<template>
    <main class="container">
        <h1>Todolator</h1>

        <h2>Tasks:</h2>
        <p>{{ tasks }}</p>

        <form class="row" @submit.prevent="addTask">
            <input id="greet-input" v-model="name" placeholder="Title" type="text" />
            <input id="greet-input" v-model="desc" placeholder="Description" type="text" />
            <input id="greet-input" v-model="timestamp" placeholder="Timestamp" type="datetime-local" />
            <button type="submit">Add task</button>
        </form>

    </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const tasks = ref();
const name = ref();
const desc = ref();
const timestamp = ref();

try {
    tasks.value = await invoke("get_tasks");
    console.log("tasks loaded");
} catch (e) {
    console.log(e);
}

console.log(tasks.value);

async function addTask() {
    try {
        const task = { name: name.value, desc: desc.value, timestamp: new Date(timestamp.value).toISOString() };

        console.log(`name: ${name.value}, description: ${desc.value}, timestamp: ${timestamp.value}`)

        await invoke("create_task", task);
        console.log("task created!");

        // const newTasks = await invoke("get_tasks");
        // console.log(`new tasks:`);
        // console.log(newTasks);

        tasks.value.push(task)
    }
    catch (e) {
        console.log(e);
    }
}

</script>

<style scoped></style>