import { filter, find, keyBy, mapValues } from "lodash-es";

import { reactive, readonly } from "vue";
import { Requests, useRequests } from "./request";
import { defineStore } from "pinia";

export const useBatchStore = defineStore({
  id: "batch",
  state() {
    return {
      batches: {},
    };
  },
  actions: {
    newBatch(batchId: number) {
      this.batches[batchId] = {
        sections: [
          { name: "og", enabled: false },
          { name: "fg", enabled: false },
          { name: "abv", enabled: false },
          { name: "ibu", enabled: false },
          { name: "srm", enabled: false },
          { name: "vol", enabled: false },
        ],
        measurements: {},
      };
      return this.batches[batchId];
    },
    unusedSections(batchId: number) {
      const batches = this.batches[batchId];
      const result = filter(
        batches.sections,
        (s) => !(this.hasSection(batchId, s.name) || s.enabled)
      );
      return result;
    },
    sectionEnabled(batchId: number, section: string) {
      const batches = this.batches[batchId];
      const result =
        this.hasSection(batchId, section) ||
        find(batches.sections, ["name", section]).enabled;
      return result;
    },
    disableSection(batchId: number, s: string) {
      const section = find(this.batches[batchId].sections, ["name", s]);
      if (section) {
        section.enabled = false;
      }
    },
    enableSection(batchId: number, s: string) {
      const section = find(this.batches[batchId].sections, ["name", s]);
      if (section) {
        section.enabled = true;
      }
    },
    hasSection(batchId: number, section: string) {
      const batch = this.batches[batchId];
      return (
        batch &&
        (batch.measurements[`target.${section}`] !== undefined ||
          batch.measurements[`actual.${section}`] !== undefined)
      );
    },
    getMeasurement(batchId: number, kind: string, name: string) {
      const key = `${kind}.${name}`;
      const batch = this.batches[batchId];
      const value = batch.measurements[key];
      if (value === undefined) {
        return 0;
      }
      return value;
    },
    async fetch(batchId: number) {
      const requests = useRequests();

      const data = await requests.post("/api/beer/batch/get", {
        batchId,
      });

      const batch = this.newBatch(batchId);
      batch.measurements = mapValues(
        keyBy(data.measurements, "name"),
        (v) => v.value
      );

      const targetAbv = await this.calculateAbv(
        batch.measurements["target.og"],
        batch.measurements["target.fg"]
      );
      batch.measurements["target.abv"] = targetAbv;

      const actualAbv = await this.calculateAbv(
        batch.measurements["actual.og"],
        batch.measurements["actual.fg"]
      );
      batch.measurements["actual.abv"] = actualAbv;
    },

    async calculateAbv(originalGravity: number, finalGravity: number) {
      const requests = useRequests();

      const payload = { originalGravity, finalGravity };
      const data = await requests.post(
        "/api/measurement/abv/calculate",
        payload
      );
      return data.abv;
    },

    async saveMeasurement(
      batchId: number,
      kind: string,
      name: string,
      value: string
    ) {
      const requests = useRequests();

      const payload = {
        batchId,
        name: `${kind}.${name}`,
        value: +value,
      };
      await requests.post("/api/beer/batch/measurement/update", payload);
      await this.fetch(batchId);
    },
  },
});
