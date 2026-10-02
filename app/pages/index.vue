<template>
  <div
    class="h-screen flex flex-col select-none bg-background text-left text-[13px]"
  >
    <!-- ═══ Row 1: title bar ═══ -->
    <TitleBar
      :bundle-name="bundle.name"
      :can-export="!!activeAsset"
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
      @export-object="exportCurrentObject"
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
              title="Copy visible dump"
              :disabled="
                activeTab?.mode !== 'dump' ||
                dumpPreview.loading ||
                !!dumpPreview.error ||
                dumpLines.length === 0
              "
              @click="copyDump"
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
              v-if="
                activeAsset.type === 'Texture2D' ||
                activeAsset.type === 'Sprite'
              "
              :asset="activeAsset"
              class="min-h-0 flex-1"
              :error="texturePreview.error"
              :image-url="texturePreview.url"
              :loading="texturePreview.loading"
            />
            <FilePreview
              v-else
              :asset="activeAsset"
              class="min-h-0 flex-1"
              :error="filePreview.error"
              :extension="filePreview.extension"
              :loading="filePreview.loading"
              :sample-count="filePreview.sampleCount"
              :sample-index="filePreview.sampleIndex"
              :text="filePreview.text"
              :url="filePreview.url"
              @select-sample="selectSample"
              @export="exportCurrentObject"
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
              v-if="dumpPreview.loading"
              class="h-full w-full flex items-center justify-center gap-2 text-neutral-10/60"
            >
              <i-lucide-loader-circle class="h-4 w-4 animate-spin" />
              <span>Loading dump…</span>
            </div>
            <div
              v-else-if="dumpPreview.error"
              class="h-full w-full flex items-center justify-center gap-2 text-danger-10"
            >
              <i-lucide-circle-alert class="h-4 w-4" />
              <span>{{ dumpPreview.error }}</span>
            </div>
            <div
              v-else-if="dumpLines.length === 0"
              class="h-full w-full flex items-center justify-center text-neutral-10/40"
            >
              <span>No data</span>
            </div>
            <template v-else>
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
                  <template v-if="l.page">
                    <button
                      class="ml-2 rounded px-2 text-accent-10 hover:bg-accent-1 disabled:opacity-40"
                      :disabled="
                        dumpPageLoading.has(JSON.stringify(l.page.path)) ||
                        l.page.offset === 0
                      "
                      @click="
                        loadDumpPage(
                          l.page.path,
                          Math.max(0, l.page.offset - 256),
                        )
                      "
                    >
                      Previous
                    </button>
                    <button
                      v-if="l.page.next !== undefined"
                      class="ml-2 rounded px-2 text-accent-10 hover:bg-accent-1 disabled:opacity-40"
                      :disabled="
                        dumpPageLoading.has(JSON.stringify(l.page.path))
                      "
                      @click="loadDumpPage(l.page.path, l.page.next)"
                    >
                      {{
                        dumpPageLoading.has(JSON.stringify(l.page.path))
                          ? "Loading…"
                          : l.page.next === 0
                            ? "Load items"
                            : "Next"
                      }}
                    </button>
                    <span
                      v-if="dumpPageErrors.get(JSON.stringify(l.page.path))"
                      class="ml-2 text-danger-10"
                    >
                      {{ dumpPageErrors.get(JSON.stringify(l.page.path)) }}
                    </span>
                  </template>
                </div>
              </div>
            </template>
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

      <!-- 解析失败(object 级可下载原始字节 .bin) -->
      <PopoverRoot v-if="failures.length > 0" v-model:open="failuresOpen">
        <PopoverTrigger
          class="flex items-center gap-1 rounded px-1.5 text-danger-10 active:bg-neutral-2 hover:bg-neutral-1"
          title="Parse failures"
        >
          <i-lucide-triangle-alert class="h-4 w-4" />
          {{ failures.length }} failed
        </PopoverTrigger>
        <PopoverPortal>
          <PopoverContent
            align="start"
            side="top"
            :side-offset="6"
            class="max-h-80 w-[440px] overflow-auto border border-neutral-5 rounded bg-panel p-1.5 text-left shadow-lg"
          >
            <p
              class="px-2 py-1.5 text-[11px] text-neutral-10/50 tracking-wide uppercase"
            >
              Parse failures ({{ failures.length }})
            </p>
            <div
              v-for="(f, i) in failures"
              :key="i"
              class="flex items-start gap-2 border-t border-neutral-5 px-2 py-1.5 first:border-t-0"
            >
              <span
                class="mt-0.5 shrink-0 rounded bg-danger-2 px-1.5 text-[10px] text-danger-10 uppercase"
                >{{ f.kind }}</span
              >
              <div class="min-w-0 flex-1">
                <p class="truncate text-[11.5px] text-neutral-10 font-mono">
                  {{ f.class_name ?? f.name ?? "unknown"
                  }}<template v-if="f.path_id"> · {{ f.path_id }}</template>
                </p>
                <p
                  class="truncate text-[11px] text-neutral-10/50"
                  :title="f.error"
                >
                  {{ f.error }}
                </p>
              </div>
              <button
                v-if="f.kind === 'object' && (f.data_len ?? 0) > 0"
                class="mt-0.5 inline-flex shrink-0 items-center gap-1 rounded px-1.5 py-0.5 text-[11px] text-neutral-10/70 active:bg-neutral-2 hover:bg-neutral-1"
                title="Save raw bytes (.bin)"
                @click="saveFailureBytes(f)"
              >
                <i-lucide-download class="h-3.5 w-3.5" />
                .bin
              </button>
            </div>
          </PopoverContent>
        </PopoverPortal>
      </PopoverRoot>

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
import {
  PopoverContent,
  PopoverPortal,
  PopoverRoot,
  PopoverTrigger,
  SplitterGroup,
  SplitterPanel,
  SplitterResizeHandle,
} from "reka-ui";
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { api } from "../api/client";
import { formatSize } from "../assets";
import FilePreview from "../components/asset/FilePreview.vue";
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
  name: sourceCountLabel(currentProject.value?.sources.length ?? 0),
  objects: assets.value.length,
  version: unityVersion.value,
}));

function sourceCountLabel(count: number): string {
  if (count === 0) return "";
  return `${count} ${count === 1 ? "source" : "sources"}`;
}

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

// 预览/导出走 /export 端点:专用格式(AudioClip/TextAsset/Font/VideoClip/
// MovieTexture)之外的任意类型由 Rust 兜底 dump 成 JSON,故不再白名单过滤。
const filePreview = ref<{
  loading: boolean;
  url: string;
  text: string | null;
  extension: string;
  sampleCount: number;
  sampleIndex: number;
  error: string;
}>({
  loading: false,
  url: "",
  text: null,
  extension: "",
  sampleCount: 1,
  sampleIndex: 0,
  error: "",
});
let fileRequest = 0;

// dump 页签:?detail=full 的截断检查视图(typed 经 codegen ToValue,三路统一 JSON)。
const dumpPreview = ref({ loading: false, error: "" });
const dumpRoot = ref<unknown>(null);
const dumpShell = ref<Record<string, unknown> | null>(null);
const dumpPages = ref(new Map<string, unknown>());
const dumpPageLoading = ref(new Set<string>());
const dumpPageErrors = ref(new Map<string, string>());
let dumpRequest = 0;

watch(
  [activeAsset, () => environment.value?.revision, () => activeTab.value?.mode],
  ([asset, revision, mode]) => {
    void loadTexturePreview(asset, revision, mode);
    // sample 不进 watcher 依赖：切资产/环境/模式总是从 0 重载，
    // 避免上一个资产的 sample 索引泄漏到 sample 数更少的 AudioClip（422）。
    void loadFilePreview(asset, revision, mode);
    void loadDumpDetail(asset, revision, mode);
  },
  { immediate: true },
);

// 用户点击 sample 按钮：显式带索引重载（不经 watcher）。
function selectSample(index: number) {
  void loadFilePreview(
    activeAsset.value,
    environment.value?.revision,
    activeTab.value?.mode,
    index,
  );
}

async function loadTexturePreview(
  asset: Asset | null,
  revision?: number,
  mode?: Mode,
) {
  const request = ++previewRequest;
  revokeTexturePreview();
  texturePreview.value = { loading: false, url: "", error: "" };
  // Sprite 与 Texture2D 同走 /png 端点。
  if (
    !asset ||
    (asset.type !== "Texture2D" && asset.type !== "Sprite") ||
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
      error: toErrorMessage(error),
    };
  }
}

function revokeTexturePreview() {
  if (texturePreview.value.url) URL.revokeObjectURL(texturePreview.value.url);
}

async function loadFilePreview(
  asset: Asset | null,
  revision?: number,
  mode?: Mode,
  sample = 0,
) {
  const request = ++fileRequest;
  if (filePreview.value.url) URL.revokeObjectURL(filePreview.value.url);
  filePreview.value = {
    loading: false,
    url: "",
    text: null,
    extension: "",
    sampleCount: 1,
    sampleIndex: sample,
    error: "",
  };
  if (
    !asset ||
    revision === undefined ||
    mode !== "preview" ||
    asset.type === "Texture2D" ||
    asset.type === "Sprite"
  )
    return;

  filePreview.value.loading = true;
  const [collection] = asset.id.split(":");
  try {
    // JSON 预览只读取有界详情；完整导出由显式保存动作触发。
    if (
      !["AudioClip", "TextAsset", "Font", "VideoClip", "MovieTexture"].includes(
        asset.type,
      )
    ) {
      const result = await api.GET(
        "/environment/collections/{cid}/objects/{pid}",
        {
          params: {
            path: { cid: Number(collection), pid: asset.pathId },
            query: { revision, detail: "full" },
          },
        },
      );
      if (request !== fileRequest) return;
      if (result.error) throw new Error(apiError(result.error));
      const detail = result.data.object.detail;
      if (!detail) throw new Error("Rust 未返回 detail");
      filePreview.value = {
        loading: false,
        url: "",
        text: JSON.stringify(
          detail.mono_shell
            ? { ...detail.mono_shell, structure: detail.value }
            : detail.value,
          null,
          2,
        ),
        extension: "json",
        sampleCount: 1,
        sampleIndex: 0,
        error: "",
      };
      return;
    }
    const result = await api.GET(
      "/environment/collections/{cid}/objects/{pid}/export",
      {
        params: {
          path: { cid: Number(collection), pid: asset.pathId },
          query: { revision, sample },
        },
        parseAs: "blob",
      },
    );
    if (request !== fileRequest) return;
    if (result.error) throw new Error(apiError(result.error));

    const blob = result.data as Blob;
    const headers = result.response.headers;
    const extension = headers.get("x-export-extension") ?? "";
    const sampleCount = Number(headers.get("x-sample-count") ?? "1") || 1;
    // JSON 兜底 dump（任意类型）与 TextAsset 文本内联展示;其余二进制走播放器/blob。
    const text =
      extension === "json" ||
      (asset.type === "TextAsset" && extension === "txt")
        ? await blob.text()
        : null;
    if (request !== fileRequest) return;
    filePreview.value = {
      loading: false,
      url: URL.createObjectURL(blob),
      text,
      extension,
      sampleCount,
      sampleIndex: sample,
      error: "",
    };
  } catch (error) {
    if (request !== fileRequest) return;
    filePreview.value = {
      loading: false,
      url: "",
      text: null,
      extension: "",
      sampleCount: 1,
      sampleIndex: sample,
      error: toErrorMessage(error),
    };
  }
}

// ── 导出到磁盘(files:save 原生保存对话框) ──

// 统一取对象的导出字节:Texture2D/Sprite 走 /png,其余走 /export(可带 sample)。
async function fetchObjectBytes(
  asset: Asset,
  revision: number,
  sample = 0,
): Promise<{ data: Blob; extension: string }> {
  const [collection] = asset.id.split(":");
  const isImage = asset.type === "Texture2D" || asset.type === "Sprite";
  const result = await api.GET(
    isImage
      ? "/environment/collections/{cid}/objects/{pid}/png"
      : "/environment/collections/{cid}/objects/{pid}/export",
    {
      params: {
        path: { cid: Number(collection), pid: asset.pathId },
        query: { revision, sample: isImage ? undefined : sample },
      },
      parseAs: "blob",
    },
  );
  if (result.error) throw new Error(apiError(result.error));
  const extension = isImage
    ? "png"
    : (result.response.headers.get("x-export-extension") ?? "bin");
  return { data: result.data as Blob, extension };
}

// 菜单 Export Current Object… / Ctrl+E。用户取消对话框时静默返回。
async function exportCurrentObject() {
  const asset = activeAsset.value;
  const revision = environment.value?.revision;
  if (!asset || revision === undefined) return;
  try {
    const sample =
      asset.type === "AudioClip" ? filePreview.value.sampleIndex : 0;
    const { data, extension } = await fetchObjectBytes(asset, revision, sample);
    const save = await window.electron.invoke("files:save", {
      defaultName: `${asset.name || "export"}.${extension}`,
      data: await data.arrayBuffer(),
    });
    if (!save.ok) throw new Error(save.error.message);
    if (save.value) {
      logLine(`Exported ${asset.name} → ${save.value.path}`, true);
    }
  } catch (error) {
    logLine(toErrorMessage(error), false);
    bottomOpen.value = true;
  }
}

// ── 解析失败对象(状态栏 popover + Save .bin) ──

const logLines = ref<Array<{ t: string; msg: string; accent: boolean }>>([]);
const failures = ref<Array<components["schemas"]["FailureDto"]>>([]);
const failuresOpen = ref(false);

async function saveFailureBytes(failure: components["schemas"]["FailureDto"]) {
  // schema 可选字段为 `?: T | null`,同时挡 undefined 与 null。
  if (failure.collection == null || failure.path_id == null) return;
  try {
    const result = await api.GET(
      "/environment/collections/{cid}/objects/{pid}/raw",
      {
        params: {
          path: { cid: failure.collection, pid: failure.path_id },
          query: { revision: environment.value?.revision },
        },
        parseAs: "blob",
      },
    );
    if (result.error) throw new Error(apiError(result.error));
    const save = await window.electron.invoke("files:save", {
      defaultName: `${failure.class_name ?? "Unknown"}_${failure.path_id}.bin`,
      data: await (result.data as Blob).arrayBuffer(),
    });
    if (!save.ok) throw new Error(save.error.message);
    if (save.value) {
      logLine(`Saved raw bytes → ${save.value.path}`, true);
    }
  } catch (error) {
    logLine(toErrorMessage(error), false);
    bottomOpen.value = true;
  }
}

function logLine(msg: string, accent: boolean) {
  logLines.value.push({ t: new Date().toLocaleTimeString(), msg, accent });
}

// ── dump 页签(?detail=full 截断检查视图 → DumpLine 树) ──

async function loadDumpDetail(
  asset: Asset | null,
  revision?: number,
  mode?: Mode,
) {
  const request = ++dumpRequest;
  dumpPreview.value = { loading: false, error: "" };
  dumpRoot.value = null;
  dumpShell.value = null;
  dumpPages.value.clear();
  dumpPageLoading.value.clear();
  dumpPageErrors.value.clear();
  if (!asset || revision === undefined || mode !== "dump") return;

  dumpPreview.value.loading = true;
  const [collection] = asset.id.split(":");
  try {
    const result = await api.GET(
      "/environment/collections/{cid}/objects/{pid}",
      {
        params: {
          path: { cid: Number(collection), pid: asset.pathId },
          query: { revision, detail: "full" },
        },
      },
    );
    if (request !== dumpRequest) return;
    if (result.error) throw new Error(apiError(result.error));
    const detail = result.data.object.detail;
    if (!detail) throw new Error("Rust 未返回 detail");

    dumpRoot.value = detail.value;
    dumpShell.value = detail.mono_shell as unknown as Record<
      string,
      unknown
    > | null;
    dumpPreview.value = { loading: false, error: "" };
  } catch (error) {
    if (request !== dumpRequest) return;
    dumpPreview.value = {
      loading: false,
      error: toErrorMessage(error),
    };
  }
}

// 页数据替换原页；切对象或模式后忽略旧响应，不累计已翻过的页。
async function loadDumpPage(path: string[], offset: number) {
  const asset = activeAsset.value;
  const revision = environment.value?.revision;
  if (!asset || revision === undefined) return;
  const request = dumpRequest;
  const key = JSON.stringify(path);
  dumpPageLoading.value.add(key);
  dumpPageErrors.value.delete(key);
  try {
    const [collection] = asset.id.split(":");
    const result = await api.GET(
      "/environment/collections/{cid}/objects/{pid}/inspect",
      {
        params: {
          path: { cid: Number(collection), pid: asset.pathId },
          query: { revision, path: key, offset, limit: 256 },
        },
      },
    );
    if (request !== dumpRequest) return;
    if (result.error) throw new Error(apiError(result.error));
    for (const cached of dumpPages.value.keys()) {
      const child: string[] = JSON.parse(cached);
      if (child.length > path.length && path.every((p, i) => child[i] === p))
        dumpPages.value.delete(cached);
    }
    dumpPages.value.set(key, result.data.value);
  } catch (error) {
    if (request === dumpRequest)
      dumpPageErrors.value.set(key, toErrorMessage(error));
  } finally {
    if (request === dumpRequest) dumpPageLoading.value.delete(key);
  }
}

function flattenJsonEntries(
  value: Record<string, unknown>,
  depth: number,
  out: DumpLine[],
  path: string[] = [],
) {
  for (const [k, v] of Object.entries(value))
    flattenJsonValue(v, k, depth, out, [...path, k]);
}

function flattenJsonValue(
  value: unknown,
  key: string,
  depth: number,
  out: DumpLine[],
  path: string[] = [],
) {
  value = dumpPages.value.get(JSON.stringify(path)) ?? value;
  if (value !== null && typeof value === "object" && !Array.isArray(value)) {
    const page = value as Record<string, unknown>;
    if (
      ["array", "bytes", "string"].includes(String(page._kind)) &&
      typeof page.len === "number" &&
      typeof page.offset === "number"
    ) {
      const items = Array.isArray(page.items) ? page.items : [];
      const next =
        typeof page.next_offset === "number" ? page.next_offset : undefined;
      out.push({
        depth: Math.max(0, depth),
        key: `${key}: ${page._kind} [${page.len}]`,
        value:
          page._kind === "array"
            ? `${items.length} items at ${page.offset}`
            : String(page._bytes_hex ?? page.text ?? ""),
        kind: "str",
        page: { path, offset: page.offset, next },
      });
      for (const [i, v] of items.entries()) {
        const index = String(Number(page.offset) + i);
        flattenJsonValue(v, index, depth + 1, out, [...path, index]);
      }
      return;
    }
    if (depth >= 0) out.push({ depth, key: `${key}:` });
    flattenJsonEntries(page, depth + 1, out, path);
    return;
  }
  if (Array.isArray(value)) {
    if (depth >= 0) out.push({ depth, key: `${key}: [${value.length}]` });
    for (const [i, v] of value.entries())
      flattenJsonValue(v, String(i), depth + 1, out, [...path, String(i)]);
    return;
  }
  if (depth >= 0)
    out.push({
      depth,
      key,
      value: value === null ? "null" : String(value),
      kind: typeof value === "string" ? "str" : "num",
    });
}

const dumpLines = computed<DumpLine[]>(() => {
  if (dumpRoot.value === null) return [];
  const lines: DumpLine[] = [
    { depth: 0, key: `${activeAsset.value?.type}:`, header: true },
  ];
  if (dumpShell.value) flattenJsonEntries(dumpShell.value, 1, lines);
  flattenJsonValue(dumpRoot.value, "value", -1, lines);
  return lines;
});

async function copyDump() {
  const text = dumpLines.value
    .map(
      (line) =>
        `${"  ".repeat(line.depth)}${line.key}${line.value === undefined ? "" : ` ${line.value}`}`,
    )
    .join("\n");
  try {
    await navigator.clipboard.writeText(text);
    logLine("Copied visible dump", true);
  } catch (error) {
    logLine(toErrorMessage(error), false);
    bottomOpen.value = true;
  }
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
    { k: "Size", v: a.size === undefined ? "—" : formatSize(a.size) },
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
  page?: { path: string[]; offset: number; next?: number };
}

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
    loadError.value = toErrorMessage(error);
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
  failures.value = [];
  failuresOpen.value = false;
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
  if (key === "e") {
    event.preventDefault();
    void exportCurrentObject();
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
  fileRequest++;
  if (filePreview.value.url) URL.revokeObjectURL(filePreview.value.url);
  dumpRequest++;
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

  // 解析失败列表(状态栏警告段 + Save .bin 的数据源)。
  const failuresResult = await api.GET("/environment/failures", {
    params: { query: { revision: summary.revision } },
  });
  failures.value = failuresResult.data?.items ?? [];

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
    loadError.value = toErrorMessage(error);
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
    loadError.value = toErrorMessage(error);
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
    loadError.value = toErrorMessage(error);
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

function toErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
</script>
