<template lang="pug">
label.mb-0 {{ label }}
input.volume.abv.block.w-full.px-1.text-sm.border-gray-300.rounded-md(
  class="focus:ring-indigo-500 focus:border-indigo-500",
  v-model="localValue",
  v-bind="$attrs",
  type="number",
  min="0",
  max="100",
  step=".25"
)
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";

export default defineComponent({
  props: {
    value: {
      type: Number,
      default: 0,
    },
    label: {
      type: String,
      default: "",
    },
  },
  emits: ["update:value"],
  setup(props, { emit }) {
    const localValue = computed({
      get: () => props.value.toFixed(2),
      set: (value) => emit("update:value", value),
    });
    return {
      localValue,
    };
  },
});
</script>

<style scoped lang="scss">
.volume {
  min-width: 2.6rem;
}
</style>
