<template lang="pug">
.m-2
  h3.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Beer Info
  .mt-1.relative.rounded-md.shadow-sm
    .block.text-sm.font-medium.text-gray-700 Beer Name
    .absolute.flex.items-center
      input#beerName.block.w-full.pl-7.pr-12.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        type="text",
        v-model.lazy="name",
        @blur="changeInfo"
      )
      .col.text-right
        span(for="beerStyle") Style
        input#beerStyle.form-control(v-model.lazy="style", @blur="changeInfo")
</template>

<script lang="ts">
import { useBeerStore } from "../store/beer";
import { isNaN } from "lodash-es";
import { defineComponent, ref, Ref } from "vue";

async function infoChanged(
  beerStore: any,
  beerId: number,
  name: Ref<string | undefined>,
  style: Ref<string | undefined>
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
  props: {
    beerId: {
      type: Number,
    },
  },
  async setup(props) {
    const beerStore = useBeerStore();

    const name = ref<string | undefined>("");
    const style = ref<string | undefined>("");

    if (!props.beerId) {
      return;
    }

    const beer = await beerStore.getBeer(props.beerId);

    name.value = beer.name;
    style.value = beer.style;

    async function changeInfo() {
      if (!props.beerId) {
        return;
      }
      await infoChanged(beerStore, +props.beerId, name, style);
    }
    return { name, style, changeInfo };
  },
});
</script>
