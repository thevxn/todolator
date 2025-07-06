<template>
    <div
        class="absolute bottom-0 w-full flex flex-row items-center justify-center gap-4  bg-secondary text-primary font-semibold rounded-b-md invisible sm:visible">
        <div v-for="hotkey in hotkeys">
            <div v-if="hotkey.screens.some((el) => el === screenCode)">
                <span class="font-semibold">[{{ hotkey.shortcut }}]: </span><span>{{
                    hotkey.description }}</span>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
type Screen = "MAIN" | "NEW_TASK_MODAL"

interface IHotkey {
    shortcut: string;
    description: string;
    screens: Array<Screen>
}

const hotkeys: Array<IHotkey> = [
    {
        shortcut: "N",
        description: "New Task",
        screens: ["MAIN"]
    },
    {
        shortcut: "↑↓",
        description: "Select Task",
        screens: ["MAIN"]
    },
    {
        shortcut: "ESC",
        description: "Close",
        screens: ["NEW_TASK_MODAL"]
    },
    {
        shortcut: "ENTER",
        description: "Save",
        screens: ["NEW_TASK_MODAL"]
    },
    {
        shortcut: "TAB",
        description: "Next input",
        screens: ["NEW_TASK_MODAL"]
    }
]

defineProps<{
    screenCode: Screen
}>()



</script>

<style></style>