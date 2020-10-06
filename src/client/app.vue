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
import { useRequests } from "/@client/store/request";

export default {
  setup() {
    const requests = useRequests();
    const requestPending = requests.pending;

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
