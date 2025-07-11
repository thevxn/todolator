<template>
  <!-- https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar -->
  <div class="titlebar">
    <div data-tauri-drag-region>
      <img :src="logoUrl" width="30">
    </div>
    <div class="controls">
      <button id="titlebar-minimize" title="minimize" @click="minimizeWindow">
        <!-- https://api.iconify.design/mdi:window-minimize.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M19 13H5v-2h14z" />
        </svg>
      </button>
      <button id="titlebar-close" title="close" @click="closeWindow" class="hover:bg-error">
        <!-- https://api.iconify.design/mdi:close.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="hover:bg-error">
          <path fill="currentColor"
            d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z" />
        </svg>
      </button>
    </div>
  </div>
  <main>
    <Suspense>
      <ReminderPopup @close="closeWindow" />
    </Suspense>
  </main>
</template>

<script setup lang="ts">
import logoUrl from './assets/logo.png'
import { invoke } from "@tauri-apps/api/core";
import ReminderPopup from './components/ReminderPopup.vue';

async function minimizeWindow() {
  try {
    await invoke("minimize");
  } catch (e) {
    console.error("Failed to minimize window:", e);
  }
}

async function closeWindow() {
  try {
    await invoke("complete_task");
  }
  catch (e) {
    console.log("Failed to complete task: ", e);
  }

  try {
    await invoke("close");
  } catch (e) {
    console.error("Failed to close window:", e);
  }
}
</script>

<style></style>
