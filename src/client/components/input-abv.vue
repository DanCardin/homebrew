<template lang="pug">
label.mb-0 {{ label }}
input.abv.form-control(
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
      set: (value) => emit("update:modelValue", value),
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
