import { state } from "./request";
import { defineStore } from 'pinia';
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

export const useBeerStore = defineStore({
  id: 'beer',
  state() {
    return {
      state: {
        beers: [],
        batches: [],
      }
    }
  },
  actions: {
    async getBeers() {
      const requests = useRequests();
      const data = await requests.post<Beer[]>("/api/beer/list", {});
      this.state.beers.splice(0, this.state.beers.length, ...data);
    },

    async getBeer(id: number) {
      const requests = useRequests();
      const data = await requests.post<Beer>("/api/beer/get", { id });
      return data;
    },

    async newBeer() {
      const requests = useRequests();
      const data = await requests.post<Beer>("/api/beer/new", {});
      return data;
    },

    async update(beer: Beer) {
      const requests = useRequests();
      const data = await requests.post("/api/beer/update", beer);
      return data;
    },

    async createBatch(beerId: number) {
      const requests = useRequests();
      await requests.post("/api/beer/batch/new", { beerId });
      await getBatches(beerId);
    },

    async getBatches(beerId: number) {
      const requests = useRequests();
      const data = await requests.post<Beer[]>("/api/beer/batch/list", {
        beerId,
      });
      this.state.batches.splice(0, this.state.batches.length, ...data);
    },

    async updateBatchDate(
      beerId: number,
      batchId: number,
      date: string
    ) {
      const requests = useRequests();
      const payload = { batchId, date };
      await requests.post("/api/beer/batch/date/update", payload);
      await getBatches(beerId);
    },

    async deleteBatch(batchId: number) {
      const requests = useRequests();
      const payload = { batchId };
      await requests.post("/api/beer/batch/delete", payload);
    },
  }
})
