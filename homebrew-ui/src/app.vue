<template lang="pug">
.flex.flex-col.min-h-screen
  .rounded-full.m-2.bg-gray-200.fixed.right-0.w-10.h-10.inline-flex.items-center.filter.drop-shadow(
    style="z-index: 50",
    :style="{ opacity: requests.pending ? '80%' : '0%' }"
  )
    cog-icon.text-gray-800.h-10.w-10.my-1.inline-block.mx-auto(
      :class="requests.pending ? 'animate-spin' : ''"
    )
  nav.bg-gray-800.py-0
    .mx-auto.px-2
      .relative.flex.items-center.justify-between
        .flex.flex-1.items-center.justify-center.items-stretch.justify-start
          .flex.flex-shrink-0.items-center
            router-link(to="/")
              span.sr-only Home
              img.h-10.w-auto(
                src="/favicon.svg",
                width="32",
                height="32",
                loading="lazy"
              )
          .block.ml-6.my-2
            .flex.text-center
              router-link.nav-item(to="/") Beers
          .flex.flex-1.justify-end.w-0
            router-link.m-2(to="/hops")
              img.w-auto.h-10(src="/hop.png", width="32", height="32")
            router-link.m-2(to="/fermentables")
              img.w-auto.h-10(src="/grain.png", width="32", height="32")

  .flex-grow
    router-view
  .bg-gray-800.text-gray-400.p-3
    .text-xs.text-right
      | Icons made by
      a.px-1(href="https://www.freepik.com", title="Freepik") Freepik
      | from
      a.px-1(href="https://www.flaticon.com/", title="Flaticon") www.flaticon.com
</template>

<script lang="ts">
import { useRequests } from "./store/request";
import { CogIcon } from "@heroicons/vue/outline";

export default {
  components: { CogIcon },
  setup() {
    const requests = useRequests();

    return { requests };
  },
};
</script>

<style lang="scss">
.nav-item {
  @apply px-3 py-2 rounded-md font-sans font-medium text-gray-300 text-center hover:text-white hover:bg-gray-700;
}
</style>
