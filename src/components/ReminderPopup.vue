<template>
  <div class="flex flex-col w-full h-full items-center justify-center">
    <h2 class="text-center mt-8 mb-0">
      {{ timestampDate().getHours().toString().padStart(2, '0') }}:{{
        timestampDate().getMinutes().toString().padStart(2, '0')
      }}
    </h2>
    <h2 class="text-font-primary mt-0">{{ task?.name }}</h2>
    <p class="overflow-auto px-4 max-h-25 mx-2" v-if="task?.desc">
      <span class="text-md text-font-primary">{{ task?.desc }}</span>
    </p>
    <button class="mb-4 mt-4" @click="$emit('close', task)">Dismiss</button>
    <PHotkeys screen-code="REMINDER_POPUP" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { TaskInstance } from '../composables/useTasks'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import PHotkeys from './PHotkeys.vue'
import { resolveResource } from '@tauri-apps/api/path'
// import { readFile } from '@tauri-apps/plugin-fs'

const audioPath = await resolveResource('resources/alarm.mp3')
// const audio = await readFile(audioPath)

const audioP = new Audio(convertFileSrc(audioPath))
await audioP.play()

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
      emit('close', task.value)
    }
  }
  window.addEventListener('keydown', handler)
  onUnmounted(() => window.removeEventListener('keydown', handler))
})

const emit = defineEmits(['close'])
</script>

<style></style>
