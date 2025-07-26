<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2 focus:outline-none"
    ref="modal"
    tabindex="0"
  >
    <div class="mt-1">
      <PIcon
        :icon="'mingcute:settings-6-line'"
        class="px-2 text-secondary"
        :height="'30px'"
        :width="'30px'"
        tabindex="-1"
      />
    </div>

    <div class="flex flex-row items-center justify-center mt-10">
      <label for="autostart">Run on startup</label>
      <input
        id="autostart"
        name="autostart"
        type="checkbox"
        class="ml-2 w-5 h-5 rounded-md appearance-auto !mb-0 hover:cursor-pointer"
        :checked="settings?.autostart"
        v-model="settings!.autostart"
      />
    </div>
    <div
      class="flex flex-col items-center justify-center gap-x-1 pb-8 gap-y-2 text-lg min-h-[180px]"
    >
      <button @click="saveSettings" tabindex="1">Save</button>
    </div>

    <PHotkeys screen-code="SETTINGS_MODAL" />
  </div>
</template>

<script lang="ts" setup>
import { nextTick, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

import PHotkeys from './PHotkeys.vue'
import PIcon from './PIcon.vue'

interface Settings {
  autostart: boolean
}

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

const settings = ref<Settings>()

onMounted(async () => {
  try {
    settings.value = await invoke('get_settings')
  } catch (e) {
    console.log('Failed to fetch settings: ', e)
  }
})

const emit = defineEmits(['submit'])

const saveSettings = async () => {
  try {
    await invoke('update_settings', { settings: settings.value })
  } catch (e) {
    console.log('Failed to save settings: ', e)
  }

  emit('submit')
}
</script>

<style></style>
