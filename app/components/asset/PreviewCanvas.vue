<template>
  <div
    class="checker relative flex items-center justify-center overflow-hidden"
  >
    <TypeIcon :type="asset.type" class="h-10 w-10 text-neutral-5" />
    <div
      class="absolute inset-x-0 bottom-0 flex items-center justify-between gap-2 px-3 py-2 text-[12px]"
      :class="caption ? 'bg-neutral-10/60 text-neutral-1' : ''"
    >
      <span v-if="caption" class="truncate">{{ asset.name }}</span>
      <span v-if="caption && asset.dims" class="shrink-0 font-mono">{{
        asset.dims
      }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import TypeIcon from "./TypeIcon.vue";
import type { MockAsset } from "../../mock/assets";

withDefaults(defineProps<{ asset: MockAsset; caption?: boolean }>(), {
  caption: true,
});
</script>

<style scoped>
.checker {
  background-image: repeating-conic-gradient(
    var(--my-color-neutral-2) 0% 25%,
    var(--my-color-surface) 0% 50%
  );
  background-size: 16px 16px;
}
</style>
