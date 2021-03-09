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
