<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2 focus:outline-none"
    ref="modal"
    tabindex="0"
    @keydown.y.stop="$emit('submit')"
    @keydown.n.stop="$emit('close')"
  >
    <h2>
      <PIcon :icon="'mingcute:settings-6-line'" class="px-2" :height="'30px'" :width="'30px'" />
    </h2>
    <div class="p-2">
      <div class="flex flex-row items-center justify-center gap-x-4">
        {{ settings }}
      </div>
    </div>
    <PHotkeys screen-code="SETTINGS_MODAL" />
  </div>
</template>

<script lang="ts" setup>
import { nextTick, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

import PHotkeys from './PHotkeys.vue'
import PIcon from './PIcon.vue'

const props = defineProps<{
  display: boolean
  // TODO: error messages
  errorText?: string
}>()

const modal = ref<HTMLInputElement | null>(null)
watch(
  () => props.display,
  (display) => {
    if (display) {
      nextTick(() => {
        // Focus the first input
        modal.value?.focus()
      })
    }
  }
)

const settings = ref()

onMounted(async () => {
  try {
    settings.value = await invoke('get_settings')
  } catch (e) {
    console.log('Failed to fetch settings: ', e)
  }
})

defineEmits(['submit', 'close'])
</script>

<style></style>
