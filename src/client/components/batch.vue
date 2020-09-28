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
      batch-targets(:batchId="batch.id")
      button.btn.btn-danger.float-right.m-2(
        @click="deleteBatch",
        type="button"
      ) Delete
</template>

<script lang="ts">
import BatchTargets from "/@client/components/batch-targets.vue";
import { beerStore } from "/@client/store/beer";
import { computed, ref } from "vue";
export default {
  components: { BatchTargets },
  props: {
    beerId: Number,
    batch: Object,
  },
  setup(props) {
    const dateInput: ref<HTMLInputElement> = ref(null);
    const editing = ref(false);
    const expanded = ref(false);
    const caretDirection = computed(() =>
      expanded.value ? "caret-down" : "caret-up"
    );

    async function changeDate() {
      if (props.date) {
        await beerStore.updateBatchDate(
          props.beerId,
          props.batch.id,
          props.date.value
        );
      }
      editing.value = false;
    }
    function toggleEditing() {
      editing.value = !editing.value;
      setTimeout(() => {
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
      date: props.batch.date,
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
};
</script>
