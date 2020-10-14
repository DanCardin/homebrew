<template lang="pug">
suspense
  beer-info(:beerId="beerId")

suspense
  batches(:beerId="beerId")
</template>

<script lang="ts">
import Batches from "../components/batches.vue";
import BeerInfo from "../components/beer-info.vue";

export default {
  components: { BeerInfo, Batches },
  props: { beerId: Number },
};
</script>
