<template lang="pug">
table.col.table.table-bordered.mx-0
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
            label.mb-0(for="targetOGInput") Target
            input#targetOGInput.form-control(
              @blur="saveMeasurement('targetOG', targetOG)",
              v-model.number="targetOG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualOGInput") Actual
            input#actualOGInput.form-control(
              v-model.number="actualOG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.specific-gravity
            label.mb-0(for="targetFGInput") Target
            input#targeFGtInput.form-control(
              v-model.number="targetFG",
              type="number",
              min="0",
              max="2",
              step=".001"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualFGInput") Actual
            input#actualFGInput.form-control
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.abv
            label.mb-0(for="targetABVInput") Target
            input#targetABVInput.form-control(
              ,
              readonly,
              v-bind:value="targetABV"
            )
          .mx-1.abv
            label.mb-0(for="actualABVInput") Actual
            input#actualABVInput.form-control
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetIBUInput") Target
            input#targetIBUInput.form-control.ibu
          .mx-1
            label.mb-0(for="actualIBUInput") Actual
            input#actualIBUInput.form-control.ibu
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetSRMInput") Target
            input#targetSRMInput.form-control.srm(
              v-model="targetSRM",
              @input="targetSRMInput"
            )
            span(:style="targetSRMHex")
              fa(icon="beer")
          .mx-1
            label.mb-0(for="actualSRMInput") Actual
            input#actualSRMInput.form-control.srm(
              v-model="actualSRM",
              @input="actualSRMInput"
            )
            span(:style="actualSRMHex")
              fa(icon="beer")
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetVolumeInput") Target
            input#targetVolumeInput.form-control
          .mx-1
            label.mb-0(for="actualVolumeInput") Actual
            input#actualVolumneInput.form-control
</template>

<script setup lang="ts">
import axios from "axios";
import { computed, reactive, ref } from "vue";

interface Color {
  color: string;
}
export default {
  setup() {
    const targetOG = ref("1");
    const actualOG = ref("1");

    const targetFG = ref("1");
    const actualFG = ref("1");

    const actualABV = ref("");

    const targetIBU = ref("");
    const actualIBU = ref("");

    const targetSRM = ref("");
    const actualSRM = ref("");

    const targetSRMHex: reactive<Color> = reactive({});
    const actualSRMHex: reactive<Color> = reactive({});

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

    async function saveMeasurement(key, value) {
      console.log(key, value);
    }
    return {
      actualSRMInput,
      targetSRMInput,
      targetABV,
      targetOG,
      actualOG,
      targetFG,
      actualFG,
      actualABV,
      targetIBU,
      actualIBU,
      targetSRM,
      actualSRM,
      targetSRMHex,
      actualSRMHex,
      saveMeasurement,
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
