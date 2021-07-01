import axios from "axios";
import { defineStore } from 'pinia';

export interface Requests {
  pending: boolean;
  post<T>(
    url: string,
    body?: unknown,
    options?: Record<string, unknown>
  ): Promise<T>;
}

export const useRequests = defineStore({
  id: 'requests',
  state() {
    return {
      pendingRequests: new Map()
    }
  },
  getters: {
    pending(state) {
      return state.pendingRequests.size &&
        [...state.pendingRequests.values()].every((v) => v === true)
    }
  },
  actions: {
    set(key: string) {
      return this.pendingRequests.set(key, true);
    },

    unset(key: string) {
      return this.pendingRequests.delete(key);
    },

    async post<T>(
      url: string,
      body?: unknown,
      options?: Record<string, unknown>
    ): Promise<T> {
      this.set(url);
      try {
        const { data } = await axios.post<T>(url, body, options);
        return data;
      } finally {
        setTimeout(() => this.unset(url), 1000);
      }
    }
  }
})
