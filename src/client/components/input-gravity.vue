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
      get: () => props.value.toFixed(3),
      set: (value) => emit("update:value", value),
    });
    return {
      localValue,
    };
  },
});
</script>

<style scoped lang="scss">
.specific-gravity {
  min-width: 4.9rem;
}
</style>
