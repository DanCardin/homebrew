import { useRequests } from "./request";
import { reactive, readonly } from "vue";

interface Fermentable {
  name: string;
  country: string;
  category: string;
  kind: string;
  color: number;
  ppg: number;
}

export function fermentablesStore() {
  const fermentables = reactive([]);

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

  async function search(query: string) {
    const { data } = await requests.post("/api/fermentable/search", { query });
    return data;
  }

  return {
    remove,
    fetch,
    create,
    bulkImport,
    search,
    fermentables: readonly(fermentables),
  };
}
