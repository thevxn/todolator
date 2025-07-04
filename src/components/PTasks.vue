<template>
    <main class="container">
        <h1>Todolator</h1>

        <h2>Tasks:</h2>
        <p>{{ tasks }}</p>

        <form class="row" @submit.prevent="greet">
            <input id="greet-input" v-model="name" placeholder="Enter a name..." />
            <button type="submit">Greet</button>
        </form>
        <p>{{ greetMsg }}</p>
    </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const tasks = ref([]);

try {
    tasks.value = await invoke("get_tasks");
} catch (e) {
    console.log(e);
}

console.log(tasks.value);

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<style scoped></style>