<template>
  <div class="shrink-0 border-b border-neutral-2 px-3 py-2.5 space-y-2">
    <div class="flex items-center gap-1">
      <p
        class="mr-auto text-[11px] text-neutral-5 font-medium tracking-wide uppercase"
      >
        Objects
      </p>
      <button
        class="rounded p-1.5"
        :class="
          view === 'table'
            ? 'bg-accent-2 text-accent-7'
            : 'text-neutral-5 hover:bg-neutral-2'
        "
        title="Asset table"
        @click="$emit('update:view', 'table')"
      >
        <i-lucide-table class="h-4 w-4" />
      </button>
      <button
        class="rounded p-1.5"
        :class="
          view === 'tree'
            ? 'bg-accent-2 text-accent-7'
            : 'text-neutral-5 hover:bg-neutral-2'
        "
        title="Container path tree"
        @click="$emit('update:view', 'tree')"
      >
        <i-lucide-folder-tree class="h-4 w-4" />
      </button>
    </div>
    <div class="relative">
      <i-lucide-search
        class="absolute left-2 top-1/2 h-3.5 w-3.5 text-neutral-5 -translate-y-1/2"
      />
      <input
        :value="query"
        placeholder="Filter…"
        class="w-full border border-neutral-3 rounded-md bg-surface py-1.5 pl-7 pr-2 text-[12px] outline-none focus:border-accent-5 placeholder:text-neutral-5"
        @input="
          $emit('update:query', ($event.target as HTMLInputElement).value)
        "
      />
    </div>
    <select
      :value="type ?? ''"
      class="w-full border border-neutral-3 rounded-md bg-surface px-2 py-1.5 text-[12px] text-neutral-7 outline-none focus:border-accent-5"
      @change="onTypeChange"
    >
      <option value="">All types</option>
      <option v-for="t in TYPES" :key="t" :value="t">{{ t }}</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import type { AssetType } from "../../mock/assets";

defineProps<{
  view: "table" | "tree";
  query: string;
  type: AssetType | null;
}>();

const emit = defineEmits<{
  "update:view": [v: "table" | "tree"];
  "update:query": [v: string];
  "update:type": [v: AssetType | null];
}>();

const TYPES: AssetType[] = [
  "Texture2D",
  "Sprite",
  "Material",
  "Mesh",
  "MonoBehaviour",
  "RectTransform",
  "SpriteRenderer",
  "TextAsset",
  "AudioClip",
  "AnimatorController",
];

function onTypeChange(e: Event) {
  const v = (e.target as HTMLSelectElement).value;
  emit("update:type", v === "" ? null : (v as AssetType));
}
</script>
