<template>
    <div v-if="display"
        class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2  bg-primary p-10 rounded-md"
        @keydown.esc="$emit('close')">

        <h1 class="text-center">New Task</h1>

        <div class="row flex flex-col justify-center items-center gap-2 mt-8"
            @keydown.ctrl.enter="$emit('submit', { name, desc, timestamp })">
            <input v-model="name" placeholder="Title" type="text" class="w-full" tabindex="1" ref="focusInput" />
            <input v-model="desc" placeholder="Description" type="text" class="w-full" tabindex="2" />
            <input v-model="timestamp" placeholder="Timestamp" type="datetime-local" class="w-full" tabindex="3" />
            <button class="mt-8 mb-0" @click="$emit('submit', { name, desc, timestamp })" tabindex="4">{{ submitText
            }}</button>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';

const props = defineProps({
    submitText: {
        type: String,
        default: null
    },
    name: {
        type: String,
        default: null
    },
    desc: {
        type: String,
        default: null
    },
    timestamp: {
        type: String,
        default: null
    },
    display: {
        type: Boolean,
        default: false
    }
})

const submitText = ref(props.submitText);
const name = ref(props.name);
const desc = ref(props.desc);
const timestamp = ref(props.timestamp);

defineEmits(["submit", "close"]);

const focusInput = ref<HTMLInputElement | null>(null);
watch(() => props.display, (display) => {
    if (display) {
        // Clear inputs
        name.value = ""
        desc.value = ""
        timestamp.value = ""

        nextTick(() => {
            // Focus the first input
            focusInput.value?.focus();
        });
    }
});

</script>

<style></style>