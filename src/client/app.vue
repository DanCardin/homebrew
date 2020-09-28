<template lang="pug">
#nav.navbar.navbar-light.bg-light.text-dark
  router-link.navbar-brand(to="/")
    img.m-2(src="favicon.svg", width="32", height="32", loading="lazy")
    span.align-middle Homebrew
  fa.justify-content-end.text-dark.m-2(
    v-if="requestPending",
    icon="cog",
    size="lg",
    spin
  )

router-view
</template>
<script lang="ts">
import { requestStore } from "/@client/store/request";
import { computed } from "vue";

export default {
  setup() {
    const pendingRequests = requestStore.getState().pendingRequests;
    const requestPending = computed(() => pendingRequests.size);

    return { requestPending };
  },
};
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
