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
          autocomplete="off"
        />

        <label for="desc" class="text-left w-full font-bold">Description</label>
        <textarea
          v-model="localTask.desc"
          placeholder="Description"
          name="desc"
          id="desc"
          class="w-full"
          tabindex="2"
          autocomplete="off"
        ></textarea>

        <!-- Recurrence is editable only in create mode -->
        <div
          class="flex flex-col w-full"
          v-if="mode === Mode.CREATE || (mode === Mode.UPDATE && !isRecurring)"
        >
          <label for="timestamp" class="text-left w-full font-bold mb-2"
            >Remind At<span class="text-error">*</span></label
          >
          <input
            v-model="localTask.start"
            name="timestamp"
            id="timestamp"
            type="datetime-local"
            class="w-full"
            tabindex="3"
            step="60"
            required
          />
        </div>

        <div class="flex flex-row w-full gap-x-2 items-center" v-if="mode === Mode.CREATE">
          <label for="recurring" class="text-left font-bold">Recurring?</label>
          <input
            type="checkbox"
            class="w-5 h-5 rounded-md appearance-auto !mb-0"
            name="recurring"
            id="recurring"
            tabindex="4"
            :checked="isRecurring"
            @click="
              isRecurring
                ? (localTask.recurrence = null)
                : (localTask.recurrence = getDefaultRecurrence())
            "
          />
        </div>

        <div class="flex flex-row w-full" v-if="isRecurring && mode === Mode.CREATE">
          <label for="recurrence-minutes" class="text-left">Repeat every</label>
          <input
            v-if="isRecurring"
            v-model="unitAmount"
            type="number"
            class="rounded h-6 w-16 mx-1"
            name="recurrence-minutes"
            id="recurrence-minutes"
            tabindex="5"
            min="1"
            autocomplete="off"
          />
          <select
            id="unit"
            name="unit"
            class="border-2 border-secondary rounded"
            v-model="unitName"
            tabindex="6"
          >
            <option
              v-for="[unit] of Object.entries(timeUnitToMinutesMap)"
              :key="unit"
              :value="unit"
            >
              {{ unitAmount > 1 ? pluralizeUnit(unit) : unit }}
            </option>
          </select>
        </div>

        <small class="text-left w-full mt-4"
          ><span class="text-error">*</span> = Required attribute</small
        >

        <div
          class="flex flex-row items-center justify-start w-full border-t-1 border-t-gray-700 mt-2"
          v-if="mode === Mode.UPDATE && isRecurring"
        >
          <!-- <div class="border-2 px-2 py-1 rounded-md border-[#11d6cc] text-[#11d6cc] font-bold">
            Recurring
          </div> -->
          <div class="mt-2 text-sm text-gray-300" v-if="isRecurring && mode === Mode.UPDATE">
            <span for="timestamp" class="text-left"
              >First recurrence at
              <b class="text-secondary">{{ new Date(localTask.start).toLocaleString() }}</b
              >,</span
            >
            <br />
            <span
              >repeats every
              <span class="text-secondary font-bold">{{ recurrence![Recurrence.AMOUNT] }}</span>
              {{
                (recurrence![Recurrence.AMOUNT] as number) > 1
                  ? pluralizeUnit(recurrence![Recurrence.UNIT] as string)
                  : recurrence![Recurrence.UNIT]
              }}</span
            >
          </div>
        </div>

        <button
          class="mt-4 mb-8"
          @click="mode === Mode.CREATE ? $emit('create', localTask) : $emit('update', localTask)"
          tabindex="7"
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
  isRecurringDefinition
} from '../helpers/task'
import {
  convertMinutesToHighestUnit,
  pluralizeUnit,
  timeUnitToMinutesMap
} from '../helpers/datetime'

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
    display: false
  }
)

const localTask = ref(props.currentTask)

enum Mode {
  CREATE,
  UPDATE
}
const mode = computed(() => (isExistingDefinition(localTask.value) ? Mode.UPDATE : Mode.CREATE))

const isRecurring = computed(() => isRecurringDefinition(localTask.value))

enum Recurrence {
  AMOUNT,
  UNIT
}

// Used for determining recurrence unit and amount when editing a task (read-only)
const recurrence = computed(() => convertMinutesToHighestUnit(localTask.value.recurrence?.minutes))

// Used for specifying recurrence when creating a new task
const unitAmount = ref<number>(1)
const unitName = ref<keyof typeof timeUnitToMinutesMap>('minute')

// If the selected time unit or amount changes, recalculate the resulting amount of minutes
watch([unitName, unitAmount], () => {
  if (localTask.value.recurrence) {
    localTask.value.recurrence.minutes = unitAmount.value * timeUnitToMinutesMap[unitName.value]
  }
})

watch(
  () => props.currentTask,
  (newVal) => {
    console.log('Task changed to ', newVal)
    localTask.value = newVal
  }
)

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
