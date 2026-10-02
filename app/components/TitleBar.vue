<template>
  <header
    class="app-drag-region h-10 flex shrink-0 items-center gap-2 border-b border-neutral-5 bg-titlebar px-2"
  >
    <DropdownMenuRoot>
      <DropdownMenuTrigger
        class="app-no-drag border-0 rounded bg-transparent p-1.5 text-neutral-10/60 outline-none active:bg-neutral-2 data-[state=open]:bg-neutral-2 focus-visible:bg-neutral-1 hover:bg-neutral-1 data-[state=open]:text-neutral-10 focus-visible:text-neutral-10"
        aria-label="Open application menu"
        data-testid="application-menu"
      >
        <i-lucide-menu class="h-4 w-4" />
      </DropdownMenuTrigger>
      <DropdownMenuPortal>
        <DropdownMenuContent
          class="titlebar-menu-content"
          :side-offset="6"
          align="start"
        >
          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="titlebar-menu-item">
              <span>File</span>
              <i-lucide-chevron-right class="titlebar-menu-chevron" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="titlebar-menu-content"
                :side-offset="5"
                :align-offset="-4"
              >
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  :disabled="loading"
                  @select="emit('openAssets')"
                >
                  <span>New Project…</span>
                  <span class="titlebar-menu-shortcut">Ctrl+N</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator class="titlebar-menu-separator" />
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  :disabled="loading"
                  @select="emit('newProject')"
                >
                  <span>Open Files or Folder as New Project…</span>
                  <span class="titlebar-menu-shortcut">Ctrl+O</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  :disabled="!projectName || loading"
                  @select="emit('addAssets')"
                >
                  <span>Add Files or Folder to Current Project…</span>
                  <span class="titlebar-menu-shortcut">Ctrl+Shift+O</span>
                </DropdownMenuItem>
                <DropdownMenuSub v-if="projects.length > 0">
                  <DropdownMenuSubTrigger class="titlebar-menu-item">
                    <span>Open Recent Project</span>
                    <i-lucide-chevron-right class="titlebar-menu-chevron" />
                  </DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent
                      class="titlebar-menu-content"
                      :side-offset="5"
                      :align-offset="-4"
                    >
                      <DropdownMenuItem
                        v-for="project in projects"
                        :key="project.id"
                        class="titlebar-menu-item"
                        :disabled="loading"
                        @select="emit('openProject', project.id)"
                      >
                        <span class="max-w-64 truncate">{{
                          project.name
                        }}</span>
                        <span class="titlebar-menu-shortcut"
                          >{{ project.sources.length }}
                          {{
                            project.sources.length === 1 ? "source" : "sources"
                          }}</span
                        >
                      </DropdownMenuItem>
                    </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>
                <DropdownMenuSub v-if="sources.length > 0">
                  <DropdownMenuSubTrigger class="titlebar-menu-item">
                    <span>Project Sources</span>
                    <i-lucide-chevron-right class="titlebar-menu-chevron" />
                  </DropdownMenuSubTrigger>
                  <DropdownMenuPortal>
                    <DropdownMenuSubContent
                      class="titlebar-menu-content"
                      :side-offset="5"
                      :align-offset="-4"
                    >
                      <DropdownMenuSub
                        v-for="source in sources"
                        :key="source.id"
                      >
                        <DropdownMenuSubTrigger class="titlebar-menu-item">
                          <i-lucide-folder v-if="source.kind === 'directory'" />
                          <i-lucide-file v-else />
                          <span class="max-w-56 truncate">{{
                            source.name
                          }}</span>
                          <i-lucide-chevron-right
                            class="titlebar-menu-chevron"
                          />
                        </DropdownMenuSubTrigger>
                        <DropdownMenuPortal>
                          <DropdownMenuSubContent
                            class="titlebar-menu-content"
                            :side-offset="5"
                            :align-offset="-4"
                          >
                            <DropdownMenuItem
                              class="titlebar-menu-item"
                              :disabled="loading"
                              @select="emit('refreshSource', source.id)"
                            >
                              Refresh Source
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              class="titlebar-menu-item text-danger-10"
                              :disabled="loading"
                              @select="emit('removeSource', source.id)"
                            >
                              Remove from Project
                            </DropdownMenuItem>
                          </DropdownMenuSubContent>
                        </DropdownMenuPortal>
                      </DropdownMenuSub>
                    </DropdownMenuSubContent>
                  </DropdownMenuPortal>
                </DropdownMenuSub>
                <DropdownMenuSeparator class="titlebar-menu-separator" />
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  :disabled="!canExport"
                  @select="emit('exportObject')"
                >
                  <span>Export Current Object…</span>
                  <span class="titlebar-menu-shortcut">Ctrl+E</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator class="titlebar-menu-separator" />
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="windowAction('close')"
                >
                  <span>Close Window</span>
                  <span class="titlebar-menu-shortcut">Alt+F4</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="titlebar-menu-item">
              <span>Edit</span>
              <i-lucide-chevron-right class="titlebar-menu-chevron" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="titlebar-menu-content"
                :side-offset="5"
                :align-offset="-4"
              >
                <DropdownMenuItem class="titlebar-menu-item" disabled>
                  <span>Undo</span>
                  <span class="titlebar-menu-shortcut">Ctrl+Z</span>
                </DropdownMenuItem>
                <DropdownMenuItem class="titlebar-menu-item" disabled>
                  <span>Redo</span>
                  <span class="titlebar-menu-shortcut">Ctrl+Shift+Z</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator class="titlebar-menu-separator" />
                <DropdownMenuItem class="titlebar-menu-item" disabled>
                  <span>Copy</span>
                  <span class="titlebar-menu-shortcut">Ctrl+C</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="titlebar-menu-item">
              <span>View</span>
              <i-lucide-chevron-right class="titlebar-menu-chevron" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="titlebar-menu-content"
                :side-offset="5"
                :align-offset="-4"
              >
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="emit('toggleBottom')"
                >
                  <span>Toggle Bottom Panel</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="emit('toggleRight')"
                >
                  <span>Toggle Object Drawer</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator class="titlebar-menu-separator" />
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="windowAction('open-devtools')"
                >
                  <span>Open Developer Tools</span>
                  <span class="titlebar-menu-shortcut">Ctrl+Shift+I</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="titlebar-menu-item">
              <span>Window</span>
              <i-lucide-chevron-right class="titlebar-menu-chevron" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="titlebar-menu-content"
                :side-offset="5"
                :align-offset="-4"
              >
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="windowAction('minimize')"
                >
                  <span>Minimize</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="windowAction('toggle-maximize')"
                >
                  <span>{{ maximized ? "Restore" : "Maximize" }}</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>

          <DropdownMenuSub>
            <DropdownMenuSubTrigger class="titlebar-menu-item">
              <span>Help</span>
              <i-lucide-chevron-right class="titlebar-menu-chevron" />
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                class="titlebar-menu-content"
                :side-offset="5"
                :align-offset="-4"
              >
                <DropdownMenuItem
                  class="titlebar-menu-item"
                  @select="aboutOpen = true"
                >
                  <span>About unity-asset-rs</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        </DropdownMenuContent>
      </DropdownMenuPortal>
    </DropdownMenuRoot>

    <span
      v-if="projectName"
      class="app-no-drag flex items-center gap-1.5 rounded-md px-2 py-1 text-[12px] text-neutral-10/70 active:bg-neutral-2 hover:bg-neutral-1"
    >
      <i-lucide-folder class="h-3.5 w-3.5 text-accent-10" />
      {{ projectName }}
    </span>
    <i-lucide-chevron-right
      v-if="projectName && bundleName"
      class="h-3 w-3 text-neutral-10/40"
    />
    <TooltipProvider v-if="bundleName" :delay-duration="250">
      <TooltipRoot>
        <TooltipTrigger as-child>
          <button
            class="app-no-drag flex items-center gap-1.5 border-0 rounded-md bg-transparent px-2 py-1 text-[12px] text-neutral-10/60 outline-none active:bg-neutral-2 focus-visible:bg-neutral-1 hover:bg-neutral-1"
            type="button"
            data-testid="source-tooltip-trigger"
          >
            <i-lucide-package class="h-3.5 w-3.5 text-accent-10" />
            {{ bundleName }}
          </button>
        </TooltipTrigger>
        <TooltipPortal>
          <TooltipContent
            class="z-70 max-w-128 min-w-80 border border-neutral-5 rounded-lg bg-panel p-3 text-left text-[12px] text-neutral-10 shadow-lg"
            :side-offset="7"
            align="start"
            data-testid="source-tooltip-content"
          >
            <p class="mb-2 text-neutral-10/40 font-medium">Loaded sources</p>
            <div class="space-y-2">
              <div
                v-for="source in loadedSources"
                :key="source.id"
                class="flex items-start gap-2"
              >
                <i-lucide-folder
                  v-if="source.kind === 'directory'"
                  class="mt-0.5 h-3.5 w-3.5 shrink-0 text-accent-10"
                />
                <i-lucide-file
                  v-else
                  class="mt-0.5 h-3.5 w-3.5 shrink-0 text-accent-10"
                />
                <div class="min-w-0">
                  <p class="text-neutral-10/30">
                    {{
                      source.kind === "directory"
                        ? "Directory · recursive"
                        : "File"
                    }}
                  </p>
                  <p class="break-all text-neutral-10 leading-5 font-mono">
                    {{ source.path }}
                  </p>
                  <template v-if="source.kind === 'directory'">
                    <p class="mb-1 mt-2 text-neutral-10/40">
                      Loaded files · {{ source.files.length }}
                    </p>
                    <div class="max-h-60 overflow-auto pr-1 space-y-1">
                      <p
                        v-for="path in source.files"
                        :key="path"
                        class="break-all text-neutral-10 leading-5 font-mono"
                      >
                        {{ path }}
                      </p>
                    </div>
                  </template>
                </div>
              </div>
            </div>
            <TooltipArrow class="fill-panel" />
          </TooltipContent>
        </TooltipPortal>
      </TooltipRoot>
    </TooltipProvider>
    <span
      v-if="unityVersion"
      class="app-no-drag flex items-center gap-1.5 rounded-md px-2 py-1 text-[12px] text-neutral-10/60 active:bg-neutral-2 hover:bg-neutral-1"
    >
      <i-lucide-git-branch class="h-3.5 w-3.5" />
      Unity {{ unityVersion }}
    </span>

    <div class="app-no-drag ml-auto h-full flex items-stretch">
      <button
        class="window-control"
        aria-label="Minimize window"
        data-testid="window-minimize"
        @click="windowAction('minimize')"
      >
        <i-lucide-minus class="h-4 w-4" />
      </button>
      <button
        class="window-control"
        :aria-label="maximized ? 'Restore window' : 'Maximize window'"
        data-testid="window-maximize"
        @click="windowAction('toggle-maximize')"
      >
        <i-lucide-copy v-if="maximized" class="h-3.5 w-3.5" />
        <i-lucide-square v-else class="h-3.5 w-3.5" />
      </button>
      <button
        class="window-control window-control-close"
        aria-label="Close window"
        data-testid="window-close"
        @click="windowAction('close')"
      >
        <i-lucide-x class="h-4 w-4" />
      </button>
    </div>
  </header>

  <DialogRoot v-model:open="aboutOpen">
    <DialogPortal>
      <DialogOverlay class="titlebar-dialog-overlay" />
      <DialogContent class="titlebar-dialog-content">
        <DialogTitle class="text-[16px] text-neutral-10 font-semibold">
          About unity-asset-rs
        </DialogTitle>
        <DialogDescription
          class="mt-2 text-[13px] text-neutral-10/70 leading-5"
        >
          A fast Unity AssetBundle explorer powered by Rust, Electron, and Vue.
        </DialogDescription>
        <div
          class="mt-4 rounded-md bg-background px-3 py-2 text-[12px] text-neutral-10/60"
        >
          {{ aboutSummary }}
        </div>
        <div class="mt-5 flex justify-end">
          <DialogClose
            class="rounded-md bg-accent-7 px-3 py-1.5 text-[12px] text-accent-contrast outline-none active:bg-accent-9 hover:bg-accent-8 focus-visible:ring-2 focus-visible:ring-accent-5"
          >
            Close
          </DialogClose>
        </div>
        <DialogClose
          class="absolute right-3 top-3 rounded p-1 text-neutral-10/50 outline-none active:bg-neutral-2 focus-visible:bg-neutral-1 hover:bg-neutral-1 hover:text-neutral-10"
          aria-label="Close dialog"
        >
          <i-lucide-x class="h-4 w-4" />
        </DialogClose>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>

<script setup lang="ts">
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogOverlay,
  DialogPortal,
  DialogRoot,
  DialogTitle,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuRoot,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
  TooltipArrow,
  TooltipContent,
  TooltipPortal,
  TooltipProvider,
  TooltipRoot,
  TooltipTrigger,
} from "reka-ui";
import { computed, ref } from "vue";
import type {
  ProjectSnapshot,
  ProjectSource,
  WindowControlAction,
} from "../../shared/ipc";

interface LoadedSource {
  files: string[];
  id: string;
  kind: "file" | "directory";
  path: string;
}

const props = defineProps<{
  bundleName: string;
  /** 是否有可导出的活动对象（无则 Export Current Object 禁用）。 */
  canExport: boolean;
  loading: boolean;
  loadedSources: LoadedSource[];
  projectName: string;
  projects: ProjectSnapshot[];
  sources: ProjectSource[];
  unityVersion: string;
}>();

const emit = defineEmits<{
  toggleBottom: [];
  toggleRight: [];
  openAssets: [];
  newProject: [];
  addAssets: [];
  openProject: [projectId: number];
  refreshSource: [sourceId: number];
  removeSource: [sourceId: number];
  exportObject: [];
}>();

const aboutOpen = ref(false);
const maximized = ref(false);

const aboutSummary = computed(() => {
  const parts: string[] = [];
  if (props.unityVersion) parts.push(`Unity ${props.unityVersion}`);
  if (props.bundleName) parts.push(props.bundleName);
  return parts.length > 0 ? parts.join(" · ") : "No bundle loaded";
});

async function windowAction(action: WindowControlAction) {
  const result = await window.electron.invoke("window:control", { action });
  if (result.ok) maximized.value = result.value.maximized;
  else console.error(result.error.message);
}
</script>

<style>
.titlebar-menu-content {
  box-shadow: 0 12px 32px
    color-mix(in srgb, var(--my-color-overlay) 32%, transparent);
  transform-origin: var(--reka-dropdown-menu-content-transform-origin);
}

.titlebar-menu-item[data-highlighted] .titlebar-menu-shortcut {
  color: var(--my-color-accent-10);
  opacity: 0.6;
}

.titlebar-menu-item[data-highlighted] .titlebar-menu-chevron {
  color: var(--my-color-accent-10);
}

.titlebar-dialog-content {
  box-shadow: 0 18px 60px
    color-mix(in srgb, var(--my-color-overlay) 48%, transparent);
}
</style>
