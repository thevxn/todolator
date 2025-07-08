<template>
    <div v-if="display"
        class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2  bg-primary rounded-md min-w-1/2">
        <div class="p-10">
            <h1 class="text-center" v-if="!currentTask.id">New Task</h1>
            <h1 class="text-center text-warning" v-else>Edit Task</h1>

            <div class="row flex flex-col justify-center items-center gap-2 mt-8"
                @keydown.enter="$emit('submit', currentTask)">
                <label for="name" class="text-left w-full font-bold">Title<span class="text-error">*</span></label>
                <input v-model="currentTask.name" placeholder="Title" type="text" class="w-full" tabindex="1"
                    ref="focusInput" name="title" required />

                <label for="desc" class="text-left w-full font-bold">Description</label>
                <textarea v-model="currentTask.desc" placeholder="Description" name="desc" class="w-full"
                    tabindex="2"></textarea>

                <label for="timestamp" class="text-left w-full font-bold">Remind At<span
                        class="text-error">*</span></label>
                <input v-model="currentTask.timestamp" placeholder="Timestamp" name="timestamp" type="datetime-local"
                    class="w-full " tabindex="3" required />
                <small class="text-left w-full mt-4"><span class="text-error ">*</span> = Required attribute</small>
                <button class="mb-4" @click="$emit('submit', currentTask)" tabindex="4">{{ submitText
                    }}</button>
                <span class="text-error text-lg" v-if="errorText">{{ errorText }}</span>
            </div>
        </div>
        <PHotkeys screen-code="NEW_TASK_MODAL" />

    </div>

</template>

<script lang="ts" setup>
import { nextTick, ref, watch } from 'vue';
import PHotkeys from './PHotkeys.vue'
import { ITask } from '../composables/useTasks';


const props = defineProps<{
    currentTask: ITask
    display: boolean
    submitText?: string
    errorText?: string
}>()

const focusInput = ref<HTMLInputElement | null>(null);
watch(() => props.display, (display) => {
    if (display) {
        nextTick(() => {
            // Focus the first input
            focusInput.value?.focus();
        });
    }
});

const emit = defineEmits(["submit", "close"]);
</script>

<style></style>