<template>
    <div
        class="absolute bottom-0 w-full flex flex-row items-center justify-center gap-4 mb-2 bg-secondary text-primary font-semibold">
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
        shortcut: "Esc",
        description: "Close Modal",
        screens: ["NEW_TASK_MODAL"]
    },
    {
        shortcut: "Enter",
        description: "Save Task",
        screens: ["NEW_TASK_MODAL"]
    }
]

defineProps<{
    screenCode: Screen
}>()



</script>

<style></style>