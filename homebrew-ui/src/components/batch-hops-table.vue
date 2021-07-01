<template lang="pug">
.grid.grid-cols-7.text-center.py-2
  span.col-span-3 Fermentables
  span.col-span-3 Weight
  button.col-span-1.create(@click="create('lb')")
    plus-icon.h-5.w-5.inline

  batch-fermentable-row(
    v-for="item of batchFermentableStore.items",
    :batchFermentable="item",
    :batchFermentableStore="batchFermentableStore",
    :fermentableStore="fermentableStore",
    :key="item.id"
  )
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
import BatchFermentableRow from "./batch-fermentable-row.vue";
import { useBatchFermentableStore } from "../store/batchIngredient";
import { fermentablesStore } from "../store/fermentables";
import { isNull, keyBy, reject } from "lodash-es";
import { PlusIcon } from "@heroicons/vue/outline";

export default defineComponent({
  components: { BatchFermentableRow, PlusIcon },
  props: {
    batchId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const batchFermentableStore = useBatchFermentableStore();

    const fermentableStore = fermentablesStore();

    await batchFermentableStore.fetch(props.batchId);
    await fermentableStore.search({
      ids: reject(
        batchFermentableStore.get(props.batchId).map((r) => r.fermentableId),
        isNull
      ),
    });

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
