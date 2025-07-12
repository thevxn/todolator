<template>
  <button class="icon-button hover:cursor-pointer" @click="$emit('clicked')">
    <Icon :icon="icon" class="w-[32px] h-[32px]" />
  </button>
</template>

<script lang="ts" setup>
import { Icon } from '@iconify/vue'

defineProps<{
  icon: string
}>()

defineEmits(['clicked'])
</script>

<style></style>
