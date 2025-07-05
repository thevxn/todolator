<template>
  <main>
    <Suspense>
      <Tasks />
    </Suspense>
  </main>
  <PHotkeys />
</template>

<script setup lang="ts">
import Tasks from "./components/PTasks.vue"
import PHotkeys from './components/PHotkeys.vue'

</script>

<style></style>
