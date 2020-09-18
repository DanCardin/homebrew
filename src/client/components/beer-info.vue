<template lang="pug">
.row.card-title.text-left
  .col
    span(for="beerName") Beer Name
    input.form-control(id="beerName")
  .col.text-right
    span(for="brewDate") Brew Date
    input.form-control(id="brewDate", type="date")
.row.card-text.text-left
  .col
    span(for="beerStyle") Style
    input.form-control(id="beerStyle")
  .col.text-right
    span(for="batchNumber") Batch No.
    input.form-control(id="batchNumber")
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({ components: {} })
export default class BeerInfo extends Vue {
}
</script>
