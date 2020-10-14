import axios from "axios";
import { computed, inject, provide, reactive } from "vue";

const createState = () => {
  const state = reactive({ pendingRequests: new Map() });
  const pendingRequests = state.pendingRequests;
  const pending = computed(
    () =>
      pendingRequests.size &&
      [...pendingRequests.values()].every((v) => v === true)
  );

  function set(key: string) {
    return state.pendingRequests.set(key, true);
  }

  function unset(key: string) {
    return state.pendingRequests.delete(key);
  }

  async function post<T>(
    url: string,
    body: unknown,
    options: Record<string, unknown>
  ): T {
    set(url);
    try {
      const response = await axios.post(url, body, options);
      return response;
    } finally {
      setTimeout(() => unset(url), 1000);
    }
  }

  return {
    post,
    pending,
  };
};

export const requestsSymbol = Symbol("request");
export const useRequests = () => inject(requestsSymbol);
export const state = createState();
export const provideRequests = () => provide(requestsSymbol, state);
