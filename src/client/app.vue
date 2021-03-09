<template lang="pug">
nav.bg-gray-800
  .mx-auto.px-2
    .relative.flex.items-center.justify-between
      .flex.flex-1.items-center.justify-center.items-stretch.justify-start
        .flex.flex-shrink-0.items-center
          router-link(to="/")
            span.sr-only Home
            img.h-10.w-auto(
              src="favicon.svg",
              width="32",
              height="32",
              loading="lazy"
            )
        .block.ml-6.my-2
          .flex.text-center
            router-link.nav-item(to="/") Beers
        .flex.flex-1.justify-end.w-0
          router-link.m-2(to="/fermentables")
            img.w-auto.h-10.wheat(width="32", height="32")

          fa.text-gray-300.h-10.w-10.my-2(
            :style="{ opacity: requestPending ? '100%' : '0%' }",
            icon="cog",
            size="lg",
            spin
          )

router-view
</template>

<script lang="ts">
import { useRequests } from "./store/request";

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
  background: rgb(227, 162, 0);
  // mask-size: cover;
}

.nav-item {
  @apply px-3 py-2 rounded-md font-sans font-medium text-gray-300 text-center hover:text-white hover:bg-gray-700;
}
</style>
