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
      <button id="titlebar-maximize" title="maximize" @click="maximizeWindow">
        <!-- https://api.iconify.design/mdi:window-maximize.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
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
      <Tasks />
    </Suspense>
  </main>
</template>

<script setup lang="ts">
import logoUrl from './assets/logo.png'
import Tasks from "./components/PTasks.vue"
import { invoke } from "@tauri-apps/api/core";

async function maximizeWindow() {
  try {
    await invoke("maximize");
  } catch (e) {
    console.error("Failed to maximize window:", e);
  }
}

async function minimizeWindow() {
  try {
    await invoke("minimize");
  } catch (e) {
    console.error("Failed to minimize window:", e);
  }
}

async function closeWindow() {
  try {
    await invoke("close");
  } catch (e) {
    console.error("Failed to close window:", e);
  }
}
</script>

<style></style>
