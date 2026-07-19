<template>
  <TooltipProvider :delay-duration="300" :skip-delay-duration="100">
    <TabsRoot
      class="h-10 min-w-0 flex shrink-0 items-stretch border-b border-neutral-5 bg-tabbar"
      :model-value="activeId ?? undefined"
      orientation="horizontal"
      @update:model-value="selectTab"
    >
      <div
        class="flex shrink-0 items-center"
        aria-label="Tab history"
        role="group"
      >
        <button
          class="group h-full w-9 flex cursor-pointer items-center justify-center text-neutral-10/60 outline-none disabled:cursor-not-allowed disabled:text-neutral-7 focus-visible:text-neutral-10 hover:text-neutral-10"
          type="button"
          :disabled="!canGoBack"
          aria-label="Go back"
          title="Go back"
          @click="go(-1)"
        >
          <span
            class="rounded p-1"
            :class="
              canGoBack
                ? 'group-focus-visible:bg-neutral-1 group-hover:bg-neutral-1 group-active:bg-neutral-2'
                : ''
            "
          >
            <i-lucide-chevron-left class="h-4 w-4" />
          </span>
        </button>
        <button
          class="group h-full w-9 flex cursor-pointer items-center justify-center text-neutral-10/60 outline-none disabled:cursor-not-allowed disabled:text-neutral-7 focus-visible:text-neutral-10 hover:text-neutral-10"
          type="button"
          :disabled="!canGoForward"
          aria-label="Go forward"
          title="Go forward"
          @click="go(1)"
        >
          <span
            class="rounded p-1"
            :class="
              canGoForward
                ? 'group-focus-visible:bg-neutral-1 group-hover:bg-neutral-1 group-active:bg-neutral-2'
                : ''
            "
          >
            <i-lucide-chevron-right class="h-4 w-4" />
          </span>
        </button>
      </div>
      <span class="h-full w-px shrink-0 bg-neutral-5" aria-hidden="true" />

      <div
        ref="viewport"
        class="editor-tabs-viewport min-w-0 flex-1 overflow-x-auto"
        @wheel="scrollHorizontally"
      >
        <TabsList
          class="h-full min-w-max flex items-stretch"
          aria-label="Open assets"
        >
          <AssetContextMenu
            v-for="tab in tabs"
            :key="tab.id"
            :asset="tab"
            :tab-ids="tabIds"
            @close="emit('menuClose', tab.id, $event)"
          >
            <div
              class="group h-full flex shrink-0 items-center"
              :class="
                tab.id === activeId
                  ? 'bg-background text-neutral-10'
                  : 'text-neutral-10/50 hover:bg-neutral-1 hover:text-neutral-10/80 active:bg-neutral-2'
              "
              :data-editor-tab="tab.id"
              @mousedown.middle.prevent="emit('close', tab.id)"
            >
              <TooltipRoot>
                <TooltipTrigger as-child>
                  <TabsTrigger
                    class="h-full min-w-0 flex items-center gap-2 border-0 bg-transparent pl-3 pr-1 text-inherit outline-none focus-visible:ring-2 focus-visible:ring-accent-5 focus-visible:ring-inset"
                    :value="tab.id"
                    @dblclick="emit('pin', tab.id)"
                  >
                    <TypeIcon
                      :type="tab.type"
                      class="h-3.5 w-3.5 shrink-0 text-accent-10"
                    />
                    <span
                      class="max-w-44 truncate"
                      :class="tab.pinned ? '' : 'italic'"
                      >{{ tab.name }}</span
                    >
                    <span
                      v-if="tab.mode === 'dump'"
                      class="rounded bg-neutral-2 px-1 text-[10px] text-neutral-10/60 uppercase"
                      >dump</span
                    >
                  </TabsTrigger>
                </TooltipTrigger>
                <TooltipPortal>
                  <TooltipContent
                    class="z-70 max-w-96 border border-neutral-5 rounded-md bg-panel px-2.5 py-1.5 text-[12px] text-neutral-10 shadow-lg"
                    :side-offset="6"
                    side="bottom"
                  >
                    <span class="break-all">{{ tab.name }}</span>
                    <TooltipArrow class="fill-panel" />
                  </TooltipContent>
                </TooltipPortal>
              </TooltipRoot>
              <button
                class="pointer-events-none mr-1 cursor-pointer rounded p-1 text-neutral-10/40 opacity-0 outline-none transition-opacity group-hover:pointer-events-auto active:bg-neutral-4 focus-visible:bg-neutral-3 hover:bg-neutral-3 focus-visible:text-neutral-10 group-hover:text-neutral-10 focus-visible:opacity-100 group-hover:opacity-100"
                type="button"
                :aria-label="`Close ${tab.name}`"
                @click="emit('close', tab.id)"
              >
                <i-lucide-x class="h-3 w-3" />
              </button>
              <span
                class="h-full w-px shrink-0 bg-neutral-5"
                aria-hidden="true"
              />
            </div>
          </AssetContextMenu>
        </TabsList>
      </div>
    </TabsRoot>
  </TooltipProvider>
</template>

<script setup lang="ts">
import {
  TabsList,
  TabsRoot,
  TabsTrigger,
  TooltipArrow,
  TooltipContent,
  TooltipPortal,
  TooltipProvider,
  TooltipRoot,
  TooltipTrigger,
} from "reka-ui";
import { computed, nextTick, ref, watch } from "vue";
import TypeIcon from "./asset/TypeIcon.vue";
import AssetContextMenu from "./AssetContextMenu.vue";
import type { TabCloseAction } from "./AssetContextMenu.vue";

export interface EditorTabItem {
  id: string;
  container: string;
  mode: "preview" | "dump";
  name: string;
  pathId: string;
  pinned: boolean;
  type: string;
}

const props = defineProps<{
  activeId: string | null;
  tabs: EditorTabItem[];
}>();

const emit = defineEmits<{
  close: [id: string];
  menuClose: [id: string, action: TabCloseAction];
  pin: [id: string];
  "update:activeId": [id: string];
}>();

const viewport = ref<HTMLElement | null>(null);
const history = ref<string[]>([]);
const historyIndex = ref(-1);
const navigatingHistory = ref(false);

const openIds = computed(() => new Set(props.tabs.map((tab) => tab.id)));
const tabIds = computed(() => props.tabs.map((tab) => tab.id));
const canGoBack = computed(() => historyTarget(-1) !== -1);
const canGoForward = computed(() => historyTarget(1) !== -1);

watch(
  () => props.activeId,
  (id) => {
    if (id) {
      if (navigatingHistory.value) {
        navigatingHistory.value = false;
      } else if (history.value[historyIndex.value] !== id) {
        history.value = history.value.slice(0, historyIndex.value + 1);
        history.value.push(id);
        historyIndex.value = history.value.length - 1;
      }
    }
    void scrollActiveIntoView(id);
  },
  { immediate: true },
);

function selectTab(value: string | number) {
  emit("update:activeId", String(value));
}

function historyTarget(direction: -1 | 1) {
  for (
    let index = historyIndex.value + direction;
    index >= 0 && index < history.value.length;
    index += direction
  ) {
    if (openIds.value.has(history.value[index])) return index;
  }
  return -1;
}

function go(direction: -1 | 1) {
  const index = historyTarget(direction);
  if (index === -1) return;
  historyIndex.value = index;
  navigatingHistory.value = true;
  emit("update:activeId", history.value[index]);
}

async function scrollActiveIntoView(id: string | null) {
  if (!id) return;
  await nextTick();
  const tab = [
    ...(viewport.value?.querySelectorAll<HTMLElement>("[data-editor-tab]") ??
      []),
  ].find((element) => element.dataset.editorTab === id);
  tab?.scrollIntoView({
    behavior: "smooth",
    block: "nearest",
    inline: "nearest",
  });
}

function scrollHorizontally(event: WheelEvent) {
  const element = viewport.value;
  if (
    !element ||
    element.scrollWidth <= element.clientWidth ||
    Math.abs(event.deltaX) >= Math.abs(event.deltaY)
  ) {
    return;
  }
  event.preventDefault();
  element.scrollLeft += event.deltaY;
}
</script>

<style scoped>
.editor-tabs-viewport {
  scrollbar-width: none;
}

.editor-tabs-viewport::-webkit-scrollbar {
  display: none;
}
</style>
