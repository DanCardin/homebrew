<template lang="pug">
.card.m-2.border-secondary
  .card-header.text-uppercase Beer Info
  .card-body.text-secondary
    .row.card-title.text-left
      .col
        span(for="beerName") Beer Name
        input#beerName.form-control(v-model.lazy="name", @blur="changeInfo")
      .col.text-right
        span(for="beerStyle") Style
        input#beerStyle.form-control(v-model.lazy="style", @blur="changeInfo")
</template>

<script lang="ts">
import { beerStore } from "../store/beer";
import { isNaN } from "lodash-es";
import { defineComponent, ref } from "vue";

async function infoChanged(
  beerId: number,
  name: ref<string>,
  style: ref<string>
) {
  if (!name.value && !style.value) {
    return;
  }
  if (isNaN(beerId)) {
    return;
  }
  const updatedBeer = await beerStore.update({
    id: beerId,
    name: name.value,
    style: style.value,
  });

  name.value = updatedBeer.name;
  style.value = updatedBeer.style;
}
export default defineComponent({
  props: { beerId: Number },
  async setup(props) {
    const name = ref("");
    const style = ref("");

    const beer = await beerStore.getBeer(props.beerId);

    name.value = beer.name;
    style.value = beer.style;

    async function changeInfo() {
      await infoChanged(+props.beerId, name, style);
    }
    return { name, style, changeInfo };
  },
});
</script>
