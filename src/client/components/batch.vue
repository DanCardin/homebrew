<template lang="pug">
.mx-0.my-2.p-0.text-gray-800
  .grid.grid-cols-2
    .col-span-1.text-left.flex
      button.bg-blue-500.hover.text-white.font-bold.py-2.px-4(
        @click="toggleEditing",
        v-show="!editing"
      )
        span.align-middle {{ date }}
      input#brewDate.form-control(
        v-model="date",
        v-show="editing",
        @blur="changeDate",
        ref="dateInput",
        type="date"
      )

    .col-span-1(@click="toggleExpanded")
      .float-right
        chevron-up-icon.h-5.w-5(v-if="expanded")
        chevron-down-icon.h-5.w-5(v-else)

  .text-dark(v-if="expanded")
    suspense
      batch-targets(:batchId="batch.id")

    h3.text-center.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Fermentables
    suspense
      batch-fermentable-table(:batchId="batch.id")

    h3.text-center.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Hops
    suspense
      batch-hops-table(:batchId="batch.id")

    button.bg-red-500.hover.text-white.font-bold.py-2.px-4.float-right(
      @click="deleteBatch",
      type="button"
    ) Delete
</template>

<script lang="ts">
import { useBatchStore } from "../store/batch";
import BatchTargets from "./batch-targets.vue";
import BatchFermentableTable from "./batch-fermentable-table.vue";
import BatchHopsTable from "./batch-hops-table.vue";
import { useBeerStore } from "../store/beer";
import { computed, ref, defineComponent } from "vue";
import { ChevronUpIcon, ChevronDownIcon } from "@heroicons/vue/outline";
import { useNoteStore } from "@/client/store/note";

export default defineComponent({
  components: {
    BatchTargets,
    ChevronDownIcon,
    ChevronUpIcon,
    BatchFermentableTable,
    BatchHopsTable,
  },
  props: {
    beerId: {
      type: Number,
      required: true,
    },
    batch: {
      type: Object,
      required: true,
    },
  },
  async setup(props) {
    const beerStore = useBeerStore();
    const batchStore = useBatchStore();
    const noteStore = useNoteStore();

    await noteStore.fetch(props.batch.id);

    const dateInput = ref<HTMLInputElement | null>(null);
    const date = ref(props.batch.date);
    const editing = ref(false);
    const expanded = ref(false);

    async function changeDate() {
      if (date.value) {
        await beerStore.updateBatchDate(
          props.beerId,
          props.batch.id,
          date.value
        );
      }
      editing.value = false;
    }
    function toggleEditing() {
      editing.value = !editing.value;
      setTimeout(() => {
        if (!dateInput.value) {
          return;
        }
        dateInput.value.focus();
      }, 0);
    }
    function toggleExpanded() {
      expanded.value = !expanded.value;
    }
    async function deleteBatch() {
      await beerStore.deleteBatch(props.batch.id);
      await beerStore.getBatches(props.beerId);
    }
    return {
      date,
      dateInput,
      expanded,
      editing,
      changeDate,
      toggleEditing,
      toggleExpanded,
      deleteBatch,
    };
  },
});
</script>
