<template lang="pug">
.m-2.p-2.overflow-hidden.rounded-lg
  h3.text-lg.leading-6.font-medium.text-gray-900.mb-3.underline
    span.mr-2 Beers
    a.badge.py-1.px-2.rounded-lg.text-center.text-green-500(
      @click="newBeer",
      class="hover:bg-gray-200 hover:text-gray-900"
    )
      plus-icon.h-5.w-5.inline
  div
    .bg-gray-100.px-4.py-5.grid.grid-cols-3.gap-4.px-6.border-b.border-gray-300.rounded-lg.mb-1.cursor-pointer(
      v-for="beer in beers",
      :key="beer.id",
      @click="selectBeer(beer.id)"
    )
      .text-md.font-medium.text-gray-600
        span Name: {{ beer.name }}
        span.ml-1(v-if="beer.style") ({{ beer.style }})
</template>

<script lang="ts">
import { router } from "@/client/routes";
import { useBeerStore } from "@/client/store/beer";
import { onMounted } from "vue";
import { PlusIcon } from "@heroicons/vue/outline";

export default {
  components: { PlusIcon },
  setup() {
    const beerStore = useBeerStore();
    const beers = beerStore.state.beers;

    onMounted(async () => {
      await beerStore.getBeers();
    });

    async function newBeer() {
      const beer = await beerStore.newBeer();
      router.push({ name: "beer", params: { beerId: beer.id } });
    }
    function selectBeer(id: number) {
      router.push({ name: "beer", params: { beerId: id } });
    }
    return { newBeer, selectBeer, beers };
  },
};
</script>
