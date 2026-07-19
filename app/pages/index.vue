<template>
  <div
    class="h-screen flex flex-col select-none bg-background text-left text-[13px]"
  >
    <!-- ═══ Row 1: title bar ═══ -->
    <TitleBar
      :bundle-name="bundle.name"
      :loading="loading"
      :project-name="projectName"
      :projects="projects"
      :sources="currentProject?.sources ?? []"
      :loaded-sources="environment?.sources ?? []"
      :unity-version="bundle.version"
      @open-assets="resetWorkspace"
      @new-project="chooseAssets('new')"
      @add-assets="chooseAssets('add')"
      @open-project="openProject"
      @refresh-source="refreshSource"
      @remove-source="removeSource"
      @toggle-bottom="bottomOpen = !bottomOpen"
      @toggle-right="drawerOpen = !drawerOpen"
    />

    <OpenAssets
      v-if="assets.length === 0"
      :loading="loading"
      :status="loadStatus"
      :error="loadError"
      @create="loadSelected($event, 'new')"
    />

    <!-- ═══ Row 2: tabs + drawer ═══ -->
    <SplitterGroup
      v-else
      auto-save-id="workspace-right-drawer"
      class="min-h-0 flex-1"
      direction="horizontal"
      @dragover.prevent
      @drop.prevent="dropIntoCurrentProject"
    >
      <!-- Editor area -->
      <SplitterPanel
        id="workspace-editor"
        as="main"
        class="min-w-0 flex flex-col"
        :order="1"
      >
        <!-- Tab strip -->
        <EditorTabs
          :active-id="activeTabId"
          :tabs="editorTabs"
          @update:active-id="activeTabId = $event"
          @close="closeTab"
          @menu-close="closeTabs"
          @pin="pinTab"
        />

        <!-- Breadcrumb / content toolbar -->
        <div
          v-if="activeAsset"
          class="h-9 flex shrink-0 items-center gap-1.5 border-b border-neutral-5 bg-background px-3 text-[12px]"
        >
          <span class="min-w-0 truncate text-neutral-10/50">{{
            activeAsset.container
          }}</span>
          <i-lucide-chevron-right class="h-3 w-3 text-neutral-10/40" />
          <span class="text-neutral-10 font-medium">{{
            activeAsset.name
          }}</span>
          <div class="ml-auto flex items-center gap-0.5">
            <button
              class="inline-flex items-center justify-center rounded p-1.5"
              :class="
                activeTab?.mode === 'preview'
                  ? 'bg-accent-2 text-accent-10'
                  : 'text-neutral-10/50 hover:bg-neutral-1 active:bg-neutral-2'
              "
              title="Preview"
              @click="setMode('preview')"
            >
              <i-lucide-eye class="h-4 w-4" />
            </button>
            <button
              class="inline-flex items-center justify-center rounded p-1.5"
              :class="
                activeTab?.mode === 'dump'
                  ? 'bg-accent-2 text-accent-10'
                  : 'text-neutral-10/50 hover:bg-neutral-1 active:bg-neutral-2'
              "
              title="Dump"
              @click="setMode('dump')"
            >
              <i-lucide-braces class="h-4 w-4" />
            </button>
            <button
              class="inline-flex items-center justify-center rounded p-1.5 text-neutral-10/50 active:bg-neutral-2 hover:bg-neutral-1"
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
            class="h-full flex flex-col items-center justify-center gap-2 text-neutral-10/50"
          >
            <i-lucide-mouse-pointer-click class="h-8 w-8 text-neutral-10/40" />
            <p>Select an object from the drawer to open it</p>
            <p class="text-[12px] text-neutral-10/40">
              Toggle the drawer with the panel buttons in the status bar
            </p>
          </div>

          <!-- Preview -->
          <div
            v-else-if="activeTab?.mode === 'preview'"
            class="h-full flex flex-col"
          >
            <PreviewCanvas
              :asset="activeAsset"
              class="min-h-0 flex-1"
              :error="texturePreview.error"
              :image-url="texturePreview.url"
              :loading="texturePreview.loading"
            />
            <div
              class="grid grid-cols-4 shrink-0 border-t border-neutral-5 bg-panel divide-x divide-neutral-5"
            >
              <div v-for="row in metaRows" :key="row.k" class="px-4 py-3">
                <p
                  class="text-[11px] text-neutral-10/50 tracking-wide uppercase"
                >
                  {{ row.k }}
                </p>
                <p class="mt-0.5 truncate text-neutral-10 font-mono">
                  {{ row.v }}
                </p>
              </div>
            </div>
          </div>

          <!-- Dump -->
          <div v-else class="flex bg-panel text-[12.5px] leading-6 font-mono">
            <div
              class="shrink-0 select-none border-r border-neutral-5 bg-panel px-3 text-right text-neutral-10/40"
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
                      : 'text-neutral-10/80'
                  "
                  >{{ l.key }}</span
                >
                <span
                  v-if="l.value !== undefined"
                  :class="
                    l.kind === 'str' ? 'text-accent-10' : 'text-neutral-10/50'
                  "
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
          class="h-32 shrink-0 overflow-auto border-t border-neutral-5 bg-panel px-4 py-2 text-[12px] text-neutral-10/60 leading-5 font-mono"
        >
          <p v-for="(line, i) in logLines" :key="i">
            <span class="text-neutral-10/40">[{{ line.t }}]</span>
            <span :class="line.accent ? 'text-accent-10' : ''">{{
              ` ${line.msg}`
            }}</span>
          </p>
        </div>
      </SplitterPanel>

      <!-- Drawer (right dock) -->
      <SplitterResizeHandle
        v-if="drawerOpen"
        id="workspace-drawer-handle"
        class="relative w-px shrink-0 bg-neutral-5 outline-none after:absolute after:inset-y-0 after:w-2 data-[state=drag]:bg-accent-9 data-[state=hover]:bg-accent-8 focus-visible:bg-accent-8 after:-left-1"
      />
      <SplitterPanel
        v-if="drawerOpen"
        id="workspace-drawer"
        as="aside"
        class="min-w-0 flex flex-col bg-drawer"
        :default-size="320"
        :max-size="640"
        :min-size="240"
        :order="2"
        size-unit="px"
      >
        <DrawerHeader
          v-model:view="drawerView"
          v-model:query="drawerQuery"
          v-model:match-case="drawerMatchCase"
          v-model:use-regex="drawerUseRegex"
          v-model:type="typeFilter"
          :types="availableTypes"
        />
        <DrawerBody
          :view="drawerView"
          :assets="drawerAssets"
          :active-id="activeTab?.id ?? null"
          :tab-ids="tabIds"
          @menu-close="closeTabs"
          @open="openAsset"
        />
      </SplitterPanel>
    </SplitterGroup>

    <!-- ═══ Row 3: status bar ═══ -->
    <footer
      v-if="assets.length > 0"
      class="h-8 flex shrink-0 items-center gap-1 border-t border-neutral-5 bg-statusbar px-2 text-[12px] text-neutral-10/60"
    >
      <span class="flex items-center gap-1 px-1.5">
        <i-lucide-circle-check class="h-4 w-4 text-accent-10" />
        {{ bundle.objects.toLocaleString() }} objects
      </span>
      <span class="px-1.5 text-neutral-10/50"
        >{{ drawerAssets.length }} shown</span
      >

      <div class="ml-auto flex items-center gap-1">
        <template v-if="activeAsset">
          <span class="px-1.5 font-mono">{{ activeAsset.pathId }}</span>
          <span class="px-1.5">{{ activeAsset.type }}</span>
          <span class="px-1.5 text-neutral-10/40">·</span>
          <span class="px-1.5 capitalize">{{ activeTab?.mode }}</span>
          <span class="px-1.5 text-neutral-10/40">·</span>
        </template>
        <button
          class="rounded p-1.5"
          :class="
            bottomOpen
              ? 'text-accent-10'
              : 'text-neutral-10/60 hover:bg-neutral-1 hover:text-neutral-10 active:bg-neutral-2'
          "
          title="Toggle bottom panel"
          @click="bottomOpen = !bottomOpen"
        >
          <i-lucide-scroll-text class="h-4 w-4" />
        </button>
        <button
          class="rounded p-1.5"
          :class="
            drawerOpen
              ? 'text-accent-10'
              : 'text-neutral-10/60 hover:bg-neutral-1 hover:text-neutral-10 active:bg-neutral-2'
          "
          title="Toggle object drawer"
          @click="drawerOpen = !drawerOpen"
        >
          <i-lucide-list-tree class="h-4 w-4" />
        </button>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { SplitterGroup, SplitterPanel, SplitterResizeHandle } from "reka-ui";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { api } from "../api/client";
import { formatSize } from "../assets";
import PreviewCanvas from "../components/asset/PreviewCanvas.vue";
import DrawerBody from "../components/DrawerBody.vue";
import DrawerHeader from "../components/DrawerHeader.vue";
import EditorTabs from "../components/EditorTabs.vue";
import OpenAssets from "../components/OpenAssets.vue";
import TitleBar from "../components/TitleBar.vue";
import type { ProjectSnapshot, SelectedPath } from "../../shared/ipc";
import type { components } from "../api/schema";
import type { Asset } from "../assets";
import type { TabCloseAction } from "../components/AssetContextMenu.vue";

type Mode = "preview" | "dump";
interface OpenTab {
  id: string;
  mode: Mode;
  pinned: boolean;
}

type EnvironmentSummary = components["schemas"]["EnvironmentSummary"];

const currentProject = ref<ProjectSnapshot | null>(null);
const projects = ref<ProjectSnapshot[]>([]);
const environment = ref<EnvironmentSummary | null>(null);
const assets = ref<Asset[]>([]);
const loading = ref(false);
const loadStatus = ref("");
const loadError = ref("");
const projectName = computed(() => currentProject.value?.name ?? "");
const unityVersion = ref("");

const bundle = computed(() => ({
  name: currentProject.value
    ? `${currentProject.value.sources.length} ${currentProject.value.sources.length === 1 ? "source" : "sources"}`
    : "",
  objects: assets.value.length,
  version: unityVersion.value,
}));

const drawerOpen = ref(true);
const bottomOpen = ref(false);
const drawerView = ref<"table" | "tree">("table");
const drawerQuery = ref("");
const drawerMatchCase = ref(false);
const drawerUseRegex = ref(false);
const typeFilter = ref<string[]>([]);

const tabs = ref<OpenTab[]>([]);
const activeTabId = ref<string | null>(null);
const tabIds = computed(() => tabs.value.map((tab) => tab.id));

const activeTab = computed(
  () => tabs.value.find((t) => t.id === activeTabId.value) ?? null,
);
const activeAsset = computed(() =>
  activeTab.value ? assetOf(activeTab.value) : null,
);
const texturePreview = ref({ loading: false, url: "", error: "" });
let previewRequest = 0;

watch(
  [activeAsset, () => environment.value?.revision, () => activeTab.value?.mode],
  ([asset, revision, mode]) => void loadTexturePreview(asset, revision, mode),
  { immediate: true },
);

async function loadTexturePreview(
  asset: Asset | null,
  revision?: number,
  mode?: Mode,
) {
  const request = ++previewRequest;
  revokeTexturePreview();
  texturePreview.value = { loading: false, url: "", error: "" };
  if (
    !asset ||
    asset.type !== "Texture2D" ||
    revision === undefined ||
    mode !== "preview"
  )
    return;

  texturePreview.value.loading = true;
  const [collection] = asset.id.split(":");
  try {
    const result = await api.GET(
      "/environment/collections/{cid}/objects/{pid}/png",
      {
        params: {
          path: { cid: Number(collection), pid: asset.pathId },
          query: { revision },
        },
        parseAs: "blob",
      },
    );
    if (request !== previewRequest) return;

    if (result.error) throw new Error(apiError(result.error));
    texturePreview.value = {
      loading: false,
      url: URL.createObjectURL(result.data),
      error: "",
    };
  } catch (error) {
    if (request !== previewRequest) return;
    texturePreview.value = {
      loading: false,
      url: "",
      error: error instanceof Error ? error.message : String(error),
    };
  }
}

function revokeTexturePreview() {
  if (texturePreview.value.url) URL.revokeObjectURL(texturePreview.value.url);
}
const editorTabs = computed(() =>
  tabs.value.map((tab) => {
    const asset = assetOf(tab);
    return {
      ...tab,
      container: asset.container,
      name: asset.name,
      pathId: asset.pathId,
      type: asset.type,
    };
  }),
);

function assetOf(tab: OpenTab): Asset {
  return assets.value.find((a) => a.id === tab.id) ?? assets.value[0];
}

function openAsset(asset: Asset, pinned = false) {
  const existing = tabs.value.find((tab) => tab.id === asset.id);
  if (existing) {
    if (pinned) existing.pinned = true;
  } else {
    const previewIndex = tabs.value.findIndex((tab) => !tab.pinned);
    const tab = { id: asset.id, mode: "preview" as const, pinned };
    if (pinned || previewIndex === -1) tabs.value.push(tab);
    else tabs.value.splice(previewIndex, 1, tab);
  }
  activeTabId.value = asset.id;
}

function pinTab(id: string) {
  const tab = tabs.value.find((item) => item.id === id);
  if (tab) tab.pinned = true;
}

function closeTab(id: string) {
  const i = tabs.value.findIndex((t) => t.id === id);
  tabs.value.splice(i, 1);
  if (activeTabId.value === id) {
    activeTabId.value =
      tabs.value[Math.min(i, tabs.value.length - 1)]?.id ?? null;
  }
}

function closeTabs(id: string, action: TabCloseAction) {
  const index = tabs.value.findIndex((tab) => tab.id === id);
  if (action !== "all" && index === -1) return;

  const closing = new Set(
    tabs.value
      .filter((_, tabIndex) => {
        if (action === "all") return true;
        if (action === "close") return tabIndex === index;
        if (action === "others") return tabIndex !== index;
        if (action === "left") return tabIndex < index;
        return tabIndex > index;
      })
      .map((tab) => tab.id),
  );
  tabs.value = tabs.value.filter((tab) => !closing.has(tab.id));
  if (activeTabId.value && closing.has(activeTabId.value)) {
    activeTabId.value =
      tabs.value[Math.min(index, tabs.value.length - 1)]?.id ?? null;
  }
}

function setMode(mode: Mode) {
  if (activeTab.value) activeTab.value.mode = mode;
}

const drawerAssets = computed(() => {
  const query = drawerQuery.value;
  let pattern: RegExp | null = null;

  if (drawerUseRegex.value && query) {
    try {
      pattern = new RegExp(query, drawerMatchCase.value ? "" : "i");
    } catch {
      return [];
    }
  }

  return assets.value.filter((asset) => {
    if (typeFilter.value.length > 0 && !typeFilter.value.includes(asset.type))
      return false;

    const searchable = asset.name + asset.container;
    if (pattern) return pattern.test(searchable);
    if (drawerMatchCase.value) return searchable.includes(query);
    return searchable.toLowerCase().includes(query.toLowerCase());
  });
});

const availableTypes = computed(() =>
  [...new Set(assets.value.map((asset) => asset.type))].sort((a, b) =>
    a.localeCompare(b),
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

const logLines = ref<Array<{ t: string; msg: string; accent: boolean }>>([]);

type OpenMode = "new" | "add";

async function chooseAssets(mode: OpenMode) {
  const result = await window.electron.invoke("files:pick", {});
  if (!result.ok) {
    loadError.value = result.error.message;
    return;
  }
  await loadSelected(result.value, mode);
}

async function inspectDroppedPaths(paths: string[], mode: OpenMode) {
  const result = await window.electron.invoke("files:inspect", { paths });
  if (!result.ok) {
    loadError.value = result.error.message;
    return;
  }
  await loadSelected(result.value, mode);
}

async function loadSelected(selected: SelectedPath[], mode: OpenMode) {
  if (selected.length === 0 || loading.value) return;
  loading.value = true;
  loadError.value = "";
  try {
    if (mode === "new") {
      loadStatus.value = "Preparing new project…";
      resetWorkspace();
      const created = await window.electron.invoke("projects:create", {
        name: projectNameFor(selected),
        sources: selected,
      });
      if (!created.ok) throw new Error(created.error.message);
      currentProject.value = created.value;
    } else {
      if (!currentProject.value) throw new Error("No current project");
      loadStatus.value = `Adding ${selected.length} source${selected.length === 1 ? "" : "s"}…`;
      const added = await window.electron.invoke("projects:add-sources", {
        projectId: currentProject.value.id,
        sources: selected,
      });
      if (!added.ok) throw new Error(added.error.message);
      currentProject.value = added.value;
    }
    await refreshEnvironment();
    await refreshProjects();
    loadStatus.value = "";
  } catch (error) {
    loadError.value = error instanceof Error ? error.message : String(error);
    if (assets.value.length > 0) {
      logLines.value.push({
        t: new Date().toLocaleTimeString(),
        msg: loadError.value,
        accent: false,
      });
      bottomOpen.value = true;
    }
  } finally {
    loading.value = false;
  }
}

function dropIntoCurrentProject(event: DragEvent) {
  const paths = pathsFromDrop(event);
  if (paths.length > 0) void inspectDroppedPaths(paths, "add");
}

function pathsFromDrop(event: DragEvent) {
  return [...(event.dataTransfer?.files ?? [])]
    .map((file) => window.electron.getPathForFile(file))
    .filter(Boolean);
}

function resetWorkspace() {
  currentProject.value = null;
  environment.value = null;
  unityVersion.value = "";
  assets.value = [];
  tabs.value = [];
  activeTabId.value = null;
  drawerQuery.value = "";
  typeFilter.value = [];
  logLines.value = [];
  bottomOpen.value = false;
}

function projectNameFor(selected: SelectedPath[]) {
  if (selected.length > 1) {
    const first = selected[0].name.replace(/\.[^.]+$/, "");
    return `${first || "Unity Assets"} +${selected.length - 1}`;
  }
  const entry = selected[0];
  if (entry.kind === "directory") return entry.name || "Untitled Project";
  return entry.name.replace(/\.[^.]+$/, "") || entry.name || "Untitled Project";
}

function onOpenShortcut(event: KeyboardEvent) {
  if (!event.ctrlKey) return;
  const key = event.key.toLowerCase();
  if (key === "n") {
    event.preventDefault();
    if (!loading.value) resetWorkspace();
    return;
  }
  if (key !== "o") return;
  event.preventDefault();
  if (loading.value) return;
  void chooseAssets(event.shiftKey && projectName.value ? "add" : "new");
}

onMounted(() => {
  window.addEventListener("keydown", onOpenShortcut);
  void restoreLastProject();
});
onBeforeUnmount(() => {
  previewRequest++;
  revokeTexturePreview();
  window.removeEventListener("keydown", onOpenShortcut);
});

async function restoreLastProject() {
  const result = await window.electron.invoke("projects:list", {});
  if (!result.ok) {
    loadError.value = result.error.message;
    return;
  }
  projects.value = result.value;
  const lastProject = result.value[0];
  if (lastProject) await openProject(lastProject.id);
}

async function refreshEnvironment() {
  const summaryResult = await api.GET("/environment");
  const summary = summaryResult.data;
  if (!summary) throw new Error("Rust returned no environment summary");
  environment.value = summary;
  const collectionsResult = await api.GET("/environment/collections", {
    params: { query: { revision: summary.revision } },
  });
  if (collectionsResult.error) {
    throw new Error(apiError(collectionsResult.error));
  }
  const collections = collectionsResult.data.items;
  const containersResult = await api.GET("/environment/containers", {
    params: { query: { revision: summary.revision } },
  });
  const containerPaths = new Map<string, string>();
  for (const item of containersResult.data?.items ?? []) {
    if (item.asset) {
      containerPaths.set(
        objectKey(item.asset.collection, item.asset.path_id),
        item.path,
      );
    }
  }

  const collectionRows = await Promise.all(
    collections.map(async (collection) => {
      const objectsResult = await api.GET(
        "/environment/collections/{cid}/objects",
        {
          params: {
            path: { cid: collection.index },
            query: { revision: summary.revision },
          },
        },
      );
      if (objectsResult.error) throw new Error(apiError(objectsResult.error));
      return { collection, objects: objectsResult.data.items };
    }),
  );

  const nextAssets = collectionRows.flatMap(({ collection, objects }) =>
    objects.map((object) =>
      toAsset(
        collection,
        object,
        containerPaths.get(objectKey(collection.index, object.path_id)),
      ),
    ),
  );
  assets.value = nextAssets;
  unityVersion.value = collectionRows[0]?.collection.unity_version ?? "";
  reconcileTabs();
  logLines.value.push({
    t: new Date().toLocaleTimeString(),
    msg: `revision ${summary.revision} — ${summary.sources.length} sources, ${nextAssets.length.toLocaleString()} objects`,
    accent: true,
  });
}

function toAsset(
  collection: components["schemas"]["CollectionItem"],
  object: components["schemas"]["ObjectMeta"],
  container?: string,
): Asset {
  return {
    id: `${collection.index}:${object.path_id}`,
    name: object.name || `${object.class_name}#${object.path_id}`,
    container: container ?? collection.name_fixed ?? collection.name,
    type: object.class_name,
    pathId: object.path_id,
    size: 0,
  };
}

function reconcileTabs() {
  const ids = new Set(assets.value.map((asset) => asset.id));
  tabs.value = tabs.value.filter((tab) => ids.has(tab.id));
  if (activeTabId.value && !ids.has(activeTabId.value)) {
    activeTabId.value = tabs.value[0]?.id ?? null;
  }
}

async function refreshProjects() {
  const result = await window.electron.invoke("projects:list", {});
  if (result.ok) projects.value = result.value;
}

async function openProject(projectId: number) {
  if (loading.value) return;
  loading.value = true;
  loadError.value = "";
  loadStatus.value = "Opening project…";
  try {
    const result = await window.electron.invoke("projects:open", { projectId });
    if (!result.ok) throw new Error(result.error.message);
    resetWorkspace();
    currentProject.value = result.value;
    await refreshEnvironment();
    await refreshProjects();
  } catch (error) {
    loadError.value = error instanceof Error ? error.message : String(error);
  } finally {
    loading.value = false;
    loadStatus.value = "";
  }
}

async function refreshSource(sourceId: number) {
  if (!currentProject.value || loading.value) return;
  loading.value = true;
  loadStatus.value = "Refreshing source…";
  try {
    const result = await window.electron.invoke("projects:refresh-source", {
      projectId: currentProject.value.id,
      sourceId,
    });
    if (!result.ok) throw new Error(result.error.message);
    currentProject.value = result.value;
    await refreshEnvironment();
  } catch (error) {
    loadError.value = error instanceof Error ? error.message : String(error);
  } finally {
    loading.value = false;
    loadStatus.value = "";
  }
}

async function removeSource(sourceId: number) {
  if (!currentProject.value || loading.value) return;
  loading.value = true;
  loadStatus.value = "Removing source…";
  try {
    const result = await window.electron.invoke("projects:remove-source", {
      projectId: currentProject.value.id,
      sourceId,
    });
    if (!result.ok) throw new Error(result.error.message);
    currentProject.value = result.value;
    await refreshEnvironment();
    await refreshProjects();
  } catch (error) {
    loadError.value = error instanceof Error ? error.message : String(error);
  } finally {
    loading.value = false;
    loadStatus.value = "";
  }
}

function objectKey(collection: number, pathId: string) {
  return `${collection}:${pathId}`;
}

function apiError(error: unknown) {
  if (typeof error === "object" && error && "error" in error) {
    return String(error.error);
  }
  return String(error);
}
</script>
