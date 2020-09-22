<template lang="pug">
.row.card-title.text-left
  .col
    span(for="beerName") Beer Name
    input#beerName.form-control(v-model.lazy="name")
  .col.text-right
    span(for="brewDate") Brew Date
    input#brewDate.form-control(
      @v-model="date",
      @blur="changeDate",
      type="date"
    )
.row.card-text.text-left
  .col
    span(for="beerStyle") Style
    input#beerStyle.form-control(v-model.lazy="style", @blur="changeStyle")
  .col.text-right
    span(for="batchNumber") Batch No.
    input#batchNumber.form-control(v-model.lazy="batch", @blur="changeBatch")
</template>

<script lang="ts">
import { brewsStore } from "/@client/store/brews";
import { isNaN } from "lodash-es";
import { onMounted, ref, watch } from "vue";
export default {
  props: {
    brewId: Number,
  },
  setup(props) {
    const name = ref("");
    const style = ref("");
    const date = ref("");
    const batch = ref("");
    onMounted(async () => {
      console.log("prop", props.brewId);
    });
    watch(name, (name, oldName) => {
      if (name === oldName) {
        return;
      }
      const brewId = +(props.brewId || "");
      if (isNaN(brewId)) {
        return;
      }
      brewsStore.update({ id: brewId, name: name });
    });
    return { name, style, date, batch };
  },
};
</script>
