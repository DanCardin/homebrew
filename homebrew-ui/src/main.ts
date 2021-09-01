import { Chart, registerables } from "chart.js";

import { createApp } from "vue";

import "@/styles/index.css";

import App from "@/app.vue";
import { router } from "@/routes";
import { createPinia } from "pinia";

Chart.register(...registerables);

createApp(App).use(router).use(createPinia()).mount("#app");
