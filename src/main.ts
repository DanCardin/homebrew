import { createApp } from "vue";

import App from "@/client/app.vue";
import "@/client/registerServiceWorker";
import router from "@/client/router";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faBeer, faPlus } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import "bootstrap/scss/bootstrap.scss";

library.add(faBeer);
library.add(faPlus);

createApp(App)
  .use(router)
  .component("fa", FontAwesomeIcon)
  .mount("#app");
