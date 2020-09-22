import "/@client/registerServiceWorker";
import "bootstrap/scss/bootstrap.scss";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faBeer, faPlus } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import App from "/@client/app.vue";
import router from "/@client/router";
import { createApp } from "vue";

library.add(faBeer);
library.add(faPlus);

createApp(App).use(router).component("fa", FontAwesomeIcon).mount("#app");
