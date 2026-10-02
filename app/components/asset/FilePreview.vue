<template>
  <div
    class="h-full min-h-0 flex flex-col items-center justify-center gap-4 overflow-auto p-6"
  >
    <div
      v-if="loading"
      class="flex items-center gap-2 rounded bg-panel px-3 py-2 text-neutral-10/60"
    >
      <i-lucide-loader-circle class="h-4 w-4 animate-spin" />
      <span>Exporting {{ asset.type }}…</span>
    </div>

    <div
      v-else-if="error"
      class="max-w-md flex flex-col items-center gap-2 text-center text-danger-10"
    >
      <i-lucide-circle-alert class="h-8 w-8" />
      <span>{{ error }}</span>
    </div>

    <template v-else>
      <!-- 音频:播放器 + 多 sample 切换 -->
      <template v-if="asset.type === 'AudioClip'">
        <div
          v-if="sampleCount > 1"
          class="max-w-full flex flex-wrap items-center justify-center gap-1"
        >
          <button
            v-for="n in sampleCount"
            :key="n"
            class="rounded px-2 py-1 text-[12px] font-mono"
            :class="
              n - 1 === sampleIndex
                ? 'bg-accent-5 text-accent-contrast'
                : 'text-neutral-10/60 hover:bg-neutral-1 active:bg-neutral-2'
            "
            @click="emit('selectSample', n - 1)"
          >
            #{{ n - 1 }}
          </button>
        </div>
        <audio v-if="url" class="max-w-xl w-full" controls :src="url" />
      </template>

      <!-- 视频:内联播放 -->
      <video
        v-else-if="asset.type === 'VideoClip' || asset.type === 'MovieTexture'"
        class="max-h-full max-w-full"
        controls
        :src="url"
      />

      <!-- 文本:TextAsset 文本 / 任意类型的 JSON 兜底 dump -->
      <pre
        v-else-if="text !== null"
        class="max-h-full max-w-3xl w-full flex-1 overflow-auto whitespace-pre-wrap border border-neutral-5 rounded bg-panel p-4 text-[12.5px] text-neutral-10/90 leading-5 font-mono"
        >{{ text }}</pre>

      <!-- 字体:用 FontFace 渲染示例文本 -->
      <div
        v-else-if="asset.type === 'Font'"
        class="max-w-3xl w-full flex flex-col gap-3 text-neutral-10"
      >
        <p
          class="border-b border-neutral-5 pb-3 text-4xl"
          :style="fontReady ? { fontFamily: fontFamilyName } : {}"
        >
          The quick brown fox jumps over the lazy dog
        </p>
        <p
          class="text-2xl"
          :style="fontReady ? { fontFamily: fontFamilyName } : {}"
        >
          明日方舟 Arknights 0123456789
        </p>
      </div>

      <!-- 其余:图标 + 文件名 -->
      <div v-else class="flex flex-col items-center gap-3 text-neutral-10/60">
        <TypeIcon :type="asset.type" class="h-12 w-12" />
      </div>

      <div class="flex items-center gap-3 text-[12px] text-neutral-10/50">
        <span class="font-mono">{{ fileName }}</span>
        <button
          v-if="url || text !== null"
          class="inline-flex items-center gap-1 rounded px-2 py-1 text-neutral-10/70 active:bg-neutral-2 hover:bg-neutral-1"
          @click="emit('export')"
        >
          <i-lucide-download class="h-4 w-4" />
          Download
        </button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";
import TypeIcon from "./TypeIcon.vue";
import type { Asset } from "../../assets";

const props = withDefaults(
  defineProps<{
    asset: Asset;
    url?: string;
    text?: string | null;
    extension?: string;
    sampleCount?: number;
    sampleIndex?: number;
    loading?: boolean;
    error?: string;
  }>(),
  {
    url: "",
    text: null,
    extension: "",
    sampleCount: 1,
    sampleIndex: 0,
    loading: false,
    error: "",
  },
);

const emit = defineEmits<{ selectSample: [index: number]; export: [] }>();

const fileName = computed(
  () =>
    `${props.asset.name || "export"}${props.extension ? `.${props.extension}` : ""}`,
);

// Font 预览:把 blob URL 注册成 FontFace 再渲染示例文本。
const fontReady = ref(false);
const fontFamilyName = computed(() => `asset-font-${props.asset.pathId}`);
let activeFace: FontFace | null = null;

function releaseActiveFace() {
  // 换 url / 卸载时注销上一个 face,避免同名 family 重复注册堆积。
  if (activeFace) {
    document.fonts.delete(activeFace);
    activeFace = null;
  }
}

watch(
  () => props.url,
  (url) => {
    fontReady.value = false;
    releaseActiveFace();
    if (!url || props.asset.type !== "Font") return;
    const face = new FontFace(fontFamilyName.value, `url(${url})`);
    face
      .load()
      .then(() => {
        document.fonts.add(face);
        activeFace = face;
        fontReady.value = true;
      })
      .catch(() => {
        // 字体损坏/浏览器不支持:保持 fontReady=false,示例文本回退默认字体。
      });
  },
  { immediate: true },
);

onBeforeUnmount(releaseActiveFace);
</script>
