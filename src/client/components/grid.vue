<template lang="pug">
table.table.table-bordered
  thead
    tr
      th(
        v-for="key of columns",
        @click="sort(key)",
        :class="sortKey == key ? 'active': ''"
      )
        span.text-uppercase {{ key }}
        span.arrow(:class="desc ? 'desc': 'asc'")
      th(v-if="rowButton")
  tbody
    tr
      slot(name="firstRow")
    tr(v-for="entry of filteredRows")
      td(v-for="key in columns")
        span {{ entry[key] }}
      td(v-if="rowButton")
        button.btn(:class="rowButton.class", @click="clickAction(entry[rowButton.key])")
          fa(v-if="rowButton.icon", :icon="rowButton.icon")
</template>

<script lang="ts">
import { filter, reverse, sortBy } from "lodash-es";
import { computed, ref } from "vue";

export default {
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
      if (props.filterKey) {
        rows = filter(rows, (row) =>
          Object.keys(row).some(
            (key) =>
              String(row[key]).toLowerCase().indexOf(props.filterKey) > -1
          )
        );
      }
      if (sortKey.value) {
        rows = sortBy(rows, (row) => row[sortKey.value]);
      }
      if (!desc.value) {
        rows = reverse(rows);
      }
      return rows;
    });

    function sort(key) {
      if (key == sortKey.value) {
        desc.value = !desc.value;
      } else {
        sortKey.value = key;
        desc.value = true;
      }
    }
    function clickAction(key) {
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
};
</script>

<style scoped lang="scss">
@import "../scss/custom.scss";

th {
  @extend .text-secondary;
}

th :hover {
  @extend .text-dark;
}

th.active {
  @extend .text-dark;
}

th.active .arrow {
  opacity: 1;
}

.arrow {
  display: inline-block;
  vertical-align: middle;
  width: 0;
  height: 0;
  margin-left: 5px;
  opacity: 0;
}

.arrow.asc {
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-bottom: 4px solid;
}

.arrow.desc {
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-top: 4px solid;
}
</style>
