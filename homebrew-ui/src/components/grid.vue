<template lang="pug">
table.table.table-auto
  thead
    tr
      th.text-left(
        v-for="key of columns",
        :key="key",
        @click="sort(key)",
        :class="sortKey == key ? 'active' : ''"
      )
        span.uppercase {{ key }}
        span.arrow(:class="desc ? 'desc' : 'asc'")
      th(v-if="rowButton")
  tbody
    tr
      slot(name="firstRow")
    tr(v-for="entry of filteredRows")
      td(v-for="key in columns")
        span {{ entry[key] }}
      td(v-if="rowButton")
        button.btn(
          :class="rowButton.class",
          @click="clickAction(entry[rowButton.key])"
        )
          fa(v-if="rowButton.icon", :icon="rowButton.icon")
</template>

<script lang="ts">
import { filter, reverse, sortBy } from "lodash-es";
import { computed, ref, defineComponent } from "vue";

export default defineComponent({
  props: {
    rows: Array,
    columns: Array,
    filterKey: String,
    rowButton: Object,
  },
  emits: ["row-button-click"],
  setup(props, { emit }) {
    const sortKey = ref("");
    const desc = ref(true);

    const filteredRows = computed(() => {
      let rows = props.rows;
      let filterKey = props.filterKey;

      if (!rows) {
        return [];
      }

      if (filterKey) {
        rows = filter(rows, (row: Record<string, unknown>) =>
          Object.keys(row).some(
            (key) =>
              String(row[key])
                .toLowerCase()
                .indexOf(filterKey as string) > -1
          )
        );
      }
      if (sortKey.value) {
        rows = sortBy(
          rows,
          (row: Record<string, unknown>) => row[sortKey.value]
        );
      }
      if (!desc.value) {
        rows = reverse(rows);
      }
      return rows;
    });

    function sort(key: string) {
      if (key == sortKey.value) {
        desc.value = !desc.value;
      } else {
        sortKey.value = key;
        desc.value = true;
      }
    }
    function clickAction(key: unknown) {
      emit("row-button-click", key);
    }
    return {
      clickAction,
      desc,
      filteredRows,
      sort,
      sortKey,
    };
  },
});
</script>

<style scoped lang="scss">
// th {
//   @extend .text-secondary;
// }
//
// th :hover {
//   @extend .text-dark;
// }
//
// th.active {
//   @extend .text-dark;
// }
//
// th.active .arrow {
//   opacity: 1;
// }
//
// .arrow {
//   display: inline-block;
//   vertical-align: middle;
//   width: 0;
//   height: 0;
//   margin-left: 5px;
//   opacity: 0;
// }
//
// .arrow.asc {
//   border-left: 4px solid transparent;
//   border-right: 4px solid transparent;
//   border-bottom: 4px solid;
// }
//
// .arrow.desc {
//   border-left: 4px solid transparent;
//   border-right: 4px solid transparent;
//   border-top: 4px solid;
// }
</style>
