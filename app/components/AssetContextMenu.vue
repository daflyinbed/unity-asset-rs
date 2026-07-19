<template>
  <ContextMenuRoot>
    <ContextMenuTrigger as-child>
      <slot />
    </ContextMenuTrigger>
    <ContextMenuPortal>
      <ContextMenuContent
        class="z-80 min-w-52 border border-neutral-5 rounded-md bg-panel p-1 text-[12px] text-neutral-10 shadow-xl outline-none"
        :collision-padding="8"
      >
        <ContextMenuItem
          class="menu-item"
          :disabled="tabIndex === -1"
          @select="emit('close', 'close')"
        >
          Close
        </ContextMenuItem>
        <ContextMenuItem
          class="menu-item"
          :disabled="tabIndex === -1 || tabIds.length <= 1"
          @select="emit('close', 'others')"
        >
          Close Others
        </ContextMenuItem>
        <ContextMenuItem
          class="menu-item"
          :disabled="tabIndex <= 0"
          @select="emit('close', 'left')"
        >
          Close to the Left
        </ContextMenuItem>
        <ContextMenuItem
          class="menu-item"
          :disabled="tabIndex === -1 || tabIndex === tabIds.length - 1"
          @select="emit('close', 'right')"
        >
          Close to the Right
        </ContextMenuItem>
        <ContextMenuItem
          class="menu-item"
          :disabled="tabIds.length === 0"
          @select="emit('close', 'all')"
        >
          Close All
        </ContextMenuItem>

        <ContextMenuSeparator class="my-1 h-px bg-neutral-5" />

        <ContextMenuItem class="menu-item" @select="copy(asset.name)">
          Copy Name
        </ContextMenuItem>
        <ContextMenuItem class="menu-item" @select="copy(asset.container)">
          Copy Container Name
        </ContextMenuItem>
        <ContextMenuItem class="menu-item" @select="copy(String(asset.pathId))">
          Copy Path ID
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenuPortal>
  </ContextMenuRoot>
</template>

<script setup lang="ts">
import {
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuPortal,
  ContextMenuRoot,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "reka-ui";
import { computed } from "vue";
import type { Asset } from "../assets";

export type TabCloseAction = "close" | "others" | "left" | "right" | "all";

const props = defineProps<{
  asset: Pick<Asset, "container" | "id" | "name" | "pathId">;
  tabIds: string[];
}>();

const emit = defineEmits<{
  close: [action: TabCloseAction];
}>();

const tabIndex = computed(() => props.tabIds.indexOf(props.asset.id));

function copy(value: string) {
  void navigator.clipboard.writeText(value);
}
</script>

<style scoped>
.menu-item {
  display: flex;
  cursor: default;
  align-items: center;
  border-radius: 0.25rem;
  padding: 0.375rem 0.5rem;
  outline: none;
}

.menu-item[data-highlighted] {
  background: var(--my-color-accent-3);
  color: var(--my-color-accent-10);
}

.menu-item[data-disabled] {
  color: var(--my-color-neutral-7);
}
</style>
