<template>
  <main
    class="min-h-0 flex flex-1 items-center justify-center bg-background p-8"
    @dragenter.prevent="dragging = true"
    @dragover.prevent="dragging = true"
    @dragleave="onDragLeave"
    @drop.prevent="onDrop"
  >
    <section
      class="max-w-160 w-full border-2 rounded-2xl border-dashed bg-panel px-8 py-10 transition-colors"
      :class="dragging ? 'border-accent-6 bg-accent-1' : 'border-neutral-5'"
      data-testid="open-assets-dropzone"
    >
      <div
        class="mx-auto h-14 w-14 flex items-center justify-center rounded-2xl bg-accent-2 text-accent-10"
      >
        <i-lucide-package-open class="h-7 w-7" />
      </div>
      <h1 class="mt-5 text-center text-[20px] text-neutral-10 font-semibold">
        Create a project from Unity assets
      </h1>
      <p
        class="mx-auto mt-2 max-w-100 text-center text-[13px] text-neutral-10/60 leading-5"
      >
        Choose or drop one or more AssetBundle files or folders. This creates a
        new project; folders are scanned recursively by the Rust loader.
      </p>

      <div v-if="sources.length > 0" class="mt-6 flex flex-col gap-1.5">
        <div
          v-for="source in sources"
          :key="source.path"
          class="flex items-center gap-3 border border-neutral-5 rounded-lg bg-surface px-3 py-2"
        >
          <i-lucide-folder
            v-if="source.kind === 'directory'"
            class="h-4 w-4 shrink-0 text-accent-10"
          />
          <i-lucide-file v-else class="h-4 w-4 shrink-0 text-accent-10" />
          <span class="min-w-0 flex-1">
            <span class="block truncate text-[13px] text-neutral-10">{{
              source.name
            }}</span>
            <span class="block truncate text-[11px] text-neutral-10/50">{{
              source.path
            }}</span>
          </span>
          <button
            class="h-7 w-7 flex shrink-0 items-center justify-center rounded-md text-neutral-10/50 outline-none active:bg-neutral-2 focus-visible:bg-neutral-1 hover:bg-neutral-1 hover:text-danger-10"
            :aria-label="`Remove ${source.name}`"
            :disabled="loading"
            @click="removeSource(source.path)"
          >
            <i-lucide-x class="h-4 w-4" />
          </button>
        </div>
      </div>

      <div
        class="mx-auto mt-5 max-w-100 flex items-center justify-between gap-5 rounded-lg bg-surface px-3 py-2.5"
      >
        <span class="min-w-0">
          <span id="arknights-label" class="block text-[13px] text-neutral-10"
            >Arknights</span
          >
          <span
            id="arknights-description"
            class="block text-[11px] text-neutral-10/50"
          >
            Decode compression flags 4 and 5 with the Arknights LZ4 variant.
          </span>
        </span>
        <SwitchRoot
          v-model="arknights"
          class="relative h-5 w-9 inline-flex shrink-0 cursor-pointer items-center border-0 rounded-full bg-neutral-4 p-0.5 outline-none transition-colors data-[state=checked]:bg-accent-7 focus-visible:ring-2 focus-visible:ring-accent-5"
          :disabled="loading"
          aria-labelledby="arknights-label"
          aria-describedby="arknights-description"
        >
          <SwitchThumb
            class="block h-4 w-4 rounded-full bg-neutral-contrast shadow-sm transition-transform data-[state=checked]:translate-x-4"
          />
        </SwitchRoot>
      </div>

      <div class="mt-6 flex items-center justify-center gap-2">
        <button
          class="inline-flex items-center gap-2 border border-neutral-5 rounded-lg bg-surface px-4 py-2.5 text-[13px] text-neutral-10 font-medium outline-none disabled:cursor-wait active:bg-neutral-2 hover:bg-neutral-1 focus-visible:ring-2 focus-visible:ring-accent-5"
          :disabled="loading || choosing"
          data-testid="choose-assets"
          @click="chooseSources"
        >
          <i-lucide-folder-open class="h-4 w-4" />
          {{ choosing ? "Choosing…" : "Add files or folders" }}
        </button>
        <button
          class="inline-flex items-center gap-2 border-0 rounded-lg bg-accent-7 px-4 py-2.5 text-[13px] text-accent-contrast font-medium outline-none disabled:cursor-not-allowed active:bg-accent-9 disabled:bg-neutral-2 hover:bg-accent-8 disabled:text-neutral-7 focus-visible:ring-2 focus-visible:ring-accent-5"
          :disabled="loading || sources.length === 0"
          data-testid="create-project"
          @click="createProject"
        >
          <i-lucide-plus class="h-4 w-4" />
          {{ loading ? "Creating project…" : "Create project" }}
        </button>
      </div>
      <p v-if="status" class="mt-4 text-center text-[12px] text-neutral-10/50">
        {{ status }}
      </p>
      <p
        v-if="localError || error"
        class="mt-4 text-center text-[12px] text-danger-10"
      >
        {{ localError || error }}
      </p>
    </section>
  </main>
</template>

<script setup lang="ts">
import { SwitchRoot, SwitchThumb } from "reka-ui";
import { ref } from "vue";
import type { SelectedPath } from "../../shared/ipc";

defineProps<{
  error: string;
  loading: boolean;
  status: string;
}>();

const emit = defineEmits<{
  create: [sources: SelectedPath[]];
}>();

const dragging = ref(false);
const choosing = ref(false);
const arknights = ref(false);
const localError = ref("");
const sources = ref<SelectedPath[]>([]);

function addSources(selected: SelectedPath[]) {
  const byPath = new Map(sources.value.map((source) => [source.path, source]));
  for (const source of selected) byPath.set(source.path, source);
  sources.value = [...byPath.values()];
}

function removeSource(path: string) {
  sources.value = sources.value.filter((source) => source.path !== path);
}

function createProject() {
  emit(
    "create",
    sources.value.map((source) => ({ ...source, arknights: arknights.value })),
  );
}

async function chooseSources() {
  choosing.value = true;
  localError.value = "";
  try {
    const result = await window.electron.invoke("files:pick", {});
    if (!result.ok) throw new Error(result.error.message);
    addSources(result.value);
  } catch (error) {
    localError.value = error instanceof Error ? error.message : String(error);
  } finally {
    choosing.value = false;
  }
}

function onDragLeave(event: DragEvent) {
  const current = event.currentTarget as HTMLElement;
  if (!event.relatedTarget || !current.contains(event.relatedTarget as Node)) {
    dragging.value = false;
  }
}

async function onDrop(event: DragEvent) {
  dragging.value = false;
  const paths = [...(event.dataTransfer?.files ?? [])]
    .map((file) => window.electron.getPathForFile(file))
    .filter(Boolean);
  if (paths.length === 0) return;

  localError.value = "";
  const result = await window.electron.invoke("files:inspect", { paths });
  if (result.ok) addSources(result.value);
  else localError.value = result.error.message;
}
</script>
