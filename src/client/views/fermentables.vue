<template lang="pug">
.container
  .row
    .col-3
      button.btn.btn-success(@click="fileInput.click()", type="button") Import
        input.file-upload(
          @change="fileImport($event.target.name, $event.target.files)",
          type="file",
          ref="fileInput"
        )
    .col-3.offset-6
      input.form-control(v-model="tableFilter", placeholder="Filter")
  grid.align-middle(
    @row-button-click="fermentableStore.remove",
    :columns="columns",
    :rows="fermentableStore.items",
    :filterKey="tableFilter",
    :rowButton="{ class: 'trash', key: 'id', icon: 'trash' }"
  )
    template(v-slot:firstRow)
      td
        input.form-control(v-model="name", placeholder="Name")
      td
        input.form-control(v-model="country", placeholder="Country")
      td
        input.form-control(v-model="category", placeholder="Category")
      td
        input.form-control(v-model="kind", placeholder="Kind")
      td
        input.form-control(v-model="color", placeholder="Color")
      td
        input.form-control(v-model="ppg", placeholder="PPG")
      td
        button.btn.create(
          :class="pending ? '' : 'enabled'",
          @click="createFermentable"
        )
          fa(icon="plus")
</template>

<script lang="ts">
import Grid from "../components/grid.vue";
import { fermentablesStore } from "../store/fermentables";
import { useRequests } from "../store/request";
import { every } from "lodash-es";
import { ref } from "vue";

export default {
  components: { Grid },
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
