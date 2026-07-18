<template>
  <div
    class="h-20 flex flex-col justify-end overflow-hidden border border-gray-300 rounded-md p-2"
    :class="className"
  >
    <span
      class="text-xs leading-tight font-mono"
      style="
        color: var(--my-color-neutral-10);
        background: var(--my-color-background);
        padding: 2px 4px;
        border-radius: 3px;
      "
    >
      {{ label }}
    </span>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  className: string;
  label: string;
}>();
</script>
