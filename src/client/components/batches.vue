<template lang="pug">
.card.m-2.border-secondary
  .card-header.text-uppercase Batches
  .card-body.text-secondary
    .row.card-text.text-left
      .col-2.m-2.button.btn.btn-primary(@click="createBatch(beerId)")
        span Create Batch

      .row(v-for="batch of batches" :key="batch.id")
        batch(:beerId="beerId", :batch="batch")
</template>

<script lang="ts">
import Batch from "../components/batch.vue";
import BeerInfo from "../components/beer-info.vue";
import { beerStore } from "../store/beer";
import { computed } from "vue";

export default {
  components: { BeerInfo, Batch },
  props: { beerId: Number },
  async setup(props) {
    const store = beerStore.getState();
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
};
</script>
