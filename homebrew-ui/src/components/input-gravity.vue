<template lang="pug">
label.mb-0 {{ label }}
.relative
  note.absolute.inset-y-2.-left-3(:target="target", :batchId="batchId")
  input.specific-gravity.abv.block.w-full.px-1.text-sm.border-gray-300.rounded-md(
    class="focus:ring-indigo-500 focus:border-indigo-500",
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
import Note from "./note.vue";

export default defineComponent({
  components: { Note },
  props: {
    target: {
      type: String,
      required: true,
    },
    batchId: {
      type: Number,
      required: true,
    },
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
      get: () => (props.value || 1).toFixed(3),
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
