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
            gravity-input(
              @change.lazy="saveMeasurement('targetOG', $event.target.value)",
              :value="batchInfo.measurements.targetOG",
              label="Target"
            )
          .mx-1.specific-gravity
            gravity-input(
              @change.lazy="saveMeasurement('actualOG', $event.target.value)",
              :value="batchInfo.measurements.actualOG",
              label="Actual"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.specific-gravity
            gravity-input(
              @change.lazy="saveMeasurement('targetFG', $event.target.value)",
              :value="batchInfo.measurements.targetFG",
              label="Target"
            )
          .mx-1.specific-gravity
            gravity-input(
              @change.lazy="saveMeasurement('actualFG', $event.target.value)",
              :value="batchInfo.measurements.actualFG",
              label="Actual"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.abv
            abv-input(:value="abvInfo.target.abv", label="Target")
          .mx-1.abv
            abv-input(:value="abvInfo.actual.abv", label="Actual")
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

<script lang="ts">
import AbvInput from "/@client/components/abvInput.vue";
import GravityInput from "/@client/components/gravityInput.vue";
import { createBatchStore } from "/@client/store/batch";
import { useRequests } from "/@client/store/request";
import axios from "axios";
import { reactive, ref } from "vue";

interface Color {
  color: string;
}
export default {
  components: { AbvInput, GravityInput },
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
