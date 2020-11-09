<template lang="pug">
tr
  td.p-0
    span {{ fermentableStore.get(batchFermentable.fermentableId) }}
  td.p-0
    search-select(
      :selectedItem="fermentableStore.get(batchFermentable.fermentableId)",
      :search="search",
      @update:modelValue="selectFermentable"
    )
      template(v-slot:row="props")
        span(v-if="props.item.country") {{ getUnicodeFlagIcon(props.item.country) }}
        span {{ props.item.name }}
  td.p-0
    input.table-input.form-control(
      :value="batchFermentable.amount",
      @input.lazy="updateValue($event.target.value)"
    )
  td.p-0
    button.trash(@click="batchFermentableStore.remove(batchFermentable.id)")
      fa(icon="trash")
</template>

<script lang="ts">
import { PropType, defineComponent } from "vue";
import type { Fermentable } from "../types/fermentable";
import SearchSelect from "./search-select.vue";
import getUnicodeFlagIcon from "country-flag-icons/unicode";

export default defineComponent({
  components: { SearchSelect },
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
  async setup(props) {
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

<style scoped lang="scss">
.table-input {
  border: none;
}
</style>
