import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Home from "@/client/views/home.vue";
import BrewLog from "@/client/views/brew-log.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home
  },
  {
    path: "/brew/<id>",
    name: "brew",
    component: BrewLog
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
