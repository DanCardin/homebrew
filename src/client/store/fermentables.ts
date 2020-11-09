import type { SearchQuery, Fermentable } from "../types/fermentable";
import { reactive, readonly, computed } from "vue";
import { keyBy } from "lodash-es";
import { useRequests } from "./request";

export function fermentablesStore() {
  const fermentables = reactive([]);
  const byId = computed(() => keyBy(fermentables, "id"));

  const requests = useRequests();

  async function fetch() {
    const { data } = await requests.post("/api/fermentable/list", {});
    fermentables.splice(0, fermentables.length, ...data);
  }

  async function create(fermentable: Fermentable) {
    await requests.post("/api/fermentable/new", {
      name: fermentable.name,
      country: fermentable.country,
      category: fermentable.category,
      kind: fermentable.kind,
      color: fermentable.color,
      ppg: fermentable.ppg,
    });
    await fetch();
  }

  async function remove(id: number) {
    await requests.post("/api/fermentable/delete", { id });
    await fetch();
  }

  async function bulkImport(file: File) {
    const formData = new FormData();
    formData.append("file", file);
    requests.post("/api/fermentable/import", formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    await fetch();
  }

  async function search({ query, ids }: SearchQuery): unknown[] {
    const { data } = await requests.post("/api/fermentable/search", {
      query,
      ids,
    });
    fermentables.splice(0, fermentables.length, ...data);
    return data;
  }

  function get(id: number) {
    console.log(id);
    console.log(byId);
    console.log(byId[id]);
    return byId[id];
  }

  return {
    remove,
    fetch,
    create,
    bulkImport,
    search,
    get,
    items: readonly(fermentables),
  };
}
