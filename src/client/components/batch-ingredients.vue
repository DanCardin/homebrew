<template lang="pug">
.card.m-2.border-secondary.subsection
  .card-header.text-uppercase Fermentables
  .card-body.text-left.text-secondary
    .row
      .col-5
        batch-fermentable-table(:batchId="batchId")
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
              th(width="75%") Other Fermentables
              th Weight/Amt
          tbody
            tr
              td
                input.form-control
              td
                input.form-control
</template>

<script lang="ts">
import { defineComponent } from "vue";
import BatchFermentableTable from "./batch-fermentable-table.vue";
import { fermentablesStore } from "../store/fermentables";
import { useRequests } from "../store/request";

interface Color {
  color: string;
}
export default defineComponent({
  components: { BatchFermentableTable },
  props: {
    batchId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    return {};
  },
});
</script>

<style scoped lang="scss">
@import "../scss/custom.scss";

.subsection {
  border-radius: 0;
  border-left: none;
  border-right: none;
}
</style>
