<template lang="pug">
table.col.table.table-bordered
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
          .mx-1
            input-gravity(
              @change.lazy="saveMeasurement('targetOG', $event.target.value)",
              :value="batchInfo.measurements.targetOG",
              label="Target"
            )
          .mx-1
            input-gravity(
              @change.lazy="saveMeasurement('actualOG', $event.target.value)",
              :value="batchInfo.measurements.actualOG",
              label="Actual"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            input-gravity(
              @change.lazy="saveMeasurement('targetFG', $event.target.value)",
              :value="batchInfo.measurements.targetFG",
              label="Target"
            )
          .mx-1
            input-gravity(
              @change.lazy="saveMeasurement('actualFG', $event.target.value)",
              :value="batchInfo.measurements.actualFG",
              label="Actual"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            input-abv(:value="abvInfo.target.abv", label="Target")
          .mx-1
            input-abv(:value="abvInfo.actual.abv", label="Actual")
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
            input-volume(
              @change.lazy="saveMeasurement('targetVolume', $event.target.value)",
              :value="batchInfo.measurements.targetVolume",
              label="Target"
            )
          .mx-1
            input-volume(
              @change.lazy="saveMeasurement('actualVolume', $event.target.value)",
              :value="batchInfo.measurements.actualVolume",
              label="Actual"
            )
</template>

<script lang="ts">
import InputAbv from "./input-abv.vue";
import InputGravity from "./input-gravity.vue";
import InputVolume from "./input-volume.vue";
import { createBatchStore } from "../store/batch";
import { useRequests } from "../store/request";
import axios from "axios";
import { reactive, ref } from "vue";

interface Color {
  color: string;
}
export default {
  components: { InputAbv, InputGravity, InputVolume },
  props: {
    batchId: Number,
  },
  async setup(props) {
    const requests = useRequests();
    const { batchInfo, abvInfo, fetch, saveMeasurement } = createBatchStore(
      requests,
      props.batchId
    );
    await fetch();

    const actualOG = ref("1");
    const actualFG = ref("1");
    const targetIBU = ref("");
    const actualIBU = ref("");
    const targetSRM = ref("");
    const actualSRM = ref("");
    const targetSRMHex: reactive<Color> = reactive({});
    const actualSRMHex: reactive<Color> = reactive({});

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
      actualOG,
      actualFG,
      targetIBU,
      actualIBU,
      targetSRM,
      actualSRM,
      targetSRMHex,
      actualSRMHex,
      abvInfo,
      batchInfo,
      saveMeasurement,
    };
  },
};
</script>

<style scoped lang="scss">
.text-sm {
  font-size: 0.6em;
}
.ibu {
  min-width: 2.6rem;
}
.srm {
  min-width: 2.6rem;
}
</style>
