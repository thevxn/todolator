<template>
  <div
    class="absolute bottom-0 w-full flex flex-row items-center justify-center gap-4 bg-secondary text-primary font-semibold rounded-b-md invisible sm:visible"
  >
    <div
      v-for="hotkey in hotkeys.filter((h) => h.screens.includes(screenCode))"
      :key="hotkey.shortcut"
    >
      <span class="font-semibold">[{{ hotkey.shortcut }}]: </span
      ><span>{{ hotkey.description }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
type Screen = 'MAIN' | 'NEW_TASK_MODAL' | 'CONFIRMATION_MODAL' | 'REMINDER_POPUP'

interface IHotkey {
  shortcut: string
  description: string
  screens: Array<Screen>
}

const hotkeys = [
  {
    shortcut: 'N',
    description: 'New Task',
    screens: ['MAIN']
  },
  {
    shortcut: '↑↓',
    description: 'Select Task',
    screens: ['MAIN']
  },
  {
    shortcut: '⏎',
    description: 'Open Selected',
    screens: ['MAIN']
  },
  {
    shortcut: '⌫',
    description: 'Delete Selected',
    screens: ['MAIN']
  },
  {
    shortcut: 'ESC',
    description: 'Close',
    screens: ['NEW_TASK_MODAL']
  },
  {
    shortcut: '⏎',
    description: 'Save',
    screens: ['NEW_TASK_MODAL']
  },
  {
    shortcut: 'CTRL + ⏎',
    description: 'Dismiss Reminder',
    screens: ['REMINDER_POPUP']
  },
  {
    shortcut: '⇥',
    description: 'Next input',
    screens: ['NEW_TASK_MODAL']
  },
  {
    shortcut: 'Y',
    description: 'Yes',
    screens: ['CONFIRMATION_MODAL']
  },
  {
    shortcut: 'N',
    description: 'No',
    screens: ['CONFIRMATION_MODAL']
  }
] as const as Readonly<Array<IHotkey>>

defineProps<{
  screenCode: Screen
}>()
</script>

<style></style>
