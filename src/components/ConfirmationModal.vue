<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2 focus:outline-none"
    ref="modal"
    tabindex="0"
    @keydown.y.stop="$emit('submit')"
    @keydown.n.stop="$emit('close')"
  >
    <div class="mt-1">
      <PIcon
        :icon="'mingcute:delete-2-line'"
        class="px-2 text-error"
        :height="'30px'"
        
        
        :width="'30px'"
      />
    </div>
    <div class="px-2 pb-8">
      <span class="text-center text-font-primary mb-7 w-full inline-block text-lg mt-2">
        Delete task
        <b
          ><i>{{ props.name }}</i></b
        >?
      </span>
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
    <PHotkeys screen-code="CONFIRMATION_MODAL" :color="'bg-error'" />
  </div>
</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue'

import PHotkeys from './PHotkeys.vue'
import PIcon from './PIcon.vue'

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
  }
)

defineEmits(['submit', 'close'])
</script>

<style></style>
