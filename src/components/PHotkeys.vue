<template>
    <div
        class="absolute bottom-0 w-full flex flex-row items-center justify-center gap-4 mb-2 bg-secondary text-primary font-semibold">
        <div v-for="hotkey in hotkeys"><span class="font-semibold">[{{ hotkey.shortcut }}]: </span><span>{{
            hotkey.description }}</span></div>
    </div>
</template>

<script lang="ts" setup>
const hotkeys = [
    {
        shortcut: "N",
        description: "New Task"
    },
    {
        shortcut: "Esc",
        description: "Close Modal"
    },
    {
        shortcut: "Shift + Enter",
        description: "Save Task"
    },
]

</script>

<style></style>