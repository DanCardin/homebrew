<template lang="pug">
.grid.grid-cols-12
  .col-span-6
    input.form-control.w-full.px-4.text-sm.border-gray-300.rounded-md(
      class="focus:ring-indigo-500 focus:border-indigo-500",
      v-model="tableFilter",
      placeholder="Filter",
      type="text"
    )
  .col-start-12.col-span-1
    button.w-full.bg-blue-500.hover.text-white.font-bold.py-2.px-4(
      @click="fileInput.click()",
      type="button"
    ) Import
      input.file-upload(
        @change="fileImport($event.target.name, $event.target.files)",
        type="file",
        ref="fileInput"
      )
grid(
  @row-button-click="fermentableStore.remove",
  :columns="columns",
  :rows="fermentableStore.items",
  :filterKey="tableFilter",
  :rowButton="{ class: 'trash', key: 'id', icon: true }"
)
  template(v-slot:firstRow)
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="name",
        placeholder="Name",
        type="text"
      )
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="country",
        placeholder="Country",
        type="text"
      )
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="category",
        placeholder="Category",
        type="text"
      )
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="kind",
        placeholder="Kind",
        type="text"
      )
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="color",
        placeholder="Color",
        type="text"
      )
    td
      input.w-full.px-4.text-sm.border-gray-300.rounded-md(
        class="focus:ring-indigo-500 focus:border-indigo-500",
        v-model="ppg",
        placeholder="PPG",
        type="text"
      )
    td
      button.btn.create(
        :class="pending ? '' : 'enabled'",
        @click="createFermentable"
      )
        plus-icon.w-5.h-5
</template>

<script lang="ts">
import Grid from "../components/grid.vue";
import { fermentablesStore } from "../store/fermentables";
import { useRequests } from "../store/request";
import { every } from "lodash-es";
import { ref } from "vue";
import { PlusIcon } from "@heroicons/vue/outline";

export default {
  components: { Grid, PlusIcon },
  setup() {
    const fermentableStore = fermentablesStore();
    const tableFilter = ref("");
    const fileInput = ref<HTMLInputElement | null>(null);

    const requests = useRequests();
    const pending = requests ? requests.pending : false;

    const name = ref("");
    const country = ref("");
    const category = ref("");
    const kind = ref("");
    const color = ref("");
    const ppg = ref("");

    fermentableStore.fetch();

    async function createFermentable() {
      const values = [
        name.value,
        country.value,
        category.value,
        kind.value,
        color.value,
        ppg.value,
      ];
      if (!every(values, (v) => v !== "")) {
        await fermentableStore.create({
          name: name.value,
          country: country.value,
          category: category.value,
          kind: kind.value,
          color: +color.value,
          ppg: +ppg.value,
        });

        name.value = "";
        country.value = "";
        category.value = "";
        kind.value = "";
        color.value = "";
        ppg.value = "";
      }
    }
    async function fileImport(name: string, filesList: FileList) {
      if (filesList.length !== 1) {
        fileInput.value = null;
      }
      await fermentableStore.bulkImport(filesList[0]);
    }
    return {
      category,
      color,
      columns: ["name", "country", "category", "kind", "color", "ppg"],
      country,
      createFermentable,
      fileImport,
      fileInput,
      kind,
      name,
      pending,
      ppg,
      fermentableStore,
      tableFilter,
    };
  },
};
</script>

<style scoped lang="scss">
.file-upload {
  cursor: pointer;
  outline: none;
  position: absolute;
  top: 0;
  left: 0;
  width: 0;
  height: 0;
  overflow: hidden;
  opacity: 0;
}
</style>
