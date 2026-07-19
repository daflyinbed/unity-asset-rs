<template>
  <div class="min-h-0 flex-1 overflow-auto py-1">
    <!-- Table view -->
    <template v-if="view === 'table'">
      <AssetContextMenu
        v-for="a in assets"
        :key="a.id"
        :asset="a"
        :tab-ids="tabIds"
        @close="$emit('menuClose', a.id, $event)"
      >
        <button
          class="w-full flex items-center gap-2 px-3 py-1.5 text-left"
          :class="
            a.id === activeId
              ? 'bg-accent-2 text-accent-10'
              : 'text-neutral-10/70 hover:bg-neutral-1 active:bg-neutral-2'
          "
          @click="$emit('open', a, false)"
          @dblclick="$emit('open', a, true)"
        >
          <TypeIcon
            :type="a.type"
            class="h-3.5 w-3.5 shrink-0 text-accent-10"
          />
          <span class="min-w-0 flex-1">
            <span class="block truncate">{{ a.name }}</span>
            <span
              class="block truncate text-[11px] text-neutral-10 font-light"
              >{{ a.container }}</span
            >
          </span>
          <span
            class="max-w-24 shrink-0 truncate text-[11px] text-neutral-10/50"
          >
            {{ a.type }}
          </span>
        </button>
      </AssetContextMenu>
    </template>

    <!-- Container path tree view -->
    <TreeRoot
      v-else
      v-slot="{ flattenItems }"
      v-model:expanded="expanded"
      :items="treeItems"
      :get-key="(node) => node.key"
      class="m-0 list-none p-0 outline-none"
    >
      <TreeItem
        v-for="item in flattenItems"
        :key="item._id"
        v-bind="item.bind"
        v-slot="{ isExpanded }"
        as="li"
        class="list-none outline-none"
        @select="preventTreeSelection"
      >
        <button
          v-if="item.value.kind === 'dir'"
          class="w-full flex items-center gap-1.5 py-1 pr-3 text-left text-neutral-10 outline-none active:bg-neutral-2 focus-visible:bg-neutral-1 hover:bg-neutral-1"
          :style="{ paddingLeft: `${12 + (item.level - 1) * 16}px` }"
        >
          <i-lucide-chevron-down
            class="h-3.5 w-3.5 shrink-0 text-neutral-10/50 transition-transform"
            :class="isExpanded ? '' : '-rotate-90'"
          />
          <i-lucide-folder class="h-3.5 w-3.5 shrink-0 text-neutral-10/50" />
          <span class="truncate">{{ item.value.name }}</span>
          <span class="ml-auto text-[11px] text-neutral-10/40">
            {{ item.value.count }}
          </span>
        </button>
        <AssetContextMenu
          v-else
          :asset="item.value.asset"
          :tab-ids="tabIds"
          @close="$emit('menuClose', item.value.asset.id, $event)"
        >
          <button
            class="w-full flex items-center gap-1.5 py-1 pr-3 text-left outline-none focus-visible:bg-neutral-1"
            :style="{ paddingLeft: `${12 + (item.level - 1) * 16 + 18}px` }"
            :class="
              item.value.asset.id === activeId
                ? 'bg-accent-2 text-accent-10'
                : 'text-neutral-10 hover:bg-neutral-1 active:bg-neutral-2'
            "
            @click="$emit('open', item.value.asset, false)"
            @dblclick="$emit('open', item.value.asset, true)"
          >
            <TypeIcon
              :type="item.value.asset.type"
              class="h-3.5 w-3.5 shrink-0 text-accent-10"
            />
            <span class="min-w-0 flex-1 truncate">{{ item.value.name }}</span>
            <span
              class="max-w-24 shrink-0 truncate text-[11px] text-neutral-10/50"
            >
              {{ item.value.asset.type }}
            </span>
          </button>
        </AssetContextMenu>
      </TreeItem>
    </TreeRoot>

    <p
      v-if="assets.length === 0"
      class="px-3 py-8 text-center text-[12px] text-neutral-10/50"
    >
      No objects match the filter
    </p>
  </div>
</template>

<script setup lang="ts">
import { TreeItem, TreeRoot } from "reka-ui";
import { computed, ref, watch } from "vue";
import TypeIcon from "./asset/TypeIcon.vue";
import AssetContextMenu from "./AssetContextMenu.vue";
import type { Asset } from "../assets";
import type { TabCloseAction } from "./AssetContextMenu.vue";

const props = defineProps<{
  view: "table" | "tree";
  assets: Asset[];
  activeId: string | null;
  tabIds: string[];
}>();

defineEmits<{
  menuClose: [id: string, action: TabCloseAction];
  open: [asset: Asset, pinned: boolean];
}>();

interface TreeDir {
  name: string;
  path: string;
  dirs: Map<string, TreeDir>;
  files: Asset[];
  count: number;
}

function newDir(name: string, path: string): TreeDir {
  return { name, path, dirs: new Map(), files: [], count: 0 };
}

const root = computed(() => {
  const r = newDir("", "");
  for (const a of props.assets) {
    const segs = a.container.split("/").filter(Boolean);
    let dir = r;
    let path = "";
    for (const seg of segs) {
      path = path ? `${path}/${seg}` : seg;
      if (!dir.dirs.has(seg)) dir.dirs.set(seg, newDir(seg, path));
      dir = dir.dirs.get(seg)!;
      dir.count++;
    }
    dir.files.push(a);
  }
  return r;
});

interface DirectoryNode {
  key: string;
  kind: "dir";
  name: string;
  count: number;
  children: TreeNode[];
}

interface AssetNode {
  key: string;
  kind: "asset";
  name: string;
  asset: Asset;
}

type TreeNode = DirectoryNode | AssetNode;

function compactDirectory(start: TreeDir): DirectoryNode {
  const names = [start.name];
  let dir = start;

  while (dir.files.length === 0 && dir.dirs.size === 1) {
    dir = [...dir.dirs.values()][0]!;
    names.push(dir.name);
  }

  return {
    key: dir.path,
    kind: "dir",
    name: names.join(" / "),
    count: dir.count + dir.files.length,
    children: directoryChildren(dir),
  };
}

function directoryChildren(dir: TreeDir): TreeNode[] {
  const directories = [...dir.dirs.values()]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(compactDirectory);
  const assets = [...dir.files]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map<AssetNode>((asset) => ({
      key: `${dir.path}/${asset.name}#${asset.id}`,
      kind: "asset",
      name: asset.name,
      asset,
    }));
  return [...directories, ...assets];
}

const treeItems = computed<TreeNode[]>(() => directoryChildren(root.value));
const expanded = ref<string[]>([]);
const knownDirectoryKeys = new Set<string>();

watch(
  treeItems,
  (items) => {
    const currentlyExpanded = new Set(expanded.value);
    const keys: string[] = [];
    const collect = (nodes: TreeNode[]) => {
      for (const node of nodes) {
        if (node.kind === "dir") {
          keys.push(node.key);
          collect(node.children);
        }
      }
    };
    collect(items);
    expanded.value = keys.filter(
      (key) => currentlyExpanded.has(key) || !knownDirectoryKeys.has(key),
    );
    for (const key of keys) knownDirectoryKeys.add(key);
  },
  { immediate: true },
);

function preventTreeSelection(event: Event) {
  event.preventDefault();
}
</script>
