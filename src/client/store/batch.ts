import { keyBy, mapValues } from "lodash-es";
import { reactive, readonly } from "vue";

export interface BatchInfo {
  measurements: Record<string, number>;
}

export interface AbvInfo {
  measurements: Record<string, { abv: number }>;
}

export function createBatchStore(requests, batchId: number) {
  const batchInfo: reactive<BatchInfo> = reactive({
    measurements: { targetFG: 1, targetOG: 1 },
  });
  const abvInfo: reactive<AbvInfo> = reactive({
    target: { abv: 0 },
    actual: { abv: 0 },
  });

  async function calculateAbv(originalGravity: number, finalGravity: number) {
    const payload = { originalGravity, finalGravity };
    const { data } = await requests.post(
      "/api/measurement/abv/calculate",
      payload
    );
    return data;
  }

  async function fetch() {
    const { data } = await requests.post("/api/beer/batch/get", {
      batchId,
    });
    batchInfo.measurements = mapValues(
      keyBy(data.measurements, "name"),
      (v) => v.value
    );

    const targetAbv = await calculateAbv(
      batchInfo.measurements.targetOG,
      batchInfo.measurements.targetFG
    );
    abvInfo.target = targetAbv;

    const actualAbv = await calculateAbv(
      batchInfo.measurements.actualOG,
      batchInfo.measurements.actualFG
    );
    abvInfo.actual = actualAbv;
  }
  async function saveMeasurement(name: string, value: string) {
    const payload = { batchId, name, value: +value };
    await requests.post("/api/beer/batch/measurement/update", payload);
    await fetch();
  }
  return {
    fetch,
    saveMeasurement,
    abvInfo: readonly(abvInfo),
    batchInfo: readonly(batchInfo),
  };
}

export function createBatchIngredientStore(requests, batchId: number) {
  const ingredients = reactive({
    grain: [],
  });

  async function fetchIngredients(...kinds: string[]) {
    if (!kinds) {
      kinds = ["grain"];
    }
    for (const kind of kinds) {
      const { data } = await requests.post("/api/beer/batch/ingredient/list", {
        batchId,
        kind,
      });
      ingredients[kind] = data;
    }
  }

  async function newIngredient(kind: string, unit: string) {
    await requests.post("/api/beer/batch/ingredient/new", {
      batchId,
      kind,
      unit,
    });
    await fetchIngredients(kind);
  }

  async function deleteIngredient(kind: string, id: number) {
    await requests.post("/api/beer/batch/ingredient/delete", { id });
    await fetchIngredients(kind);
  }

  return {
    deleteIngredient,
    fetchIngredients,
    newIngredient,
    ingredients: readonly(ingredients),
  };
}
