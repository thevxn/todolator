<template>
  <div class="flex flex-col w-full h-full items-center justify-center">
    <div>
      <h2 class="text-center">{{ task?.name }}</h2>
      <p>
        <span class="text-lg">{{ task?.desc }}</span>
      </p>
      <button @click="$emit('close', task)">Done</button>
    </div>
    <PHotkeys screen-code="REMINDER_POPUP" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Task } from '../composables/useTasks'
import { invoke } from '@tauri-apps/api/core'
import PHotkeys from './PHotkeys.vue'

const task = ref<Task>()

try {
  task.value = await invoke('get_next_task')
} catch (e) {
  console.log('Failed to load task: ', e)
  console.log(e)
}

defineEmits(['close'])
</script>

<style></style>
