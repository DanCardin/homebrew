<template lang="pug">
table.table.table-bordered
  thead
    tr
      th(width="75%") Fermentables
      th Weight
      th.p-0.align-middle
        button.create(@click="create('lb')")
          fa(icon="plus")
  tbody
    batch-fermentable-row(
      v-for="item of batchFermentableStore.items"
      :batchFermentableStore="batchFermentableStore",
      :fermentableStore="fermentableStore",
      :batchFermentable="item"
    )
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import BatchFermentableRow from "./batch-fermentable-row.vue";
import { createBatchFermentableStore } from "../store/batchIngredient";
import { fermentablesStore } from "../store/fermentables";
import { isNull, keyBy, reject } from "lodash-es";
import { useRequests } from "../store/request";

export default defineComponent({
  components: { BatchFermentableRow },
  props: {
    batchId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const requests = useRequests();
    const batchFermentableStore = createBatchFermentableStore(
      requests,
      props.batchId
    );

    const fermentableStore = fermentablesStore();

    await batchFermentableStore.fetch();
    await fermentableStore.search({
      ids: reject(
        batchFermentableStore.items.map((r) => r.fermentableId),
        isNull
      ),
    });

    console.log("rows", batchFermentableStore.items);
    console.log("ferm", fermentableStore.items);
    return {
      batchFermentableStore,
      fermentableStore,
      create: async (unit: string) => {
        await batchFermentableStore.create(unit);
      },
    };
  },
});
</script>
