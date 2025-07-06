<template>
  <!-- https://v2.tauri.app/learn/window-customization/#creating-a-custom-titlebar -->
  <div class="titlebar">
    <div data-tauri-drag-region></div>
    <div class="controls">
      <button id="titlebar-minimize" title="minimize">
        <!-- https://api.iconify.design/mdi:window-minimize.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M19 13H5v-2h14z" />
        </svg>
      </button>
      <button id="titlebar-maximize" title="maximize">
        <!-- https://api.iconify.design/mdi:window-maximize.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
        </svg>
      </button>
      <button id="titlebar-close" title="close">
        <!-- https://api.iconify.design/mdi:close.svg -->
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
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
import Tasks from "./components/PTasks.vue"

</script>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: grid;
  grid-template-columns: auto max-content;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar>.controls {
  display: flex;
}

.titlebar button {
  appearance: none;
  padding: 0;
  margin: 0;
  border: none;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  background-color: transparent;
}

.titlebar button:hover {
  background: #5bbec3;
}
</style>
