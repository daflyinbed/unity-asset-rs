<template>
  <div
    class="h-screen flex flex-col select-none bg-background text-left text-[13px]"
  >
    <!-- ═══ Row 1: title bar ═══ -->
    <header
      class="h-10 flex shrink-0 items-center gap-2 border-b border-neutral-2 bg-panel px-2"
    >
      <button class="rounded p-1.5 text-neutral-6 hover:bg-neutral-2">
        <i-lucide-menu class="h-4 w-4" />
      </button>
      <span class="text-neutral-9 font-semibold">unity-asset-rs</span>
      <span
        class="flex items-center gap-1.5 rounded-md px-2 py-1 text-[12px] text-neutral-6 hover:bg-neutral-2"
      >
        <i-lucide-package class="h-3.5 w-3.5 text-accent-6" />
        {{ bundle.name }}
      </span>
      <span
        class="flex items-center gap-1.5 rounded-md px-2 py-1 text-[12px] text-neutral-6 hover:bg-neutral-2"
      >
        <i-lucide-git-branch class="h-3.5 w-3.5" />
        Unity {{ bundle.version }}
      </span>
      <div class="ml-auto flex items-center">
        <button class="rounded p-2 text-neutral-5 hover:bg-neutral-2">
          <i-lucide-minus class="h-4 w-4" />
        </button>
        <button class="rounded p-2 text-neutral-5 hover:bg-neutral-2">
          <i-lucide-square class="h-3.5 w-3.5" />
        </button>
        <button
          class="rounded p-2 text-neutral-5 hover:bg-red-500/10 hover:text-red-500"
        >
          <i-lucide-x class="h-4 w-4" />
        </button>
      </div>
    </header>

    <!-- ═══ Row 2: tabs + drawer ═══ -->
    <div class="min-h-0 flex flex-1">
      <!-- Drawer (left dock) -->
      <aside
        v-if="drawerSide === 'left'"
        class="w-80 flex shrink-0 flex-col border-r border-neutral-2 bg-panel"
      >
        <DrawerHeader
          v-model:view="drawerView"
          v-model:query="drawerQuery"
          v-model:type="typeFilter"
        />
        <DrawerBody
          :view="drawerView"
          :assets="drawerAssets"
          :active-id="activeTab?.id ?? null"
          @open="openAsset"
        />
      </aside>

      <!-- Editor area -->
      <main class="min-w-0 flex flex-1 flex-col">
        <!-- Tab strip -->
        <div
          class="h-10 flex shrink-0 items-stretch gap-px overflow-x-auto border-b border-neutral-2 bg-panel px-1.5 pt-1.5"
        >
          <button
            v-for="t in tabs"
            :key="t.id"
            class="group min-w-0 flex items-center gap-2 border-x border-t rounded-t-md px-3"
            :class="
              t.id === activeTabId
                ? 'border-neutral-2 bg-background text-neutral-9'
                : 'border-transparent text-neutral-5 hover:bg-neutral-2 hover:text-neutral-8'
            "
            @click="activeTabId = t.id"
          >
            <TypeIcon
              :type="assetOf(t).type"
              class="h-3.5 w-3.5 shrink-0 text-accent-6"
            />
            <span class="max-w-44 truncate">{{ assetOf(t).name }}</span>
            <span
              v-if="t.mode === 'dump'"
              class="rounded bg-neutral-2 px-1 text-[10px] text-neutral-6 uppercase"
              >dump</span
            >
            <span
              class="rounded p-0.5 text-neutral-4 hover:bg-neutral-3 hover:text-neutral-9"
              @click.stop="closeTab(t.id)"
            >
              <i-lucide-x class="h-3 w-3" />
            </span>
          </button>
        </div>

        <!-- Breadcrumb / content toolbar -->
        <div
          v-if="activeAsset"
          class="h-9 flex shrink-0 items-center gap-1.5 border-b border-neutral-2 bg-background px-3 text-[12px]"
        >
          <span class="text-neutral-5">{{ bundle.name }}</span>
          <i-lucide-chevron-right class="h-3 w-3 text-neutral-4" />
          <span class="text-neutral-5">{{ activeAsset.type }}</span>
          <i-lucide-chevron-right class="h-3 w-3 text-neutral-4" />
          <span class="text-neutral-9 font-medium">{{ activeAsset.name }}</span>
          <div class="ml-auto flex items-center gap-0.5">
            <button
              class="rounded p-1.5"
              :class="
                activeTab?.mode === 'preview'
                  ? 'bg-accent-2 text-accent-7'
                  : 'text-neutral-5 hover:bg-neutral-2'
              "
              title="Preview"
              @click="setMode('preview')"
            >
              <i-lucide-eye class="h-4 w-4" />
            </button>
            <button
              class="rounded p-1.5"
              :class="
                activeTab?.mode === 'dump'
                  ? 'bg-accent-2 text-accent-7'
                  : 'text-neutral-5 hover:bg-neutral-2'
              "
              title="Dump"
              @click="setMode('dump')"
            >
              <i-lucide-braces class="h-4 w-4" />
            </button>
            <button
              class="rounded p-1.5 text-neutral-5 hover:bg-neutral-2"
              title="Copy dump"
            >
              <i-lucide-copy class="h-4 w-4" />
            </button>
          </div>
        </div>

        <!-- Content -->
        <div class="min-h-0 flex-1 overflow-auto">
          <!-- Empty state -->
          <div
            v-if="!activeAsset"
            class="h-full flex flex-col items-center justify-center gap-2 text-neutral-5"
          >
            <i-lucide-mouse-pointer-click class="h-8 w-8 text-neutral-4" />
            <p>Select an object from the drawer to open it</p>
            <p class="text-[12px] text-neutral-4">
              Toggle the drawer with the panel buttons in the status bar
            </p>
          </div>

          <!-- Preview -->
          <div
            v-else-if="activeTab?.mode === 'preview'"
            class="h-full flex flex-col"
          >
            <PreviewCanvas :asset="activeAsset" class="min-h-0 flex-1" />
            <div
              class="grid grid-cols-4 shrink-0 border-t border-neutral-2 bg-surface divide-x divide-neutral-2"
            >
              <div v-for="row in metaRows" :key="row.k" class="px-4 py-3">
                <p class="text-[11px] text-neutral-5 tracking-wide uppercase">
                  {{ row.k }}
                </p>
                <p class="mt-0.5 truncate text-neutral-9 font-mono">
                  {{ row.v }}
                </p>
              </div>
            </div>
          </div>

          <!-- Dump -->
          <div v-else class="flex bg-surface text-[12.5px] leading-6 font-mono">
            <div
              class="shrink-0 select-none border-r border-neutral-2 bg-panel px-3 text-right text-neutral-4"
            >
              <div v-for="(_, i) in dumpLines" :key="i">{{ i + 1 }}</div>
            </div>
            <div class="min-w-0 flex-1 px-4 py-0">
              <div
                v-for="(l, i) in dumpLines"
                :key="i"
                class="whitespace-pre"
                :style="{ paddingLeft: `${l.depth * 1.5}rem` }"
              >
                <span
                  :class="
                    l.header
                      ? 'font-semibold text-neutral-10'
                      : 'text-neutral-8'
                  "
                  >{{ l.key }}</span
                >
                <span
                  v-if="l.value !== undefined"
                  :class="l.kind === 'str' ? 'text-accent-7' : 'text-neutral-5'"
                >
                  {{ ` ${l.value}` }}</span
                >
              </div>
            </div>
          </div>
        </div>

        <!-- Bottom panel (log) -->
        <div
          v-if="bottomOpen"
          class="h-32 shrink-0 overflow-auto border-t border-neutral-2 bg-panel px-4 py-2 text-[12px] text-neutral-6 leading-5 font-mono"
        >
          <p v-for="(line, i) in logLines" :key="i">
            <span class="text-neutral-4">[{{ line.t }}]</span>
            <span :class="line.accent ? 'text-accent-7' : ''">{{
              ` ${line.msg}`
            }}</span>
          </p>
        </div>
      </main>

      <!-- Drawer (right dock) -->
      <aside
        v-if="drawerSide === 'right'"
        class="w-80 flex shrink-0 flex-col border-l border-neutral-2 bg-panel"
      >
        <DrawerHeader
          v-model:view="drawerView"
          v-model:query="drawerQuery"
          v-model:type="typeFilter"
        />
        <DrawerBody
          :view="drawerView"
          :assets="drawerAssets"
          :active-id="activeTab?.id ?? null"
          @open="openAsset"
        />
      </aside>
    </div>

    <!-- ═══ Row 3: status bar ═══ -->
    <footer
      class="h-8 flex shrink-0 items-center gap-1 border-t border-neutral-2 bg-panel px-2 text-[12px] text-neutral-6"
    >
      <button
        class="rounded p-1.5 hover:bg-neutral-2 hover:text-neutral-9"
        title="Search objects"
        @click="drawerSide = drawerSide ?? 'right'"
      >
        <i-lucide-search class="h-4 w-4" />
      </button>
      <span class="flex items-center gap-1 px-1.5">
        <i-lucide-circle-check class="h-4 w-4 text-accent-7" />
        {{ bundle.objects.toLocaleString() }} objects
      </span>
      <span class="px-1.5 text-neutral-5">{{ drawerAssets.length }} shown</span>

      <div class="ml-auto flex items-center gap-1">
        <template v-if="activeAsset">
          <span class="px-1.5 font-mono">{{ activeAsset.pathId }}</span>
          <span class="px-1.5">{{ activeAsset.type }}</span>
          <span class="px-1.5 text-neutral-4">·</span>
          <span class="px-1.5 capitalize">{{ activeTab?.mode }}</span>
          <span class="px-1.5 text-neutral-4">·</span>
        </template>
        <button
          class="rounded p-1.5"
          :class="
            drawerSide === 'left'
              ? 'bg-accent-2 text-accent-7'
              : 'text-neutral-6 hover:bg-neutral-2 hover:text-neutral-9'
          "
          title="Toggle left drawer"
          @click="toggleSide('left')"
        >
          <i-lucide-panel-left class="h-4 w-4" />
        </button>
        <button
          class="rounded p-1.5"
          :class="
            bottomOpen
              ? 'bg-accent-2 text-accent-7'
              : 'text-neutral-6 hover:bg-neutral-2 hover:text-neutral-9'
          "
          title="Toggle bottom panel"
          @click="bottomOpen = !bottomOpen"
        >
          <i-lucide-panel-bottom class="h-4 w-4" />
        </button>
        <button
          class="rounded p-1.5"
          :class="
            drawerSide === 'right'
              ? 'bg-accent-2 text-accent-7'
              : 'text-neutral-6 hover:bg-neutral-2 hover:text-neutral-9'
          "
          title="Toggle right drawer"
          @click="toggleSide('right')"
        >
          <i-lucide-panel-right class="h-4 w-4" />
        </button>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import PreviewCanvas from "../components/asset/PreviewCanvas.vue";
import TypeIcon from "../components/asset/TypeIcon.vue";
import DrawerBody from "../components/zed/DrawerBody.vue";
import DrawerHeader from "../components/zed/DrawerHeader.vue";
import { formatSize, MOCK_ASSETS, MOCK_BUNDLES } from "../mock/assets";
import type { AssetType, MockAsset } from "../mock/assets";

type Mode = "preview" | "dump";
interface OpenTab {
  id: number;
  mode: Mode;
}

const bundle = MOCK_BUNDLES[0];

const drawerSide = ref<"left" | "right" | null>("right");
const bottomOpen = ref(false);
const drawerView = ref<"table" | "tree">("table");
const drawerQuery = ref("");
const typeFilter = ref<AssetType | null>(null);

const tabs = ref<OpenTab[]>([{ id: MOCK_ASSETS[1].id, mode: "preview" }]);
const activeTabId = ref<number | null>(tabs.value[0].id);

const activeTab = computed(
  () => tabs.value.find((t) => t.id === activeTabId.value) ?? null,
);
const activeAsset = computed(() =>
  activeTab.value ? assetOf(activeTab.value) : null,
);

function assetOf(tab: OpenTab): MockAsset {
  return MOCK_ASSETS.find((a) => a.id === tab.id) ?? MOCK_ASSETS[0];
}

function openAsset(asset: MockAsset) {
  if (!tabs.value.some((t) => t.id === asset.id)) {
    tabs.value.push({ id: asset.id, mode: "preview" });
  }
  activeTabId.value = asset.id;
}

function closeTab(id: number) {
  const i = tabs.value.findIndex((t) => t.id === id);
  tabs.value.splice(i, 1);
  if (activeTabId.value === id) {
    activeTabId.value =
      tabs.value[Math.min(i, tabs.value.length - 1)]?.id ?? null;
  }
}

function setMode(mode: Mode) {
  if (activeTab.value) activeTab.value.mode = mode;
}

function toggleSide(side: "left" | "right") {
  drawerSide.value = drawerSide.value === side ? null : side;
}

const drawerAssets = computed(() =>
  MOCK_ASSETS.filter(
    (a) =>
      (typeFilter.value === null || a.type === typeFilter.value) &&
      (a.name + a.container)
        .toLowerCase()
        .includes(drawerQuery.value.toLowerCase()),
  ),
);

const metaRows = computed(() => {
  const a = activeAsset.value;
  if (!a) return [];
  return [
    { k: "Type", v: a.type },
    { k: "Size", v: formatSize(a.size) },
    { k: "PathID", v: String(a.pathId) },
    { k: "Dims", v: a.dims ?? "—" },
  ];
});

interface DumpLine {
  depth: number;
  key: string;
  value?: string;
  kind?: "str" | "num";
  header?: boolean;
}

const dumpLines = computed<DumpLine[]>(() => {
  const a = activeAsset.value;
  if (!a) return [];
  const lines: DumpLine[] = [
    { depth: 0, key: `${a.type}:`, header: true },
    { depth: 1, key: "m_Name", value: a.name, kind: "str" },
    { depth: 1, key: "m_PathID", value: String(a.pathId), kind: "num" },
  ];
  const [w, h] = (a.dims ?? "0×0").split("×");
  if (a.type === "Sprite" || a.type === "Texture2D") {
    lines.push(
      { depth: 1, key: "m_Rect:" },
      { depth: 2, key: "x", value: "0", kind: "num" },
      { depth: 2, key: "y", value: "0", kind: "num" },
      { depth: 2, key: "width", value: w, kind: "num" },
      { depth: 2, key: "height", value: h, kind: "num" },
      { depth: 1, key: "m_TextureFormat", value: "ASTC_6x6", kind: "str" },
      { depth: 1, key: "m_MipCount", value: "1", kind: "num" },
      { depth: 1, key: "m_StreamData:" },
      { depth: 2, key: "offset", value: "0", kind: "num" },
      { depth: 2, key: "size", value: String(a.size), kind: "num" },
      { depth: 2, key: "path", value: "archive:/resS", kind: "str" },
    );
  } else if (a.type === "TextAsset") {
    lines.push(
      { depth: 1, key: "m_Script", value: a.container, kind: "str" },
      { depth: 1, key: "m_Size", value: String(a.size), kind: "num" },
    );
  } else if (a.type === "AudioClip") {
    lines.push(
      { depth: 1, key: "m_Length", value: "3.72", kind: "num" },
      { depth: 1, key: "m_Channels", value: "2", kind: "num" },
      { depth: 1, key: "m_Frequency", value: "44100", kind: "num" },
    );
  } else {
    lines.push(
      { depth: 1, key: "m_Container", value: a.container, kind: "str" },
      { depth: 1, key: "m_DataSize", value: String(a.size), kind: "num" },
    );
  }
  return lines;
});

const logLines = [
  { t: "00:00.00", msg: "serve 127.0.0.1:8080", accent: false },
  {
    t: "00:00.42",
    msg: `loaded ${bundle.name} — ${bundle.objects.toLocaleString()} objects`,
    accent: true,
  },
  {
    t: "00:04.17",
    msg: "decode Sprite char_002_amiya_1 (ASTC 6×6)",
    accent: false,
  },
  {
    t: "00:04.31",
    msg: "export char_002_amiya_1.png → /tmp/out",
    accent: true,
  },
];
</script>
