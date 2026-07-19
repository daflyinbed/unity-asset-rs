export type AssetType =
  | "Texture2D"
  | "Sprite"
  | "Material"
  | "Mesh"
  | "MonoBehaviour"
  | "RectTransform"
  | "SpriteRenderer"
  | "TextAsset"
  | "AudioClip"
  | "AnimatorController";

export interface MockAsset {
  id: number;
  name: string;
  container: string;
  type: AssetType;
  pathId: number;
  size: number;
  dims?: string;
}

export interface MockBundle {
  name: string;
  path: string;
  objects: number;
  size: number;
  version: string;
  platform: string;
}

export const MOCK_BUNDLES: MockBundle[] = [
  {
    name: "char_002_amiya.ab",
    path: "example/chararts/char_002_amiya.ab",
    objects: 2847,
    size: 48_234_168,
    version: "2021.3.39f1",
    platform: "Android",
  },
  {
    name: "DownloadCommonUI.unity3d",
    path: "example/DownloadCommonUI.unity3d",
    objects: 1204,
    size: 12_884_992,
    version: "2021.3.39f1",
    platform: "Android",
  },
  {
    name: "battle_effects.ab",
    path: "example/effects/battle_effects.ab",
    objects: 693,
    size: 96_411_648,
    version: "2021.3.39f1",
    platform: "iOS",
  },
];

export const MOCK_ASSETS: MockAsset[] = [
  {
    id: 1,
    name: "char_002_amiya_1+",
    container: "dyn/art/char_002_amiya_1+.png",
    type: "Sprite",
    pathId: 6121,
    size: 20_971_520,
    dims: "2048×2048",
  },
  {
    id: 2,
    name: "char_002_amiya_1",
    container: "dyn/art/char_002_amiya_1.png",
    type: "Sprite",
    pathId: -4312,
    size: 7_968_440,
    dims: "646×1305",
  },
  {
    id: 3,
    name: "char_002_amiya_2b",
    container: "dyn/art/char_002_amiya_2b.png",
    type: "Sprite",
    pathId: 2926,
    size: 37_748_736,
    dims: "2048×2048",
  },
  {
    id: 4,
    name: "char_002_amiya_2",
    container: "dyn/art/char_002_amiya_2.png",
    type: "Sprite",
    pathId: -4801,
    size: 94_371_840,
    dims: "4096×4096",
  },
  {
    id: 5,
    name: "char_002_amiya_Atlas",
    container: "dyn/battle/char_002_amiya_Atlas.asset",
    type: "MonoBehaviour",
    pathId: -1704,
    size: 80,
  },
  {
    id: 6,
    name: "char_002_amiya_SkeletonData",
    container: "dyn/battle/char_002_amiya_SkeletonData.asset",
    type: "MonoBehaviour",
    pathId: 1924,
    size: 148,
  },
  {
    id: 7,
    name: "MonoBehaviour#73",
    container: "dyn/art/mat_amiya_1.mat",
    type: "MonoBehaviour",
    pathId: 3168,
    size: 124,
  },
  {
    id: 8,
    name: "MonoBehaviour#55",
    container: "dyn/art/mat_amiya_skin.mat",
    type: "MonoBehaviour",
    pathId: 1505,
    size: 56,
  },
  {
    id: 9,
    name: "RectTransform#100",
    container: "dyn/art/ui/root.canvas",
    type: "RectTransform",
    pathId: 8971,
    size: 108,
  },
  {
    id: 10,
    name: "RectTransform#13",
    container: "dyn/art/ui/panel.rect",
    type: "RectTransform",
    pathId: -7331,
    size: 108,
  },
  {
    id: 11,
    name: "SpriteRenderer#82",
    container: "dyn/build/prefab/amiya.prefab",
    type: "SpriteRenderer",
    pathId: 4172,
    size: 196,
  },
  {
    id: 12,
    name: "char_002_amiya.atlas",
    container: "dyn/battle/char_002_amiya.atlas.txt",
    type: "TextAsset",
    pathId: -2371,
    size: 5_646_336,
  },
  {
    id: 13,
    name: "char_002_amiya.skel",
    container: "dyn/battle/char_002_amiya.skel.bytes",
    type: "TextAsset",
    pathId: 7861,
    size: 17_301_504,
  },
  {
    id: 14,
    name: "build_char_002_amiya",
    container: "dyn/build/build_char_002_amiya.json",
    type: "TextAsset",
    pathId: 5575,
    size: 82_432,
  },
  {
    id: 15,
    name: "amiya_idle",
    container: "dyn/anim/amiya_idle.controller",
    type: "AnimatorController",
    pathId: 2210,
    size: 12_288,
  },
  {
    id: 16,
    name: "mat_amiya_1",
    container: "dyn/art/mat_amiya_1.mat",
    type: "Material",
    pathId: 3355,
    size: 2_048,
  },
  {
    id: 17,
    name: "tex_amiya_face",
    container: "dyn/art/tex_amiya_face.png",
    type: "Texture2D",
    pathId: 6640,
    size: 1_048_576,
    dims: "512×512",
  },
  {
    id: 18,
    name: "mesh_amiya_body",
    container: "dyn/mesh/amiya_body.mesh",
    type: "Mesh",
    pathId: 9012,
    size: 3_145_728,
  },
  {
    id: 19,
    name: "voice_amiya_greet",
    container: "dyn/audio/voice_amiya_greet.ogg",
    type: "AudioClip",
    pathId: 1102,
    size: 892_928,
  },
  {
    id: 20,
    name: "MonoBehaviour#58",
    container: "dyn/battle/amiya_skill.asset",
    type: "MonoBehaviour",
    pathId: 1595,
    size: 40,
  },
];

export function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function countByType(
  assets: MockAsset[],
): Array<{ type: AssetType; count: number }> {
  const map = new Map<AssetType, number>();
  for (const a of assets) map.set(a.type, (map.get(a.type) ?? 0) + 1);
  return [...map.entries()]
    .map(([type, count]) => ({ type, count }))
    .sort((x, y) => y.count - x.count);
}
