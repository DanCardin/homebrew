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
      select.text-sm.border-none(class="sm:text-tiny-heading")
        option Lb
        option Oz
.col-span-1.m-2
  button.trash(@click="batchFermentableStore.remove(batchFermentable.id)")
    trash-icon.w-5.h-5
</template>

<script lang="ts">
import { PropType, defineComponent } from "vue";
import type { Fermentable } from "../types/fermentable";
import SearchSelect from "./search-select.vue";
import getUnicodeFlagIcon from "country-flag-icons/unicode";
import { TrashIcon } from "@heroicons/vue/outline";

export default defineComponent({
  components: { SearchSelect, TrashIcon },
  props: {
    batchFermentableStore: {
      required: true,
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
    async function updateValue(value: string) {
      await props.batchFermentableStore.update(
        props.batchFermentable.id,
        props.batchFermentable.fermentableId,
        +value
      );
    }

    async function selectFermentable(selection) {
      await props.batchFermentableStore.update(
        props.batchFermentable.id,
        selection.id,
        props.batchFermentable.amount
      );
    }

    return {
      search: (query: string) => props.fermentableStore.search({ query }),
      selectFermentable,
      updateValue,
      getUnicodeFlagIcon,
    };
  },
});
</script>
