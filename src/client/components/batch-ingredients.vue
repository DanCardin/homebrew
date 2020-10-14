<template lang="pug">
.card.m-2.border-secondary.subsection
  .card-header.text-uppercase Ingredients
  .card-body.text-left.text-secondary
    .row
      .col-5
        table.table.table-bordered
          thead
            tr
              th(width="75%") Grain
              th Weight
              th.p-0.align-middle
                button.create
                  fa(icon="plus" @click="newIngredient('grain', 'lb')")
          tbody
            tr(v-for="item of ingredients.grain")
              td.p-0
                input.table-input.form-control(:value="item.name")
              td.p-0
                input.table-input.form-control(:value="item.value")
              td.p-0
                button.trash
                  fa(icon="trash" @click="deleteIngredient('grain', item.id)")
      .col-7
        table.table.table-bordered
          thead
            tr
              th(width="65%")
                span Hops
                span.text-sm.text-secondary &nbsp;(Form/AA)
              th(width="20%") Time
              th(width="15%") Weight
          tbody
            tr
              td
                input.form-control
              td
                input.form-control
              td
                input.form-control
    .row
      .col-5
        table.table.table-bordered
          thead
            tr
              th(width="75%")
                span H20 Treatments
              th Weight
          tbody
            tr
              td
                input.form-control
              td
                input.form-control
      .col-7
        table.table.table-bordered
          thead
            tr
              th(width="75%") Other Ingredients
              th Weight/Amt
          tbody
            tr
              td
                input.form-control
              td
                input.form-control
    search-select
</template>

<script lang="ts">
import SearchSelect from "./search-select.vue";
import { createBatchIngredientStore } from "../store/batch";
import { useRequests } from "../store/request";

interface Color {
  color: string;
}
export default {
  components: { SearchSelect },
  props: {
    batchId: Number,
  },
  async setup(props) {
    const requests = useRequests();
    const {
      deleteIngredient,
      fetchIngredients,
      newIngredient,
      updateIngredient,
      ingredients,
    } = createBatchIngredientStore(requests, props.batchId);
    await fetchIngredients();
    return {
      deleteIngredient,
      fetchIngredients,
      newIngredient,
      updateIngredient,
      ingredients,
    };
  },
};
</script>

<style scoped lang="scss">
@import "../scss/custom.scss";

.subsection {
  border-radius: 0;
  border-left: none;
  border-right: none;
}

.table-input {
  border: none;
}
</style>
