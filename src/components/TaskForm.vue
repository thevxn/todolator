<template>
  <div
    v-if="display"
    class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 z-2 bg-primary rounded-md min-w-1/2 overflow-y-auto"
  >
    <div class="mt-1">
      <PIcon
        :icon="'mingcute:add-line'"
        class="px-2"
        :class="`text-${highlightClass}`"
        :height="'30px'"
        :width="'30px'"
        v-if="mode === Mode.CREATE"
      />
      <PIcon
        :icon="'mingcute:edit-2-line'"
        class="px-2 rounded-md"
        :class="`text-${highlightClass}`"
        :height="'30px'"
        :width="'30px'"
        v-if="mode === Mode.UPDATE"
      />
    </div>
    <div class="px-5 pb-5 pt-2">
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
          :class="mode === Mode.UPDATE ? 'edit' : null"
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
          class="w-full min-h-25"
          :class="mode === Mode.UPDATE ? 'edit' : null"
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
            class="w-full cursor-pointer"
            :class="mode === Mode.UPDATE ? 'edit' : null"
            tabindex="3"
            step="60"
            required
          />
        </div>

        <div class="flex flex-row w-full items-center mt-2" v-if="mode === Mode.CREATE">
          <PIcon
            :icon="'mingcute:information-line'"
            width="24px"
            height="24px"
            class="text-secondary outline-none hover:cursor-help"
            title="A recurring task keeps repeating in the specified interval. You cannot change the recurrence of a task after it has been created."
          />
          <label for="recurring" class="text-left font-bold mr-2 ml-0.5">Recurring?</label>
          <input
            type="checkbox"
            class="w-5 h-5 rounded-md appearance-auto !mb-0 hover:cursor-pointer"
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

        <div
          class="flex flex-row w-full items-center gap-x-2 pl-6"
          v-if="isRecurring && mode === Mode.CREATE"
        >
          <label for="recurrence-minutes" class="text-left">Repeat every</label>
          <input
            v-if="isRecurring"
            v-model="unitAmount"
            type="number"
            class="rounded h-8 hover:cursor-pointer"
            :style="{ width: `${Math.max(String(unitAmount || 3).length + 4, 5)}ch` }"
            name="recurrence-minutes"
            id="recurrence-minutes"
            tabindex="5"
            min="1"
            autocomplete="off"
          />
          <select
            id="unit"
            name="unit"
            class="rounded bg-primary hover:cursor-pointer"
            v-model="unitName"
            tabindex="6"
          >
            <option
              v-for="[unit] of Object.entries(timeUnitToMinutesMap)"
              :key="unit"
              :value="unit"
              class="hover:bg-secondary hover:text-primary hover:font-bold hover:cursor-pointer"
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
              <b class="text-warning">{{ new Date(localTask.start).toLocaleString() }}</b
              >,</span
            >
            <br />
            <span
              >repeats every
              <span class="text-warning font-bold">{{ recurrence![Recurrence.AMOUNT] }}</span>
              {{
                (recurrence![Recurrence.AMOUNT] as number) > 1
                  ? pluralizeUnit(recurrence![Recurrence.UNIT] as string)
                  : recurrence![Recurrence.UNIT]
              }}</span
            >
          </div>
        </div>
        <div class="flex flex-col items-center justify-center mt-4 mb-5">
          <button
            @click="mode === Mode.CREATE ? $emit('create', localTask) : $emit('update', localTask)"
            tabindex="7"
            :class="{
              'bg-warning border-warning hover:text-warning hover:bg-primary active:bg-warning active:border-warning active:text-primary':
                mode === Mode.UPDATE
            }"
          >
            {{ submitText }}
          </button>
          <span class="text-error text-lg" v-if="errorText">{{ errorText }}</span>
        </div>
      </div>
    </div>
    <PHotkeys
      screen-code="NEW_TASK_MODAL"
      :color="mode === Mode.UPDATE ? 'bg-warning' : undefined"
    />
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
import PIcon from './PIcon.vue'

const props = withDefaults(
  defineProps<{
    // Provided if updating an existing definition
    // Not provided if creating a new one (undefined passed from the parent)
    currentTask?: CreatedTaskDefinition | TaskDefinition

    display: boolean
    submitText?: string
    errorText?: string
  }>(),
  {
    currentTask: () => getDefaultTaskDefinition(),
    display: false
  }
)

const localTask = ref(props.currentTask)

enum Mode {
  CREATE,
  UPDATE
}
const mode = computed(() => (isExistingDefinition(localTask.value) ? Mode.UPDATE : Mode.CREATE))
const highlightClass = computed(() => (mode.value === Mode.CREATE ? 'secondary' : 'warning'))

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

const focusInput = ref<HTMLInputElement | null>(null)
watch(
  () => props.display,
  (display) => {
    if (display) {
      nextTick(() => {
        // Focus the first input
        focusInput.value?.focus()
      })
    } else {
      // Reset the input values
      localTask.value = getDefaultTaskDefinition()
      unitAmount.value = 1
      unitName.value = 'minute'
    }
  }
)

defineEmits(['create', 'update', 'close'])
</script>

<style></style>
