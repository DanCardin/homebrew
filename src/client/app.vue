<template lang="pug">
#nav.navbar.navbar-light.bg-light.text-dark
  router-link.navbar-brand(to="/")
    img.m-2(src="favicon.svg", width="32", height="32", loading="lazy")
    span.align-middle Homebrew
  .justify-content-end
    router-link.m-2(to="/fermentables")
      span.m-2.wheat(width="32", height="32")
    fa.text-dark.m-2(
      :style="{ opacity: requestPending ? '100%' : '0%' }",
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
.wheat {
  mask: url("/fermentable.svg");
  width: 32px;
  height: 32px;
  display: inline-block;
  background: rgb(227, 162, 0);
  mask-size: cover;
}
</style>
