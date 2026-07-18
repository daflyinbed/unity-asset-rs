<template>
  <main class="p-8 font-sans" style="color: var(--text)">
    <h1 class="mb-4 text-2xl">Home</h1>
    <RouterLink to="/color-palette" class="underline">
      Go to /color-palette
    </RouterLink>
  </main>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>
