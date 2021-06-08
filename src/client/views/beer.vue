<template lang="pug">
suspense
  beer-info.m-2(:beerId="beerId")

suspense
  batches.mx-2.my-4(:beerId="beerId")
</template>

<script lang="ts">
import Batches from "../components/batches.vue";
import BeerInfo from "../components/beer-info.vue";

export default {
  components: { BeerInfo, Batches },
  props: { beerId: Number },
};
</script>
