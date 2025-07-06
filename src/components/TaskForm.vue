<template>
    <div v-if="display"
        class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2  bg-primary  rounded-md min-w-1/2"
        @keydown.esc="$emit('close')">
        <div class="p-10">
            <h1 class="text-center">New Task</h1>

            <div class="row flex flex-col justify-center items-center gap-2 mt-8"
                @keydown.enter="$emit('submit', { name, desc, timestamp })">
                <label for="name" class="text-left w-full font-bold">Title<span class="text-error">*</span></label>
                <input v-model="name" placeholder="Title" type="text" class="w-full" tabindex="1" ref="focusInput"
                    name="title" required />

                <label for="desc" class="text-left w-full font-bold">Description</label>
                <textarea v-model="desc" placeholder="Description" name="desc" class="w-full" tabindex="2"></textarea>

                <label for="timestamp" class="text-left w-full font-bold">Remind At<span
                        class="text-error">*</span></label>
                <input v-model="timestamp" placeholder="Timestamp" name="timestamp" type="datetime-local"
                    class="w-full " tabindex="3" required />
                <small class="text-left w-full mt-4"><span class="text-error ">*</span> = Required attribute</small>
                <button class="mb-4" @click="$emit('submit', { name, desc, timestamp })" tabindex="4">{{ submitText
                }}</button>
                <span class="text-error text-lg" v-if="errorText">{{ errorText }}</span>
                <!-- TODO: Add hotkey text on the bottom of the modal as well as the main window. Only show relevant hotkeys for each component. -->
            </div>
        </div>
        <PHotkeys screen-code="NEW_TASK_MODAL" />

    </div>

</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';
import PHotkeys from './PHotkeys.vue'

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
    },
    errorText: {
        type: String,
        default: null
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