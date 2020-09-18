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
            input.form-control(
              v-model="targetOG"
              type="number"
              min="0"
              max="2"
              step=".001"
              id="targetInput"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualOG") Actual
            input.form-control(
              v-model="actualOG"
              type="number"
              min="0"
              max="2"
              step=".001"
              id="actualInput"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.specific-gravity
            label.mb-0(for="targetFG") Target
            input.form-control(
              v-model="targetFG"
              type="number"
              min="0"
              max="2"
              step=".001"
              id="targetInput"
            )
          .mx-1.specific-gravity
            label.mb-0(for="actualFG") Actual
            input.form-control(id="actualInput")
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1.abv
            label.mb-0(for="targetABV") Target
            input.form-control(id="targetInput" readonly v-bind:value="targetABV")
          .mx-1.abv
            label.mb-0(for="actualABV") Actual
            input.form-control(id="actualInput")
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetIBU") Target
            input.form-control.ibu(id="targetInput")
          .mx-1
            label.mb-0(for="actualIBU") Actual
            input.form-control.ibu(id="actualInput")
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            span(:style="targetSRMHex")
              fa(icon="beer")
            label.mb-0(for="targetSRM") Target
            input.form-control.srm(
              v-model="targetSRM"
              @input="targetSRMInput"
              id="targetInput"
            )
          .mx-1
            span(:style="actualSRMHex")
              fa(icon="beer")
            label.mb-0(for="actualSRM") Actual
            input.form-control.srm(
              v-model="actualSRM"
              @input="actualSRMInput"
              id="actualInput"
            )
      td.p-1
        .d-flex.text-sm.text-secondary
          .mx-1
            label.mb-0(for="targetVolume") Target
            input.form-control(
              id="targetInput"
            )
          .mx-1
            label.mb-0(for="actualVolume") Actual
            input.form-control(id="actualInput")
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import { Watch } from "vue-property-decorator";
import axios from "axios";

@Options({ components: {} })
export default class BeerTargets extends Vue {
  targetOG = "1";
  actualOG = "1";

  targetFG = "1";
  actualFG = "1";

  targetSRM = "";
  actualSRM = "";
  targetSRMHex = {};
  actualSRMHex = {};

  @Watch("targetOG")
  onTargetOGChanged(val: string, oldVal: string) {
    if (!this.actualOG || this.actualOG == oldVal) {
      this.actualOG = val;
    }
  }

  @Watch("targetFG")
  onTargetFGChanged(val: string, oldVal: string) {
    if (!this.actualFG || this.actualFG == oldVal) {
      this.actualFG = val;
    }
  }

  get targetABV() {
    const targetOG = +this.targetOG;
    const targetFG = +this.targetFG;
    return (
      ((76.08 * (targetOG - targetFG)) / (1.775 - targetOG)) *
      (targetFG / 0.794)
    );
  }

  async targetSRMInput() {
    let color = "#000000";
    if (this.targetSRM) {
      const response = await axios.post("/api/srm.convert", { value: +this.targetSRM });
      color = response.data.value;
    }
    this.targetSRMHex = { color };
  }

  async actualSRMInput() {
    let color = "#000000";
    if (this.actualSRM) {
      const response = await axios.post("/api/srm.convert", { value: +this.actualSRM });
      color = response.data.value;
    }
    this.actualSRMHex = { color };
  }
}
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
