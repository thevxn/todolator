<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2 overflow-y-auto"
  >
    <div class="p-5">
      <!-- <h1 class="text-center" v-if="mode === Mode.CREATE">New Task</h1>
      <h1 class="text-center text-warning" v-else>Edit Task</h1> -->

      <div
        class="row flex flex-col justify-center items-center gap-2"
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
          id="title"
          required
        />

        <label for="desc" class="text-left w-full font-bold">Description</label>
        <textarea
          v-model="localTask.desc"
          placeholder="Description"
          name="desc"
          id="desc"
          class="w-full"
          tabindex="2"
        ></textarea>

        <label for="timestamp" class="text-left w-full font-bold"
          >Remind At<span class="text-error">*</span></label
        >
        <input
          v-model="localTask.start"
          name="timestamp"
          id="timestamp"
          type="datetime-local"
          class="w-full"
          tabindex="3"
          required
        />

        <div class="flex flex-row w-full gap-x-2 items-center mt-2">
          <label for="recurring" class="text-left font-bold">Recurring?</label>
          <input
            type="checkbox"
            class="w-5 h-5 rounded-md appearance-auto"
            name="recurring"
            id="recurring"
            tabindex="4"
            :disabled="mode === Mode.UPDATE"
            :checked="isRecurring"
            @click="
              isRecurring
                ? (localTask.recurrence = null)
                : (localTask.recurrence = getDefaultRecurrence())
            "
          />
        </div>

        <div class="flex flex-row w-full" v-if="isRecurring && mode === Mode.UPDATE">
          <small
            >Repeat every
            <span class="text-secondary">{{ localTask.recurrence?.minutes }}</span> minutes -
            {{ recurrenceInterval }}</small
          >
        </div>

        <div class="flex flex-row w-full" v-if="isRecurring && mode === Mode.CREATE">
          <label for="recurrence-minutes" class="text-left">Repeat every</label>
          <input
            v-if="isRecurring"
            v-model="localTask.recurrence!.minutes"
            type="number"
            class="rounded-md h-6 w-12 mx-1"
            name="recurrence-minutes"
            id="recurrence-minutes"
            tabindex="5"
            min="1"
          />
          <span class="text-left">minutes</span>
        </div>

        <small class="text-left w-full mt-4"
          ><span class="text-error">*</span> = Required attribute</small
        >
        <button
          class="mb-4"
          @click="mode === Mode.CREATE ? $emit('create', localTask) : $emit('update', localTask)"
          tabindex="6"
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
import { CreatedTaskDefinition, TaskDefinition } from '../composables/useTasks'
import {
  getDefaultRecurrence,
  getDefaultTaskDefinition,
  isExistingDefinition,
  isRecurringDefinition,
} from '../helpers/task'
import { convertMinutesToHighestUnit } from '../helpers/datetime'

enum Mode {
  CREATE,
  UPDATE,
}

const props = withDefaults(
  defineProps<{
    // Included if updating an existing definition
    // Not included if creating a new one (undefined passed from the parent)
    currentTask?: CreatedTaskDefinition | TaskDefinition
    display: boolean
    submitText?: string
    errorText?: string
  }>(),
  {
    currentTask: getDefaultTaskDefinition,
    display: false,
  }
)

const localTask = ref(props.currentTask)

const mode = computed(() => (isExistingDefinition(localTask.value) ? Mode.UPDATE : Mode.CREATE))
const isRecurring = computed(() => isRecurringDefinition(localTask.value))
const recurrenceInterval = computed(() =>
  convertMinutesToHighestUnit(localTask.value.recurrence?.minutes)
)

watch(
  () => props.currentTask,
  (newVal) => {
    console.log('Task changed to ', newVal)
    localTask.value = newVal
    // isRecurring.value = isRecurringDefinition(localTask.value)

    // if (isRecurring.value) {
    //   localTask.value.recurrence = getDefaultRecurrence()
    // }
  }
)

// watch(
//   () => localTask.value,
//   (newVal) => {
//     isRecurring.value = isRecurringDefinition(newVal)
//     console.log(isRecurring.value)
//   },
// )

// TODO: Is this actually needed?
watch(
  () => props.display,
  (shown) => {
    if (!shown) {
      localTask.value = getDefaultTaskDefinition()
    }
  }
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
  { immediate: true }
)

defineEmits(['create', 'update', 'close'])
</script>

<style></style>
