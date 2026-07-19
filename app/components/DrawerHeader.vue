<template>
  <div class="shrink-0 border-b border-neutral-5 px-3 py-2.5 space-y-2">
    <div class="flex items-center gap-1">
      <p
        class="mr-auto text-[11px] text-neutral-10/50 font-medium tracking-wide uppercase"
      >
        Objects
      </p>
      <button
        class="rounded p-1.5"
        :class="
          view === 'table'
            ? 'text-accent-10'
            : 'text-neutral-10/50 hover:bg-neutral-1 active:bg-neutral-2'
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
            ? 'text-accent-10'
            : 'text-neutral-10/50 hover:bg-neutral-1 active:bg-neutral-2'
        "
        title="Container path tree"
        @click="$emit('update:view', 'tree')"
      >
        <i-lucide-folder-tree class="h-4 w-4" />
      </button>
    </div>
    <div class="relative">
      <i-lucide-search
        class="absolute left-2 top-1/2 h-3.5 w-3.5 text-neutral-10/50 -translate-y-1/2"
      />
      <input
        :value="query"
        placeholder="Filter…"
        class="w-full border border-neutral-5 rounded-md bg-surface py-1.5 pl-7 pr-17 text-[12px] outline-none focus:border-accent-5 hover:border-neutral-6 placeholder:text-neutral-10/50"
        @input="
          $emit('update:query', ($event.target as HTMLInputElement).value)
        "
      />
      <div
        class="absolute right-1 top-1/2 flex items-center gap-0.5 -translate-y-1/2"
      >
        <button
          type="button"
          class="h-6 w-7 flex items-center justify-center rounded outline-none"
          :class="
            matchCase
              ? 'text-accent-10 hover:bg-neutral-1 active:bg-neutral-2'
              : 'text-neutral-10/50 hover:bg-neutral-1 hover:text-neutral-10 active:bg-neutral-2'
          "
          title="Match case"
          aria-label="Match case"
          :aria-pressed="matchCase"
          @click="$emit('update:matchCase', !matchCase)"
        >
          <i-lucide-case-sensitive class="h-4 w-4" />
        </button>
        <button
          type="button"
          class="h-6 w-7 flex items-center justify-center rounded outline-none"
          :class="
            useRegex
              ? 'text-accent-10 hover:bg-neutral-1 active:bg-neutral-2'
              : 'text-neutral-10/50 hover:bg-neutral-1 hover:text-neutral-10 active:bg-neutral-2'
          "
          title="Use regular expression"
          aria-label="Use regular expression"
          :aria-pressed="useRegex"
          @click="$emit('update:useRegex', !useRegex)"
        >
          <i-lucide-regex class="h-4 w-4" />
        </button>
      </div>
    </div>
    <SelectRoot :model-value="type" multiple @update:model-value="updateType">
      <SelectTrigger
        as="div"
        class="relative min-h-8 w-full flex flex-wrap cursor-pointer items-center gap-1 border border-neutral-5 rounded-md bg-surface py-1 pl-1.5 pr-8 text-[12px] text-neutral-10 outline-none focus:border-accent-5 hover:border-neutral-6"
        aria-label="Filter by object type"
      >
        <span v-if="type.length === 0" class="min-w-0 flex-1 px-0.5 text-left">
          All types
        </span>
        <span
          v-for="selectedType in type"
          v-else
          :key="selectedType"
          class="max-w-full flex items-center gap-1 rounded bg-accent-2 py-0.5 pl-1.5 pr-1 text-accent-10"
        >
          <span class="truncate">{{ selectedType }}</span>
          <button
            type="button"
            class="group h-5 w-5 flex shrink-0 cursor-pointer items-center justify-center text-accent-10/60 outline-none hover:text-accent-10"
            :aria-label="`Remove ${selectedType} filter`"
            @pointerdown.prevent.stop="removeType(selectedType)"
            @click.prevent.stop
          >
            <span
              class="h-4 w-4 flex items-center justify-center rounded-sm group-focus-visible:bg-accent-3 group-hover:bg-accent-3"
            >
              <i-lucide-x class="block h-3 w-3" />
            </span>
          </button>
        </span>
        <SelectIcon
          class="absolute right-2 top-1/2 shrink-0 text-neutral-10/50 -translate-y-1/2"
        >
          <i-lucide-chevron-down class="h-3.5 w-3.5" />
        </SelectIcon>
      </SelectTrigger>
      <SelectPortal>
        <SelectContent
          position="popper"
          :side-offset="4"
          class="z-80 max-h-72 min-w-[var(--reka-select-trigger-width)] overflow-hidden border border-neutral-5 rounded-md bg-panel p-1 text-[12px] text-neutral-10 shadow-xl"
        >
          <SelectViewport>
            <SelectItem
              v-for="t in types"
              :key="t"
              :value="t"
              class="type-option"
            >
              <SelectItemText>{{ t }}</SelectItemText>
              <SelectItemIndicator class="ml-auto shrink-0 text-accent-10">
                <i-lucide-check class="h-3.5 w-3.5" />
              </SelectItemIndicator>
            </SelectItem>
          </SelectViewport>
        </SelectContent>
      </SelectPortal>
    </SelectRoot>
  </div>
</template>

<script setup lang="ts">
import {
  SelectContent,
  SelectIcon,
  SelectItem,
  SelectItemIndicator,
  SelectItemText,
  SelectPortal,
  SelectRoot,
  SelectTrigger,
  SelectViewport,
} from "reka-ui";

const props = defineProps<{
  view: "table" | "tree";
  query: string;
  matchCase: boolean;
  useRegex: boolean;
  type: string[];
  types: string[];
}>();

const emit = defineEmits<{
  "update:view": [v: "table" | "tree"];
  "update:query": [v: string];
  "update:matchCase": [value: boolean];
  "update:useRegex": [value: boolean];
  "update:type": [value: string[]];
}>();

function updateType(value: unknown) {
  emit("update:type", Array.isArray(value) ? value.map(String) : []);
}

function removeType(type: string) {
  emit(
    "update:type",
    props.type.filter((selectedType) => selectedType !== type),
  );
}
</script>

<style scoped>
.type-option {
  display: flex;
  cursor: pointer;
  align-items: center;
  gap: 0.375rem;
  border-radius: 0.25rem;
  padding: 0.375rem 0.5rem;
  outline: none;
}

.type-option[data-highlighted] {
  background: var(--my-color-accent-3);
  color: var(--my-color-accent-10);
}
</style>
