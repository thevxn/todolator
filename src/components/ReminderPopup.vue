<template>
    <div class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2  bg-primary rounded-md min-w-1/2"
        ref="modal">
        <div class="p-10">
            <h2>{{ task?.name }}</h2>
            <p>
                <span class="text-lg">{{ task?.desc }}</span>
            </p>
            <button @click="$emit('close')">Done</button>
        </div>
        <PHotkeys screen-code="REMINDER_POPUP" />
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { ITask } from '../composables/useTasks';
import { invoke } from '@tauri-apps/api/core';
import PHotkeys from './PHotkeys.vue';

const task = ref<ITask>()

try {
    task.value = await invoke("get_next_task");
} catch (e) {
    console.log("Failed to load task: ", e)
    console.log(e)
}

defineEmits(["close"]);

</script>

<style></style>