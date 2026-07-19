import vue from "@vitejs/plugin-vue";
import UnoCSS from "unocss/vite";
import IconsResolver from "unplugin-icons/resolver";
import Icons from "unplugin-icons/vite";
import Components from "unplugin-vue-components/vite";
import { defineConfig } from "vite";
import { electronSimple } from "vite-plugin-electron/multi-env";
import VueRouter from "vue-router/vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    electronSimple({
      main: {
        input: "electron/main.ts",
      },
      preload: {
        input: "electron/preload.ts",
        options: {
          build: {
            rolldownOptions: {
              output: {
                entryFileNames: "[name].cjs",
              },
            },
          },
        },
      },
    }),
    UnoCSS(),
    VueRouter({
      routesFolder: "app/pages",
    }),
    vue(),
    Components({
      // 仅解析 unplugin-icons，本地组件仍按现有约定显式 import
      dirs: [],
      resolvers: [IconsResolver()],
      dts: true,
    }),
    Icons({
      autoInstall: false,
      compiler: "vue3",
      defaultClass: "icon",
    }),
  ],
});
