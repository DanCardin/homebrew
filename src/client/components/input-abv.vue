<template lang="pug">
label.mb-0 {{ label }}
.relative
  input.abv.block.w-full.px-1.text-sm.border-gray-300.rounded-md(
    class="focus:ring-0",
    :value="localValue",
    v-bind="$attrs",
    readonly,
    type="text",
    min="0",
    max="100",
    step=".01"
  )
  .text-gray-300.absolute.inset-y-1.right-2.text-lg %
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
  emits: ["update:modelValue"],
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
.abv {
  min-width: 3.5rem;
}
</style>
