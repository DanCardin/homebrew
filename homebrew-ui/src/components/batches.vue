<template lang="pug">
.grid.grid-cols-2
  .col-span-1.flex
    h3.my-auto.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Batches

  .col-span-1.mt-2.text-gray-600.text-right
    button.bg-blue-500.hover.text-white.font-bold.py-2.px-4(
      class="rounded:bg-blue-700",
      @click="createBatch(beerId)"
    ) Create Batch

  .col-span-2.py-4.px-8.bg-white.shadow-lg.rounded-lg.my-2
    .grid.grid-cols-1(v-for="batch of batches", :key="batch.id")
      batch.col-span-1(:beerId="beerId", :batch="batch")
</template>

<script lang="ts">
import Batch from "../components/batch.vue";
import BeerInfo from "../components/beer-info.vue";
import { useBeerStore } from "../store/beer";
import { computed, defineComponent } from "vue";

export default defineComponent({
  components: { BeerInfo, Batch },
  props: {
    beerId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const beerStore = useBeerStore();

    const store = beerStore.state;
    const batches = computed(() => store.batches);

    await beerStore.getBatches(props.beerId);

    async function createBatch(beerId: number) {
      beerStore.createBatch(beerId);
    }
    return {
      batches,
      createBatch,
    };
  },
});
</script>
