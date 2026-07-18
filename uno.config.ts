import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
} from "unocss";

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
});
