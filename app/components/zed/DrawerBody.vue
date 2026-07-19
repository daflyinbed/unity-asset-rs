<template>
  <div class="min-h-0 flex-1 overflow-auto py-1">
    <!-- Table view -->
    <template v-if="view === 'table'">
      <button
        v-for="a in assets"
        :key="a.id"
        class="w-full flex items-center gap-2 px-3 py-1.5 text-left"
        :class="
          a.id === activeId
            ? 'bg-accent-2 text-neutral-10'
            : 'text-neutral-7 hover:bg-neutral-2'
        "
        @click="$emit('open', a)"
      >
        <TypeIcon :type="a.type" class="h-3.5 w-3.5 shrink-0 text-accent-6" />
        <span class="min-w-0 flex-1">
          <span class="block truncate">{{ a.name }}</span>
          <span class="block truncate text-[11px] text-neutral-5">{{
            a.container
          }}</span>
        </span>
        <span class="shrink-0 text-[11px] text-neutral-5 font-mono">{{
          formatSize(a.size)
        }}</span>
      </button>
    </template>

    <!-- Container path tree view -->
    <template v-else>
      <template v-for="n in flatTree" :key="n.key">
        <button
          v-if="n.isDir"
          class="w-full flex items-center gap-1.5 px-3 py-1 text-left text-neutral-7 hover:bg-neutral-2"
          :style="{ paddingLeft: `${12 + n.depth * 16}px` }"
          @click="toggle(n.key)"
        >
          <i-lucide-chevron-down
            class="h-3.5 w-3.5 shrink-0 text-neutral-5 transition-transform"
            :class="collapsed.has(n.key) ? '-rotate-90' : ''"
          />
          <i-lucide-folder class="h-3.5 w-3.5 shrink-0 text-neutral-5" />
          <span class="truncate font-medium">{{ n.name }}</span>
          <span class="ml-auto text-[11px] text-neutral-4">{{ n.count }}</span>
        </button>
        <button
          v-else
          class="w-full flex items-center gap-1.5 px-3 py-1 text-left"
          :style="{ paddingLeft: `${12 + n.depth * 16 + 18}px` }"
          :class="
            n.asset && n.asset.id === activeId
              ? 'bg-accent-2 text-neutral-10'
              : 'text-neutral-7 hover:bg-neutral-2'
          "
          @click="n.asset && $emit('open', n.asset)"
        >
          <TypeIcon
            v-if="n.asset"
            :type="n.asset.type"
            class="h-3.5 w-3.5 shrink-0 text-accent-6"
          />
          <span class="min-w-0 flex-1 truncate">{{ n.name }}</span>
          <span
            v-if="n.asset"
            class="shrink-0 text-[11px] text-neutral-5 font-mono"
            >{{ formatSize(n.asset.size) }}</span
          >
        </button>
      </template>
    </template>

    <p
      v-if="assets.length === 0"
      class="px-3 py-8 text-center text-[12px] text-neutral-5"
    >
      No objects match the filter
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { formatSize } from "../../mock/assets";
import TypeIcon from "../asset/TypeIcon.vue";
import type { MockAsset } from "../../mock/assets";

const props = defineProps<{
  view: "table" | "tree";
  assets: MockAsset[];
  activeId: number | null;
}>();

defineEmits<{ open: [a: MockAsset] }>();

interface TreeDir {
  name: string;
  path: string;
  dirs: Map<string, TreeDir>;
  files: MockAsset[];
  count: number;
}

function newDir(name: string, path: string): TreeDir {
  return { name, path, dirs: new Map(), files: [], count: 0 };
}

const root = computed(() => {
  const r = newDir("", "");
  for (const a of props.assets) {
    const segs = a.container.split("/");
    let dir = r;
    let path = "";
    for (const seg of segs.slice(0, -1)) {
      path = path ? `${path}/${seg}` : seg;
      if (!dir.dirs.has(seg)) dir.dirs.set(seg, newDir(seg, path));
      dir = dir.dirs.get(seg)!;
      dir.count++;
    }
    dir.files.push(a);
  }
  return r;
});

interface FlatNode {
  key: string;
  name: string;
  depth: number;
  isDir: boolean;
  count?: number;
  asset?: MockAsset;
}

const collapsed = ref(new Set<string>());

function toggle(key: string) {
  const s = new Set(collapsed.value);
  if (s.has(key)) s.delete(key);
  else s.add(key);
  collapsed.value = s;
}

const flatTree = computed<FlatNode[]>(() => {
  const out: FlatNode[] = [];
  const walk = (dir: TreeDir, depth: number) => {
    for (const sub of [...dir.dirs.values()].sort((a, b) =>
      a.name.localeCompare(b.name),
    )) {
      out.push({
        key: sub.path,
        name: sub.name,
        depth,
        isDir: true,
        count: sub.count + sub.files.length,
      });
      if (!collapsed.value.has(sub.path)) walk(sub, depth + 1);
    }
    for (const f of [...dir.files].sort((a, b) =>
      a.name.localeCompare(b.name),
    )) {
      out.push({
        key: `${dir.path}/${f.name}#${f.id}`,
        name: f.name,
        depth,
        isDir: false,
        asset: f,
      });
    }
  };
  walk(root.value, 0);
  return out;
});
</script>
