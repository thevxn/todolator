<template>
    <div v-if="display"
        class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2  bg-primary rounded-md min-w-1/2"
        ref="modal">
        <div class=" p-10">
            <h1 class="text-center">Delete Task {{ props.name }}?</h1>
        </div>
        <PHotkeys screen-code="CONFIRMATION_MODAL" />
    </div>
</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';
import PHotkeys from './PHotkeys.vue'


const props = defineProps<{
    display: boolean
    name?: string

    // TODO: error messages
    errorText?: string
}>()

const modal = ref<HTMLInputElement | null>(null);
watch(() => props.display, (display) => {
    if (display) {
        nextTick(() => {
            // Focus the first input
            modal.value?.focus();
        });
    }
});

const emit = defineEmits(["submit", "close"]);
</script>

<style></style>