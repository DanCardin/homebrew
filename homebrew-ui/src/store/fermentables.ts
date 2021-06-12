import type { SearchQuery, Fermentable } from "../types/fermentable";
import { computed, reactive, readonly } from "vue";
import { keyBy } from "lodash-es";
import { useRequests } from "./request";

interface FermentableStore {
  items: Fermentable[];
  byId: Record<number, Fermentable>;
}

export function fermentablesStore() {
  const requests = useRequests();

  const state: FermentableStore = reactive({
    items: [],
    byId: computed(() => keyBy(state.items, "id")),
  });

  async function fetch() {
    const data = await requests.post("/api/fermentable/list", {});
    state.items.splice(0, state.items.length, ...data);
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

  async function search({ query, ids }: SearchQuery): Promise<Fermentable[]> {
    let payload = { query, ids };
    const data = await requests.post<Fermentable[]>(
      "/api/fermentable/search",
      payload
    );
    state.items.splice(0, state.items.length, ...data);
    return data;
  }

  function get(id: number) {
    return state.byId[id];
  }

  return {
    remove,
    fetch,
    create,
    bulkImport,
    search,
    get,
    items: readonly(state.items),
  };
}
