<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2"
    ref="modal"
  >
    <div class="p-10">
      <h1 class="text-center text-error mb-10">Delete task {{ props.name }}?</h1>
      <div class="flex flex-row items-center justify-center gap-x-4">
        <button
          class="bg-primary border-error text-error hover:text-primary hover:bg-error active:text-error active:bg-primary"
          @click="$emit('submit')"
        >
          Yes
        </button>
        <button
          class="bg-primary border-secondary text-secondary hover:text-primary hover:bg-secondary active:text-secondary active:bg-primary"
          @click="$emit('close')"
        >
          No
        </button>
      </div>
    </div>
    <PHotkeys screen-code="CONFIRMATION_MODAL" />
  </div>
</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue'
import PHotkeys from './PHotkeys.vue'

const props = defineProps<{
  display: boolean
  name?: string
  id?: string
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
  },
)

const emit = defineEmits(['submit', 'close'])
</script>

<style></style>
