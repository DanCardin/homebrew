<template lang="pug">
h3.text-center.text-lg.leading-6.font-medium.text-gray-900.mb-3.uppercase
  .relative.w-full Targets
    span.absolute.right-0(v-if="batchStore.unusedSections(batchId).length")
      span &nbsp;(&nbsp;
      button.font-normal.text-sm.uppercase.text-blue-500(
        v-for="s of batchStore.unusedSections(batchId)",
        @click="batchStore.enableSection(batchId, s.name)"
      ) {{ s.name }}&nbsp;
      span )

.flex.flex-nowrap.text-center.py-2.justify-evenly
  .text-tiny-heading(v-show="batchStore.sectionEnabled(batchId, 'og')")
    .relative OG
      button.w-4.h-4.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'og')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      input-gravity(
        @change.lazy="batchStore.saveMeasurement(batchId, 'target', 'og', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'target', 'og')",
        label="Target"
      )
    .mx-1
      input-gravity(
        @change.lazy="batchStore.saveMeasurement(batchId, 'actual', 'og', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'actual', 'og')",
        label="Actual"
      )

  .text-tiny-heading.text-secondary(
    v-show="batchStore.sectionEnabled(batchId, 'fg')"
  )
    .relative FG
      button.w-4.h-4.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'fg')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      input-gravity(
        @change.lazy="batchStore.saveMeasurement(batchId, 'target', 'fg', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'target', 'fg')",
        label="Target"
      )
    .mx-1
      input-gravity(
        @change.lazy="batchStore.saveMeasurement(batchId, 'actual', 'fg', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'actual', 'fg')",
        label="Actual"
      )

  .text-tiny-heading.text-secondary(
    v-show="batchStore.sectionEnabled(batchId, 'abv')"
  )
    .relative ABV
      button.w-4.h-4.float-right.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'abv')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      input-abv(
        :value="batchStore.getMeasurement(batchId, 'target', 'abv')",
        label="Target"
      )
    .mx-1
      input-abv(
        :value="batchStore.getMeasurement(batchId, 'actual', 'abv')",
        label="Actual"
      )

  .text-tiny-heading.text-secondary(
    v-show="batchStore.sectionEnabled(batchId, 'ibu')"
  )
    .relative IBU
      button.w-4.h-4.float-right.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'ibu')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      label.mb-0(for="targetIBUInput") Target
      input#targetIBUInput.ibu.block.w-full.px-1.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        type="number"
      )
    .mx-1
      label.mb-0(for="actualIBUInput") Actual
      input#actualIBUInput.ibu.block.w-full.px-1.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        type="number"
      )

  .text-tiny-heading.text-secondary(
    v-show="batchStore.sectionEnabled(batchId, 'srm')"
  )
    .relative SRM
      button.w-4.h-4.float-right.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'srm')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      label.mb-0(for="targetSRMInput") Target
      .relative.rounded-md.shadow-sm
        .absolute.inset-y-0.left-0.pl-3.flex.items-center.pointer-events-none
          .text-gray-500(class="sm:text-tiny-heading")
            beaker-icon.w-5.h-5(:style="targetSRMHex")
        input#targetSRMInput.srm.abv.block.w-full.pl-10.pr-1.text-sm.border-gray-300.rounded-md(
          class="focus:ring-indigo-500 focus:border-indigo-500",
          v-model="targetSRM",
          @input="targetSRMInput",
          type="number"
        )

    .mx-1
      label.mb-0(for="actualSRMInput") Actual
      .relative.rounded-md.shadow-sm
        .absolute.inset-y-0.left-0.pl-3.flex.items-center.pointer-events-none
          .text-gray-500(class="sm:text-tiny-heading")
            beaker-icon.w-5.h-5(:style="actualSRMHex")
        input#actualSRMInput.srm.abv.block.w-full.pl-10.pr-1.text-sm.border-gray-300.rounded-md(
          class="focus:ring-indigo-500 focus:border-indigo-500",
          v-model="actualSRM",
          @input="actualSRMInput",
          type="number"
        )

  .text-tiny-heading.text-secondary(
    v-show="batchStore.sectionEnabled(batchId, 'vol')"
  )
    .relative Volume
      button.w-4.h-4.float-right.mr-2.mt-1.absolute.right-0(
        @click="batchStore.disableSection(batchId, 'vol')"
      )
        trash-icon.w-full.h-full.text-red-500
    .mx-1
      input-volume(
        @change.lazy="batchStore.saveMeasurement(batchId, 'target', 'vol', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'target', 'vol')",
        label="Target"
      )
    .mx-1
      input-volume(
        @change.lazy="batchStore.saveMeasurement(batchId, 'actual', 'vol', $event.target.value)",
        :value="batchStore.getMeasurement(batchId, 'actual', 'vol')",
        label="Actual"
      )
</template>

<script lang="ts">
import InputAbv from "./input-abv.vue";
import InputGravity from "./input-gravity.vue";
import InputVolume from "./input-volume.vue";
import { useBatchStore } from "../store/batch";
import { useRequests } from "../store/request";
import axios from "axios";
import { reactive, ref, defineComponent, readonly, computed } from "vue";
import { BeakerIcon, TrashIcon } from "@heroicons/vue/outline";
import { filter, find } from "lodash-es";

interface Color {
  color: string;
}

export default defineComponent({
  components: { InputAbv, InputGravity, InputVolume, BeakerIcon, TrashIcon },
  props: {
    batchId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const batchStore = useBatchStore();
    await batchStore.fetch(props.batchId);

    const targetSRM = ref("");
    const actualSRM = ref("");
    const targetSRMHex = reactive<Color>({ color: "" });
    const actualSRMHex = reactive<Color>({ color: "" });

    const targetSRMInput = async () => {
      let color = "#000000";
      if (targetSRM.value) {
        const response = await axios.post("/api/measurement/srm/to_hex", {
          value: +targetSRM.value,
        });
        color = response.data.value;
      }
      targetSRMHex.color = color;
    };

    const actualSRMInput = async () => {
      let color = "#000000";
      if (actualSRM.value) {
        const response = await axios.post("/api/measurement/srm/to_hex", {
          value: +actualSRM.value,
        });
        color = response.data.value;
      }
      actualSRMHex.color = color;
    };

    return {
      actualSRMInput,
      targetSRMInput,
      targetSRM,
      actualSRM,
      targetSRMHex,
      actualSRMHex,
      batchStore,
    };
  },
});
</script>

<style scoped lang="scss">
.text-tiny-heading {
  font-size: 0.6em;
}
.ibu {
  min-width: 2.6rem;
}
.srm {
  min-width: 2.6rem;
}
</style>
