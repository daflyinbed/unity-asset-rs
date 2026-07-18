import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";

// 生成 { 1: 'var(--my-color-{group}-1)', ..., 10: 'var(--my-color-{group}-10)' }
function semanticColorGroup(group: "accent" | "neutral") {
  return Object.fromEntries(
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((n) => [
      n,
      `var(--my-color-${group}-${n})`,
    ]),
  );
}

// 把动态拼接的 bg/text/border/fill 类名预生成,避免 UnoCSS 扫描器漏检
const COLORS = ["accent", "neutral"] as const;
const VARIANTS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, "contrast"] as const;
const PROPS = ["bg", "text", "border", "fill"] as const;
const SAFELIST: string[] = [
  ...PROPS.flatMap((p) =>
    COLORS.flatMap((c) => VARIANTS.map((v) => `${p}-${c}-${v}`)),
  ),
  "bg-background",
  "bg-panel",
  "bg-surface",
  "bg-overlay",
  "text-panel",
  "text-surface",
  "text-overlay",
  "border-panel",
  "border-surface",
  "border-overlay",
];

// https://unocss.dev/guide/config-file
export default defineConfig({
  presets: [
    presetUno(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      warn: true,
    }),
  ],
  safelist: SAFELIST,
  theme: {
    colors: {
      // 语义色组: 1..10 + contrast
      // 自动支持 bg-/text-/border-/fill- 等
      // 例: bg-accent-1, text-accent-10, border-accent-5, bg-accent-contrast
      accent: {
        ...semanticColorGroup("accent"),
        contrast: "var(--my-color-accent-contrast)",
      },
      neutral: {
        ...semanticColorGroup("neutral"),
        contrast: "var(--my-color-neutral-contrast)",
      },
      // 独立背景色
      // 例: bg-background, text-panel, border-surface
      background: "var(--my-color-background)",
      panel: "var(--my-color-panel)",
      surface: "var(--my-color-surface)",
      overlay: "var(--my-color-overlay)",
    },
  },
});
