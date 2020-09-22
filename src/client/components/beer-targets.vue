<template lang="pug">
table.col.table.table-bordered.mb-0
  thead
    tr
      th OG
      th FG
      th ABV
      th IBU
      th SRM
      th Volume
  tbody
    tr.text-uppercase
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.specific-gravity
            label.mb-0(for="targetOG") Target
            input#targetInput.form-control(
              v-model.number="targetOG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualOG") Actual
            input#actualInput.form-control(
              v-model.number="actualOG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.specific-gravity
            label.mb-0(for="targetFG") Target
            input#targetInput.form-control(
              v-model.number="targetFG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualFG") Actual
            input#actualInput.form-control
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.abv
            label.mb-0(for="targetABV") Target
            input#targetInput.form-control(
              ,
              readonly,
              v-bind:value="targetABV"
            )
          .mx-1.abv
            label.mb-0(for="actualABV") Actual
            input#actualInput.form-control
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetIBU") Target
            input#targetInput.form-control.ibu
          .mx-1
            label.mb-0(for="actualIBU") Actual
            input#actualInput.form-control.ibu
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetSRM") Target
            input#targetInput.form-control.srm(
              v-model="targetSRM",
              @input="targetSRMInput"
            )
            span(:style="targetSRMHex")
              fa(icon="beer")
          .mx-1
            label.mb-0(for="actualSRM") Actual
            input#actualInput.form-control.srm(
              v-model="actualSRM",
              @input="actualSRMInput"
            )
            span(:style="actualSRMHex")
              fa(icon="beer")
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetVolume") Target
            input#targetInput.form-control
          .mx-1
            label.mb-0(for="actualVolume") Actual
            input#actualInput.form-control
</template>

<script setup lang="ts">
import axios from "axios";
import { computed, reactive, ref, watch } from "vue";

export default {
  setup() {
    const targetOG = ref("1");
    const actualOG = ref("1");

    const targetFG = ref("1");
    const actualFG = ref("1");

    const targetSRM = ref("");
    const actualSRM = ref("");
    const targetSRMHex = reactive({});
    const actualSRMHex = reactive({});

    watch(targetOG, (val: string, oldVal: string) => {
      if (!actualOG.value || actualOG.value == oldVal) {
        actualOG.value = val;
      }
    });

    watch(targetFG, (val: string, oldVal: string) => {
      if (!actualFG.value || actualFG.value == oldVal) {
        actualFG.value = val;
      }
    });

    const targetABV = computed(() => {
      const og = +targetOG.value;
      const fg = +targetFG.value;
      return ((76.08 * (og - fg)) / (1.775 - og)) * (fg / 0.794);
    });

    const targetSRMInput = async () => {
      let color = "#000000";
      if (targetSRM.value) {
        const response = await axios.post("/api/srm.convert", {
          value: +targetSRM.value,
        });
        color = response.data.value;
      }
      targetSRMHex.color = color;
    };

    const actualSRMInput = async () => {
      let color = "#000000";
      if (actualSRM.value) {
        const response = await axios.post("/api/srm.convert", {
          value: +actualSRM.value,
        });
        color = response.data.value;
      }
      actualSRMHex.color = color;
    };

    return {
      actualSRMInput,
      targetSRMInput,
      targetABV,
      targetOG,
      actualOG,
      targetFG,
      actualFG,
      targetSRM,
      actualSRM,
      targetSRMHex,
      actualSRMHex,
    };
  },
};
</script>

<style scoped lang="scss">
.text-sm {
  font-size: 0.6em;
}
.specific-gravity {
  min-width: 4.9rem;
}
.abv {
  min-width: 3.5rem;
}
.ibu {
  min-width: 2.6rem;
}
.srm {
  min-width: 2.6rem;
}
</style>
