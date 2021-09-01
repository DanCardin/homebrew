<template lang="pug">
.text-gray-800.grid.grid-flow-row.auto-rows-max
  .grid-cols-auto.flex.justify-end
    .mr-auto
      button.bg-blue-500.hover.text-white.font-bold.py-2.px-4(
        @click="toggleEditing",
        v-show="!editing"
      )
        span.align-middle {{ date }}
      input#brewDate.block.w-full.px-4.text-sm.border-gray-300.rounded-lg(
        class="focus:border-indigo-500 focus:ring-indigo-500",
        v-model="date",
        v-show="editing",
        @blur="changeDate",
        ref="dateInput",
        type="date"
      )

    button.hover.text-white.font-bold.py-2.px-4(
      :class="[expandedPlanning ? 'bg-blue-500' : 'bg-blue-200']",
      @click="expandedPlanning = !expandedPlanning"
    ) Planning
    button.hover.text-white.font-bold.py-2.px-4(
      :class="[expandedFermentation ? 'bg-blue-500' : 'bg-blue-200']",
      @click="expandedFermentation = !expandedFermentation"
    ) Fermentation
    button.bg-red-500.hover.text-white.font-bold.py-2.px-4(
      @click="isConfirmDeleteModalVisible = true",
      type="button"
    ) Delete
    confirm-delete(
      v-show="isConfirmDeleteModalVisible",
      modalHeadline="Delete Batch?",
      deleteMessage="Are you sure?",
      @deleteRecordEvent="deleteBatch",
      @close="isConfirmDeleteModalVisible = false"
    )

  .m-4.text-dark(v-if="expandedPlanning")
    suspense
      batch-targets(:batchId="batch.id")

    h3.text-center.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Fermentables

    .flex.flex-nowrap.py-2.justify-evenly
      batch-fermentable-table(:batchId="batch.id")
      batch-fermentable-table(:batchId="batch.id")

    h3.text-center.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase Hops
    suspense
      batch-hops-table(:batchId="batch.id")

  .m-4.text-dark(v-if="expandedFermentation")
    .flex-1.flex-nowrap.py-2.justify-evenly
      suspense
        batch-gravity-readings(:batchId="batch.id")
</template>

<script lang="ts">
import { useBatchStore } from "../store/batch";
import BatchTargets from "./batch-targets.vue";
import BatchFermentableTable from "./batch-fermentable-table.vue";
import BatchHopsTable from "./batch-hops-table.vue";
import BatchGravityReadings from "./batch-gravity-readings.vue";
import ConfirmDelete from "./confirm-delete.vue";
import { useBeerStore } from "../store/beer";
import { computed, defineComponent, ref } from "vue";
import { ChevronDownIcon, ChevronUpIcon } from "@heroicons/vue/outline";
import { useNoteStore } from "@/store/note";

export default defineComponent({
  components: {
    BatchTargets,
    ChevronDownIcon,
    ChevronUpIcon,
    BatchFermentableTable,
    BatchHopsTable,
    BatchGravityReadings,
    ConfirmDelete,
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

    const expandedPlanning = ref(false);
    const expandedFermentation = ref(false);
    const isConfirmDeleteModalVisible = ref(false);

    async function changeDate() {
      if (date.value) {
        editing.value = false;
        await beerStore.updateBatchDate(
          props.beerId,
          props.batch.id,
          date.value
        );
      } else {
        editing.value = false;
      }
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
    async function deleteBatch() {
      await beerStore.deleteBatch(props.batch.id);
      await beerStore.getBatches(props.beerId);
    }
    return {
      date,
      dateInput,
      expandedPlanning,
      expandedFermentation,
      editing,
      changeDate,
      toggleEditing,
      deleteBatch,
      isConfirmDeleteModalVisible,
    };
  },
});
</script>
