import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";

import Beer from "@/views/beer.vue";
import Fermentables from "@/views/fermentables.vue";
import Home from "@/views/home.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "home",
        meta: { title: 'Home' },
        component: Home,
    },
    {
        path: "/beer/:beerId",
        name: "beer",
        meta: { title: 'Beer' },
        props: (route) => ({ beerId: +route.params.beerId }),
        component: Beer,
    },
    {
        path: "/fermentables",
        name: "fermentables",
        meta: { title: 'Fermentables' },
        component: Fermentables,
    },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
