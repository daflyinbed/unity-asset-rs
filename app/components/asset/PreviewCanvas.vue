<template>
  <div
    class="checker relative flex items-center justify-center overflow-hidden"
  >
    <img
      v-if="imageUrl"
      :alt="asset.name"
      class="h-full w-full object-contain"
      draggable="false"
      :src="imageUrl"
    />
    <div
      v-else-if="loading"
      class="flex items-center gap-2 rounded bg-panel px-3 py-2 text-neutral-10/60"
    >
      <i-lucide-loader-circle class="h-4 w-4 animate-spin" />
      <span>Decoding texture…</span>
    </div>
    <div
      v-else-if="error"
      class="mx-6 max-w-md flex flex-col items-center gap-2 text-center text-danger-10"
    >
      <i-lucide-circle-alert class="h-8 w-8" />
      <span>{{ error }}</span>
    </div>
    <TypeIcon v-else :type="asset.type" class="h-10 w-10 text-neutral-10/50" />
    <div
      class="absolute inset-x-0 bottom-0 flex items-center justify-between gap-2 px-3 py-2 text-[12px]"
      :class="caption ? 'bg-neutral-7 text-neutral-contrast' : ''"
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
import type { Asset } from "../../assets";

withDefaults(
  defineProps<{
    asset: Asset;
    caption?: boolean;
    imageUrl?: string;
    loading?: boolean;
    error?: string;
  }>(),
  { caption: true, imageUrl: "", loading: false, error: "" },
);
</script>

<style scoped>
.checker {
  background-image: repeating-conic-gradient(
    var(--my-color-panel) 0% 25%,
    var(--my-color-background) 0% 50%
  );
  background-size: 16px 16px;
}
</style>
