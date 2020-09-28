import Beer from "/@client/views/beer.vue";
import Home from "/@client/views/home.vue";
import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/beer/:beerId",
    name: "beer",
    props: (route) => ({ beerId: +route.params.beerId }),
    component: Beer,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
