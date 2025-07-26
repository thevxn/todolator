<template>
  <div
    class="absolute bottom-0 w-full flex flex-row items-center justify-center gap-4 text-primary font-semibold rounded-b-md invisible sm:visible"
    :class="color"
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
type Screen = 'MAIN' | 'NEW_TASK_MODAL' | 'CONFIRMATION_MODAL' | 'REMINDER_POPUP' | 'SETTINGS_MODAL'

interface IHotkey {
  shortcut: string
  description: string
  screens: Array<Screen>
}

const hotkeys = [
  {
    shortcut: 'N',
    description: 'New',
    screens: ['MAIN']
  },
  {
    shortcut: '↑↓',
    description: 'Select',
    screens: ['MAIN']
  },
  {
    shortcut: 'S',
    description: 'Settings',
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
    screens: ['NEW_TASK_MODAL', 'SETTINGS_MODAL']
  },
  {
    shortcut: '⏎',
    description: 'Save',
    screens: ['NEW_TASK_MODAL', 'SETTINGS_MODAL']
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

withDefaults(
  defineProps<{
    screenCode: Screen
    color?: string
  }>(),
  {
    color: 'bg-secondary'
  }
)
</script>

<style></style>
