import BrewLog from "/@client/views/brew-log.vue";
import Home from "/@client/views/home.vue";
import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/brew/:brewId",
    name: "brew",
    props: true,
    component: BrewLog,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
