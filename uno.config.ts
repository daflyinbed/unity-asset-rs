import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";

// 主题颜色语义：
//
// 独立背景色：
// - background：应用主背景
// - panel：卡片、表格、抽屉、popover、dropdown、dialog 等容器背景
// - surface：输入框、checkbox、select 等表单控件背景
// - overlay：dialog 等模态层的遮罩
//
// accent / neutral / danger 色组均提供 1..10 + contrast，编号表示固定的组件角色，
// 不能当作任意的明暗色阶使用：
// - 1：outline / ghost hover 背景
// - 2：outline / ghost active 背景；surface 背景；neutral disabled 背景
// - 3：surface hover 背景
// - 4：surface active 背景
// - 5：outline / ghost 默认边框
// - 6：outline / ghost hover 边框；neutral disabled 边框
// - 7：solid 默认背景；neutral disabled 文字
// - 8：solid hover 背景
// - 9：solid active 背景
// - 10：普通文字和图标颜色
// - contrast：solid 背景上的文字和图标颜色
//
// 弱化文字仍使用 *-10 并通过透明度表达，例如 text-neutral-10/60。
// titlebar / statusbar / tabbar / drawer 是桌面工作区 chrome 的专用层级背景色。

// 生成 { 1: 'var(--my-color-{group}-1)', ..., 10: 'var(--my-color-{group}-10)' }
function semanticColorGroup(group: "accent" | "danger" | "neutral") {
  return Object.fromEntries(
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((n) => [
      n,
      `var(--my-color-${group}-${n})`,
    ]),
  );
}

// 把动态拼接的 bg/text/border/fill 类名预生成,避免 UnoCSS 扫描器漏检
const COLORS = ["accent", "danger", "neutral"] as const;
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
  "bg-titlebar",
  "bg-statusbar",
  "bg-tabbar",
  "bg-drawer",
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
  shortcuts: {
    "titlebar-dialog-content":
      "fixed left-1/2 top-1/2 z-[61] w-[calc(100vw-2rem)] max-w-104 -translate-x-1/2 -translate-y-1/2 border border-neutral-5 rounded-xl bg-panel p-5",
    "titlebar-dialog-overlay": "fixed inset-0 z-60 bg-overlay",
    "titlebar-menu-chevron": "h-3 w-3 text-neutral-10/50",
    "titlebar-menu-content":
      "z-50 min-w-52 border border-neutral-5 rounded-lg bg-panel p-1 text-neutral-10",
    "titlebar-menu-item":
      "min-h-7 flex cursor-default items-center justify-between gap-4 rounded px-2 py-1 text-[13px] leading-5 outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-accent-3 data-[disabled]:text-neutral-7 data-[highlighted]:text-accent-10",
    "titlebar-menu-separator": "mx-1 my-1 h-px bg-neutral-5",
    "titlebar-menu-shortcut": "ml-6 text-[11px] text-neutral-10/50",
    "window-control":
      "w-10 flex items-center justify-center border-0 bg-transparent text-neutral-10/60 hover:bg-neutral-1 hover:text-neutral-10 active:bg-neutral-2",
    "window-control-close":
      "hover:bg-danger-1 hover:text-danger-10 active:bg-danger-2",
  },
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
      danger: {
        ...semanticColorGroup("danger"),
        contrast: "var(--my-color-danger-contrast)",
      },
      // 独立背景色
      // 例: bg-background, text-panel, border-surface
      background: "var(--my-color-background)",
      panel: "var(--my-color-panel)",
      surface: "var(--my-color-surface)",
      overlay: "var(--my-color-overlay)",
      titlebar: "var(--my-color-titlebar)",
      statusbar: "var(--my-color-statusbar)",
      tabbar: "var(--my-color-tabbar)",
      drawer: "var(--my-color-drawer)",
    },
  },
});
