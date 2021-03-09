import { reactive, readonly, DeepReadonly } from "vue";
import type { Fermentable } from "../types/fermentable";
import { useRequests } from "./request";

interface BatchFermentable {
  batchId: number;
  fermentableId: number;
}

export interface BatchIngredientStore<T> {
  fetch(): T[];
  create(unit: string): undefined;
  remove(id: number): undefined;
  update(id: number, ingredientId: number, amount: number): undefined;

  items: DeepReadonly<T[]>;
}

export function batchIngredientStoreFactory<I>(kind: string) {
  return function (batchId: number): BatchIngredientStore<I> {
    const requests = useRequests();

    const items = reactive<I[]>([]);

    async function fetch() {
      const data = await requests.post<[]>(`/api/beer/batch/${kind}/list`, {
        batchId,
      });
      items.splice(0, items.length, ...data);
    }

    async function create(unit: string) {
      await requests.post<I>(`/api/beer/batch/${kind}/new`, {
        batchId,
        kind,
        unit,
      });
      await fetch();
    }

    async function remove(id: number) {
      await requests.post(`/api/beer/batch/${kind}/delete`, { id });
      await fetch();
    }

    async function update(id: number, ingredientId: number, amount: number) {
      await requests.post(`/api/beer/batch/${kind}/update`, {
        id,
        [`${kind}Id`]: ingredientId,
        amount,
      });
      await fetch();
    }

    return {
      remove,
      fetch,
      create,
      update,
      items: readonly(items),
    };
  };
}

export const createBatchFermentableStore = batchIngredientStoreFactory<
  BatchFermentable
>("fermentable");
