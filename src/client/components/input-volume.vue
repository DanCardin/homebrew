<template lang="pug">
label.mb-0 {{ label }}
input.volume.form-control(
  v-model="localValue",
  v-bind="$attrs",
  type="number",
  min="0",
  max="100",
  step=".25"
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
      get: () => props.value.toFixed(2),
      set: (value) => context.emit("update:value", value),
    });
    return {
      localValue,
    };
  },
};
</script>

<style scoped lang="scss">
.volume {
  min-width: 2.6rem;
}
</style>
