<template>
  <main
    class="min-h-screen p-8 font-sans"
    style="background: var(--bg); color: var(--text)"
  >
    <h1 class="mb-8 text-2xl">Color Palette</h1>

    <!-- 独立背景色 -->
    <section class="mb-10">
      <h2 class="mb-3 text-lg">Background Colors</h2>
      <div class="grid grid-cols-4 gap-3">
        <Swatch
          v-for="c in backgrounds"
          :key="c.className"
          :class-name="c.className"
          :label="c.className"
        />
      </div>
    </section>

    <!-- 语义色组 -->
    <section v-for="group in semanticGroups" :key="group.name" class="mb-10">
      <h2 class="mb-3 text-lg capitalize">{{ group.name }}</h2>
      <div class="grid grid-cols-6 gap-3">
        <Swatch
          v-for="v in group.variants"
          :key="v"
          :class-name="`bg-${group.name}-${v}`"
          :label="`bg-${group.name}-${v}`"
        />
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import Swatch from "../components/Swatch.vue";

const backgrounds = [
  { className: "bg-background" },
  { className: "bg-panel" },
  { className: "bg-surface" },
  { className: "bg-overlay" },
];

const semanticGroups = [
  {
    name: "accent" as const,
    variants: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, "contrast"] as const,
  },
  {
    name: "neutral" as const,
    variants: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, "contrast"] as const,
  },
];
</script>
