<template lang="pug">
label.mb-0 {{ label }}
input.form-control(
  :value="localValue",
  v-bind="$attrs",
  readonly,
  type="number",
  min="0",
  max="100",
  step=".01"
)
</template>

<script lang="ts">
import { computed } from "vue";

export default {
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
  setup(props, context) {
    const localValue = computed({
      get: () => props.value.toFixed(2),
      set: (value) => context.emit("update:modelValue", value),
    });
    return {
      localValue,
    };
  },
};
</script>
