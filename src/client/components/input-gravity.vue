<template lang="pug">
label.mb-0 {{ label }}
input.specific-gravity.form-control(
  v-model="localValue",
  v-bind="$attrs",
  type="number",
  min="0",
  max="2",
  step=".001"
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
  emits: ["update:value"],
  setup(props, context) {
    const localValue = computed({
      get: () => props.value.toFixed(3),
      set: (value) => context.emit("update:value", value),
    });
    return {
      localValue,
    };
  },
};
</script>

<style scoped lang="scss">
.specific-gravity {
  min-width: 4.9rem;
}
</style>
