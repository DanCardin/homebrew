import { state } from "./request";
import { reactive, readonly } from "vue";
import { useRequests } from "./request";

interface Beer {
  id: number;
  name: string | undefined;
  style: string | undefined;
}

interface BatchMeasurement {
  name: string;
  value: number;
  type: string;
}

interface Beers extends Object {
  beers: Beer[];
  batches: [];
}

export interface BeerStore {
  beers: Beer[];
  batches: unknown[];
}

export function useBeerStore() {
  const requests = useRequests();

  const state = reactive<BeerStore>({
    beers: [],
    batches: [],
  });

  async function getBeers() {
    const data = await requests.post<Beer[]>("/api/beer/list", {});
    state.beers.splice(0, state.beers.length, ...data);
  }

  async function getBeer(id: number) {
    const data = await requests.post<Beer>("/api/beer/get", { id });
    return data;
  }

  async function newBeer() {
    const data = await requests.post<Beer>("/api/beer/new", {});
    return data;
  }

  async function update(beer: Beer) {
    const data = await requests.post("/api/beer/update", beer);
    return data;
  }

  async function createBatch(beerId: number) {
    await requests.post("/api/beer/batch/new", { beerId });
    await getBatches(beerId);
  }

  async function getBatches(beerId: number) {
    const data = await requests.post<Beer[]>("/api/beer/batch/list", {
      beerId,
    });
    state.batches.splice(0, state.batches.length, ...data);
  }

  async function updateBatchDate(
    beerId: number,
    batchId: number,
    date: string
  ) {
    const payload = { batchId, date };
    await requests.post("/api/beer/batch/date/update", payload);
    await getBatches(beerId);
  }

  async function deleteBatch(batchId: number) {
    const payload = { batchId };
    await requests.post("/api/beer/batch/delete", payload);
  }

  return {
    deleteBatch,
    updateBatchDate,
    getBatches,
    createBatch,
    update,
    newBeer,
    getBeers,
    getBeer,
    state: readonly(state),
  };
}
