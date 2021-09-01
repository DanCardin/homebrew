<template lang="pug">
h3.text-gray-900 Gravity Readings
.grid
  .flex.flex-row.items-stretch
    .flex-1
      Span Date
    .flex-1
      Span Hydrometer
    .flex-1
      Span Reading
  .flex.flex-row.items-stretch
    .flex-1
      button.bg-blue-500.hover.text-white.font-bold.py-2.px-4(
        @click="toggleEditing",
        v-show="!editing"
      )
        span.align-middle {{ date || 'YYYY-MM-DD' }}
      input.block.w-full.px-4.text-sm.border-gray-300.rounded-lg(
        class="focus:border-indigo-500 focus:ring-indigo-500",
        v-show="editing",
        @blur="changeDate",
        type="date"
      )
    .flex-1
      .relative.inline-block.w-10.mr-2.align-middle.select-none.transition.duration-200.ease-in
        input#toggle.toggle-checkbox.absolute.block.w-6.h-6.rounded-full.bg-white.border-4.appearance-none.cursor-pointer(
          type="checkbox",
          name="toggle"
        )
        label.toggle-label.block.overflow-hidden.h-6.w-12.rounded-full.bg-gray-300.cursor-pointer(
          for="toggle"
        )
    .flex-1
      input.block.w-full.px-4.text-sm.border-gray-300.rounded-lg(
        class="focus:border-indigo-500 focus:ring-indigo-500",
        type="number",
        placeholder="1.000"
      )
    .flex-1
      button.text-red-500.m-2.w-5(@click="deleteReading")
        trash-icon.w-full.h-full.text-red-500
.flex.px-2.flex-nowrap
  line-chart(v-bind="chartProps")
  line-chart(v-bind="chartProps")
</template>

<script lang="ts">
import { TrashIcon } from "@heroicons/vue/outline";
import { computed, defineComponent, ref } from "vue";

import { useBeerStore } from "../store/beer";
import { LineChart, useLineChart } from "vue-chart-3";

export default defineComponent({
  components: { TrashIcon, LineChart },

  props: {
    beerId: {
      type: Number,
      required: true,
    },
  },
  async setup(props) {
    const beerStore = useBeerStore();

    const data = ref([30, 40, 60, 70, 5]);

    const chartData = computed(() => ({
      labels: ["Paris", "Nîmes", "Toulon", "Perpignan", "Autre"],
      datasets: [
        {
          data: data.value,
          label: "My First Dataset",
          fill: false,
          borderColor: "rgb(75, 192, 192)",
          tension: 0.1,
        },
      ],
    }));

    const { lineChartProps } = useLineChart({ chartData });

    return {
      chartProps: lineChartProps,
    };
  },
});
</script>

<style lang="scss">
.toggle-checkbox:checked {
  @apply right-0 border-green-400;
  right: 0;
  border-color: #68d391;
}

.toggle-checkbox:checked + .toggle-label {
  @apply bg-green-400;
  background-color: #68d391;
}
</style>
