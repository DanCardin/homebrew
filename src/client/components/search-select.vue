<template lang="pug">
.dropdown
  input.dropdown-input(
    v-show="!selectedItem",
    :value="selectValue",
    @focus="onFocus",
    @blur="onBlur",
    @input="onInput($event.target.value)",
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
      span {{ itemDisplayValue(item) }}
</template>

<script lang="ts">
import getUnicodeFlagIcon from "country-flag-icons/unicode";
import { debounce } from "lodash-es";
import { fermentablesStore } from "../store/fermentables";
import { nextTick, reactive, ref } from "vue";

export default {
  props: {},
  setup(props, { emit }) {
    const { search } = fermentablesStore();

    const items = reactive([]);

    const selectInput: ref<HTMLInputElement> = ref(null);
    const dropdownList: ref<HTMLInputElement> = ref(null);
    const opened = ref(false);
    const selectedItem = ref(null);
    const selectValue = ref("");

    async function searchQuery(value) {
      const results = await search(value);
      items.splice(0, items.length, ...results);
    }
    searchQuery("");

    return {
      items,
      opened,
      selectInput,
      dropdownList,
      selectedItem,
      selectValue,
      onInput: debounce(searchQuery, 500),
      onFocus: async () => {
        opened.value = true;
      },
      onBlur: async (e) => {
        if (
          e.relatedTarget === null ||
          e.relatedTarget.parentElement !== dropdownList.value
        ) {
          opened.value = false;
        }
      },
      selectItem: (item) => {
        selectedItem.value = item;
        opened.value = false;
        selectValue.value = "";
        emit("update:modelValue", item);
      },
      resetSelection: async () => {
        selectedItem.value = null;
        opened.value = true;
        await nextTick();
        selectInput.value.focus();
        emit("update:modelValue", null);
      },
      itemDisplayValue: (item) => {
        return `${getUnicodeFlagIcon(item.country)} ${item.name}`;
      },
    };
  },
};
</script>

<style lang="scss">
.dropdown {
  position: relative;
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
  width: 100%;
  max-height: 500px;
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
