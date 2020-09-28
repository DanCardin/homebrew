<template lang="pug">
div
  h3
    span Beers &nbsp;
    a.badge(@click="newBeer")
      fa.text-dark(icon="plus", size="xs")
  .row(v-for="beer in beers")
    .col
      .card.m-2.border-secondary
        .card-header.text-uppercase(@click="selectBeer(beer.id)")
          span Name: {{ beer.name }}
          span(v-if="beer.style") &nbsp;({{ beer.style }})
        .card-body.text-secondary.p-0
          .mx-1
            span
</template>

<script setup lang="ts">
import router from "/@client/router";
import { beerStore } from "/@client/store/beer";
import { onMounted } from "vue";

export const beers = beerStore.getState().beers;

onMounted(async () => {
  await beerStore.getBeers();
});

export async function newBeer() {
  const beer = await beerStore.newBeer();
  router.push({ name: "beer", params: { beerId: beer.id } });
}
export function selectBeer(id: number) {
  router.push({ name: "beer", params: { beerId: id } });
}
</script>
