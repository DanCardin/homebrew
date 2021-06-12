import { keyBy, mapValues } from "lodash-es";
import { filter, find } from "lodash-es";
import { reactive, readonly } from "vue";
import { Requests, useRequests } from "./request";
import { defineStore } from 'pinia';

export const useNoteStore = defineStore({
  id: "note",
  state() {
    return {
      batches: {}
    };
  },
  actions: {
    get(batchId: number) {
      return this.batches[batchId];
    },

    async fetch(batchId: number) {
      const requests = useRequests();

      const data = await requests.post("/api/beer/batch/note/list", {
        batchId,
      });

      data.forEach((n) => {n.time = new Date(n.time).toLocaleString()});
      this.batches[batchId] = data;
    },

    async newNote(batchId: number, target: string, value: string) {
      const requests = useRequests();

      const payload = {
        batchId, target, value, time: new Date().toISOString().slice(0, -1),
      };
      await requests.post("/api/beer/batch/note/new", payload);
      await this.fetch(batchId);
    },

    async updateNote(batchId: number, note: string, time: string) {
      const requests = useRequests();

      const payload = {
        batchId, note, time
      };
      await requests.post("/api/beer/batch/note/update", payload);
      await this.fetch(batchId);
    },

    async deleteNote(batchId: number, noteId: number) {
      const requests = useRequests();

      const payload = {
        batchId, noteId,
      };
      await requests.post("/api/beer/batch/note/delete", payload);
      await this.fetch(batchId);
    }
  }
});
