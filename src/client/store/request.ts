import axios from "axios";
import { computed, inject, provide, reactive } from "vue";

export interface Requests {
  pending: boolean;
  post<T>(
    url: string,
    body?: unknown,
    options?: Record<string, unknown>
  ): Promise<T>;
}

function createState() {
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
    body?: unknown,
    options?: Record<string, unknown>
  ): Promise<T> {
    set(url);
    try {
      const { data } = await axios.post<T>(url, body, options);
      return data;
    } finally {
      setTimeout(() => unset(url), 1000);
    }
  }

  return {
    post,
    pending,
  };
}

export const requestsSymbol = Symbol("request");
export const state = createState();
export const provideRequests = () => provide(requestsSymbol, state);
export const useRequests = () => {
  const requests = inject<Requests>(requestsSymbol);
  if (requests) {
    return requests;
  }
  return createState();
};
