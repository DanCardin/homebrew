import { createApp } from "vue";

import "@/client/styles/index.css";

import App from "@/client/app.vue";
import { router } from "@/client/routes";
import { createPinia } from 'pinia'


createApp(App)
  .use(router)
  .use(createPinia())
  .mount("#app");
