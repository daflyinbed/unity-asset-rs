<template>
  <component :is="icon" />
</template>

<script setup lang="ts">
import { computed } from "vue";
import IconBox from "~icons/lucide/box";
import IconFile from "~icons/lucide/file";
import IconFileCode from "~icons/lucide/file-code";
import IconFileText from "~icons/lucide/file-text";
import IconImage from "~icons/lucide/image";
import IconLayers from "~icons/lucide/layers";
import IconMove from "~icons/lucide/move";
import IconMusic from "~icons/lucide/music";
import IconPaintbrush from "~icons/lucide/paintbrush";
import IconPalette from "~icons/lucide/palette";
import IconPlay from "~icons/lucide/play";
import type { AssetType } from "../../mock/assets";
import type { Component } from "vue";

const props = defineProps<{ type: AssetType }>();

const MAP: Record<AssetType, Component> = {
  Texture2D: IconImage,
  Sprite: IconLayers,
  Material: IconPalette,
  Mesh: IconBox,
  MonoBehaviour: IconFileCode,
  RectTransform: IconMove,
  SpriteRenderer: IconPaintbrush,
  TextAsset: IconFileText,
  AudioClip: IconMusic,
  AnimatorController: IconPlay,
};

const icon = computed(() => MAP[props.type] ?? IconFile);
</script>
