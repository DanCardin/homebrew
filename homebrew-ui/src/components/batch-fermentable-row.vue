<template lang="pug">
.col-span-3.m-2
  search-select(
    :selectedItem="fermentableStore.get(batchFermentable.fermentableId)",
    :search="search",
    @update:modelValue="selectFermentable"
  )
    template(v-slot:row="props")
      span(v-if="props.item.country") {{ getUnicodeFlagIcon(props.item.country) }}
      span {{ props.item.name }}
.col-span-3.m-2
  .relative.rounded-md.shadow-sm
    input#actualSRMInput.block.w-full.pl-5.pr-1.text-sm.border-gray-300.rounded-md(
      class="focus:ring-indigo-500 focus:border-indigo-500",
      type="text",
      :value="batchFermentable.amount",
      @input.lazy="updateValue($event.target.value)"
    )
    .absolute.inset-y-0.right-2.pl-3.flex.items-center
      select.text-sm.border-none.text-tiny-heading(
        :value="batchFermentable.unit",
        @change="updateUnit($event.target.value)"
      )
        option(value="") ...
        option(value="lb") Lb
        option(value="oz") Oz
.col-span-1.m-2
  button(@click="batchFermentableStore.remove(batchId, batchFermentable.id)")
    trash-icon.w-5.h-5.text-red-500
</template>

<script lang="ts">
import { PropType, defineComponent, ref } from "vue";
import type { Fermentable } from "../types/fermentable";
import SearchSelect from "./search-select.vue";
import getUnicodeFlagIcon from "country-flag-icons/unicode";
import { TrashIcon } from "@heroicons/vue/outline";
import { useBatchFermentableStore } from "../store/batchIngredient";

export default defineComponent({
  components: { SearchSelect, TrashIcon },
  props: {
    batchId: {
      required: true,
      type: Number,
    },
    fermentableStore: {
      required: true,
    },
    batchFermentable: {
      type: Object,
      required: true,
    },
  },
  setup(props) {
    const batchFermentableStore = useBatchFermentableStore();

    const selectedUnit = ref("");

    return {
      search(query: string) {
        props.fermentableStore.search({ query });
      },

      async selectFermentable(selection) {
        await batchFermentableStore.update(
          props.batchId,
          props.batchFermentable.id,
          selection && selection.id,
          props.batchFermentable.amount
        );
      },

      async updateValue(value: string) {
        let actualValue = !!value ? +value : null;

        await batchFermentableStore.update(
          props.batchId,
          props.batchFermentable.id,
          props.batchFermentable.fermentableId,
          actualValue,
          props.batchFermentable.unit
        );
      },

      async updateUnit(value: string) {
        await batchFermentableStore.update(
          props.batchId,
          props.batchFermentable.id,
          props.batchFermentable.fermentableId,
          props.batchFermentable.amount,
          value
        );
      },

      getUnicodeFlagIcon,
      batchFermentableStore,
      selectedUnit,
    };
  },
});
</script>
