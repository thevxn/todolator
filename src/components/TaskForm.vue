<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2"
  >
    <div class="p-10">
      <h1 class="text-center" v-if="mode === Mode.CREATE">New Task</h1>
      <h1 class="text-center text-warning" v-else>Edit Task</h1>

      <div
        class="row flex flex-col justify-center items-center gap-2 mt-8"
        @keydown.enter="
          mode === Mode.CREATE ? $emit('create', localTask) : $emit('update', localTask)
        "
      >
        <label for="name" class="text-left w-full font-bold"
          >Title<span class="text-error">*</span></label
        >
        <input
          v-model="localTask.name"
          placeholder="Title"
          type="text"
          class="w-full"
          tabindex="1"
          ref="focusInput"
          name="title"
          required
        />

        <label for="desc" class="text-left w-full font-bold">Description</label>
        <textarea
          v-model="localTask.desc"
          placeholder="Description"
          name="desc"
          class="w-full"
          tabindex="2"
        ></textarea>

        <label for="timestamp" class="text-left w-full font-bold"
          >Remind At<span class="text-error">*</span></label
        >

        <input
          v-model="localTask.start"
          name="timestamp"
          type="datetime-local"
          class="w-full"
          tabindex="3"
          required
        />
        <!-- <input
          v-model="localTask.timestamp"
          name="timestamp"
          type="datetime-local"
          class="w-full"
          tabindex="3"
          required
          v-if="isExistingDefinition(localTask)"
        /> -->

        <small class="text-left w-full mt-4"
          ><span class="text-error">*</span> = Required attribute</small
        >
        <button
          class="mb-4"
          @click="mode === Mode.CREATE ? $emit('create', localTask) : $emit('update', localTask)"
          tabindex="4"
        >
          {{ submitText }}
        </button>
        <span class="text-error text-lg" v-if="errorText">{{ errorText }}</span>
      </div>
    </div>
    <PHotkeys screen-code="NEW_TASK_MODAL" />
  </div>
</template>

<script lang="ts" setup>
import { computed, nextTick, ref, watch } from 'vue'
import PHotkeys from './PHotkeys.vue'
import { TaskDefinition } from '../composables/useTasks'
import { getDefaultTaskDefinition, isExistingDefinition } from '../helpers/task'

enum Mode {
  CREATE,
  UPDATE,
}

const props = withDefaults(
  defineProps<{
    // Included if updating an existing definition
    // Not included if creating a new one (undefined passed from the parent)
    currentTask?: TaskDefinition

    display: boolean
    submitText?: string
    errorText?: string
  }>(),
  {
    currentTask: getDefaultTaskDefinition,
    display: false,
  },
)

const localTask = ref(props.currentTask)

const mode = computed(() => {
  return isExistingDefinition(localTask.value) ? Mode.UPDATE : Mode.CREATE
})
watch(
  () => props.currentTask,
  (newVal) => {
    console.log('Task changed to ', newVal)
    localTask.value = newVal
  },
)

// TODO: Is this actually needed?
watch(
  () => props.display,
  (shown) => {
    if (!shown) {
      localTask.value = getDefaultTaskDefinition()
    }
  },
)

const focusInput = ref<HTMLInputElement | null>(null)
watch(
  () => props.display,
  (display) => {
    if (display) {
      nextTick(() => {
        // Focus the first input
        focusInput.value?.focus()
      })
    }
  },
  { immediate: true },
)

defineEmits(['create', 'update', 'close'])
</script>

<style></style>
