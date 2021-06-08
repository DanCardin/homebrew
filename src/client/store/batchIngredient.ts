import { reactive, readonly, DeepReadonly } from "vue";
import type { Fermentable } from "../types/fermentable";
import { useRequests } from "./request";
import { defineStore } from 'pinia';

interface BatchFermentable {
  batchId: number;
  fermentableId: number;
}

function batchIngredientStoreFactory(kind: string) {
  return defineStore({
    id: kind,
    state() {
      return {
        batches: {},
      }
    },
    actions: {
      get(batchId: number) {
        return this.batches[batchId];
      },

      async fetch(batchId: number) {
        const requests = useRequests();
        const data = await requests.post(`/api/beer/batch/${kind}/list`, {
          batchId,
        });
        this.batches[batchId] = data;
      },

      async create(batchId: number, unit: string) {
        const requests = useRequests();
        await requests.post(`/api/beer/batch/${kind}/new`, {
          batchId,
          kind,
          unit,
        });
        await this.fetch(batchId);
      },

      async remove(batchId: number, id: number) {
        const requests = useRequests();
        await requests.post(`/api/beer/batch/${kind}/delete`, { id });
        await this.fetch(batchId);
      },

      async update(batchId: number, id: number, ingredientId: number, amount: number) {
        const requests = useRequests();
        await requests.post(`/api/beer/batch/${kind}/update`, {
          id,
          [`${kind}Id`]: ingredientId,
          amount,
        });
        await this.fetch(batchId);
      }
    }
  });
}

export const useBatchFermentableStore = batchIngredientStoreFactory("fermentable");
