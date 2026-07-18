import vue from "@vitejs/plugin-vue";
import UnoCSS from "unocss/vite";
import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";
import VueRouter from "vue-router/vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    UnoCSS(),
    VueRouter({
      routesFolder: "app/pages",
    }),
    vue(),
    Icons({
      autoInstall: false,
      compiler: "vue3",
      defaultClass: "icon",
    }),
  ],
});
