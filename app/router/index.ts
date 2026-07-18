import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "home",
    component: () => import("../pages/Home.vue"),
  },
  {
    path: "/color-palette",
    name: "color-palette",
    component: () => import("../pages/ColorPalette.vue"),
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
