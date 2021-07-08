<template lang="pug">
.grid.grid-cols-2
  span wat
</template>

<script lang="ts">
import { useBeerStore } from "../store/beer";
import { computed, defineComponent } from "vue";

export default defineComponent({
  props: {
    beerId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const beerStore = useBeerStore();

    const store = beerStore.state;

    return {
    };
  },
});
</script>
