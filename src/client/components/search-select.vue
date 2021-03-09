<template lang="pug">
.dropdown
  input.dropdown-input(
    v-show="!selectedItem",
    @focus="onFocus",
    @blur="onBlur",
    @input.stop="onInput($event.target.value)",
    type="text",
    placeholder="Find"
    ref="selectInput",
  )
  .dropdown-selected(v-show="!!selectedItem", @click="resetSelection")
    span {{ selectedItem && selectedItem.name }}
  .dropdown-list(v-show="opened", ref="dropdownList")
    .dropdown-item(
      v-for="item of items",
      :key="item.id",
      @click="selectItem(item)",
      tabindex="0"
    )
      slot(:item="item" name="row")
</template>

<script lang="ts">
import { PropType, defineComponent, nextTick, reactive, ref } from "vue";
import { debounce } from "lodash-es";

export default defineComponent({
  props: {
    modelValue: Object,
    search: {
      type: Function as PropType<(arg0: string) => object[]>,
      required: true,
    },
    selectedItem: {
      type: Object,
    },
  },
  emits: ["update:modelValue"],
  setup(props, { emit }) {
    const items = reactive<object[]>([]);

    const selectInput = ref<HTMLInputElement | null>(null);
    const dropdownList = ref<HTMLInputElement | null>(null);
    const opened = ref(false);
    // const selectedItem = ref(null);

    async function searchQuery(value: string) {
      const results = await props.search(value);
      items.splice(0, items.length, ...results);
    }

    return {
      items,
      opened,
      selectInput,
      dropdownList,
      // selectedItem,
      onInput: debounce(searchQuery, 500),
      onFocus: async () => {
        searchQuery("");
        opened.value = true;
      },
      onBlur: async (e: FocusEvent) => {
        if (
          e.relatedTarget === null ||
          (e.relatedTarget as HTMLElement).parentElement !== dropdownList.value
        ) {
          opened.value = false;
        }
      },
      selectItem: (item: unknown) => {
        // props.selectedItem.value = item;
        opened.value = false;

        emit("update:modelValue", item);
      },
      resetSelection: async () => {
        // props.selectedItem.value = null;
        opened.value = true;

        await nextTick();

        if (selectInput.value) {
          selectInput.value.focus();
        }

        emit("update:modelValue", null);
      },
    };
  },
});
</script>

<style lang="scss">
.dropdown {
  width: 100%;
  max-width: 400px;
  margin: 0 auto;
}

.dropdown-input,
.dropdown-selected {
  width: 100%;
  padding: 10px 16px;
  border: 1px solid transparent;
  background: #edf2f7;
  line-height: 1.5em;
  outline: none;
  border-radius: 8px;
}

.dropdown-input:focus,
.dropdown-selected:hover {
  background: #fff;
  border-color: #e2e8f0;
}

.dropdown-input::placeholder {
  opacity: 0.7;
}

.dropdown-selected {
  font-weight: bold;
  cursor: pointer;
}

.dropdown-list {
  position: absolute;
  z-index: 1;
  width: 100%;
  max-height: 20em;
  margin-top: 4px;
  overflow-y: scroll;
  background: #ffffff;
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1),
    0 4px 6px -2px rgba(0, 0, 0, 0.05);
  border-radius: 8px;
}

.dropdown-item {
  display: flex;
  width: 100%;
  padding: 11px 16px;
  cursor: pointer;
}

.dropdown-item:hover {
  background: #edf2f7;
}

.dropdown-item-flag {
  max-width: 24px;
  max-height: 18px;
  margin: auto 12px auto 0px;
}
</style>
