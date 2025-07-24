<template>
  <button class="icon-button" @click="$emit('clicked')">
    <Icon :icon="icon" :width :height />
  </button>
</template>

<script lang="ts" setup>
import { Icon } from '@iconify/vue'

withDefaults(
  defineProps<{
    icon: string
    height?: string
    width?: string
  }>(),
  {
    width: '32px',
    height: '32px'
  }
)

defineEmits(['clicked'])
</script>

<style></style>
