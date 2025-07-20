<template>
  <div class="flex flex-col w-full h-full items-center justify-center">
    <h2 class="text-center">
      [{{ timestampDate().getHours().toString().padStart(2, '0') }}:{{
        timestampDate().getMinutes().toString().padStart(2, '0')
      }}]
      {{ task?.name }}
    </h2>
    <p class="mb-2" v-if="task?.desc">
      <span class="text-lg">{{ task?.desc }}</span>
    </p>
    <button @click="$emit('close', task)">Done</button>
    <PHotkeys screen-code="REMINDER_POPUP" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { TaskInstance } from '../composables/useTasks'
import { invoke } from '@tauri-apps/api/core'
import PHotkeys from './PHotkeys.vue'

const task = ref<TaskInstance>()
const timestampDate = () => new Date(task.value?.timestamp as string)

try {
  task.value = await invoke('get_next_task')
} catch (e) {
  console.log('Failed to load task: ', e)
  console.log(e)
}

onMounted(async () => {
  const handler = async (e: KeyboardEvent) => {
    if (e.ctrlKey && e.key === 'Enter') {
      emit('close')
    }
  }
  window.addEventListener('keydown', handler)
  onUnmounted(() => window.removeEventListener('keydown', handler))
})

const emit = defineEmits(['close'])
</script>

<style></style>
