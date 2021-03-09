<template lang="pug">
.card.m-2.border-secondary.p-0
  .card-body.text-secondary.p-0
    .row.card-text.text-left
      .col-3.text-left
        a.btn(@click="toggleEditing", v-show="!editing")
          span.align-middle {{ date }}
        input#brewDate.form-control(
          v-model="date",
          v-show="editing",
          @blur="changeDate",
          ref="dateInput",
          type="date"
        )

      .col.text-dark.text-right.btn.mr-2(@click="toggleExpanded")
        fa(:icon="caretDirection", :key="caretDirection", size="lg")

    .text-dark(v-if="expanded")
      suspense
        batch-targets.mx-0(:batchId="batch.id")
      suspense
        batch-ingredients.mx-0(:batchId="batch.id")
      button.btn.btn-danger.float-right.m-2(
        @click="deleteBatch",
        type="button"
      ) Delete
</template>

<script lang="ts">
import BatchIngredients from "./batch-ingredients.vue";
import BatchTargets from "./batch-targets.vue";
import { useBeerStore } from "../store/beer";
import { computed, ref, defineComponent } from "vue";
export default defineComponent({
  components: { BatchTargets, BatchIngredients },
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
  setup(props) {
    const beerStore = useBeerStore();

    const dateInput = ref<HTMLInputElement | null>(null);
    const date = ref(props.batch.date);
    const editing = ref(false);
    const expanded = ref(false);
    const caretDirection = computed(() =>
      expanded.value ? "caret-down" : "caret-up"
    );

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
      caretDirection,
      toggleExpanded,
      deleteBatch,
    };
  },
});
</script>
