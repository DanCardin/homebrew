import "/@client/registerServiceWorker";
import "/@client/scss/custom.scss";

import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faBeer,
  faCaretDown,
  faCaretUp,
  faCog,
  faPlus,
  faTrash,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import App from "/@client/app.vue";
import router from "/@client/router";
import { requestsSymbol, state } from "/@client/store/request";
import { createApp } from "vue";

library.add(faBeer);
library.add(faCaretDown);
library.add(faCaretUp);
library.add(faCog);
library.add(faPlus);
library.add(faTrash);

createApp(App)
  .use(router)
  .component("fa", FontAwesomeIcon)
  .provide(requestsSymbol, state)
  .mount("#app");
