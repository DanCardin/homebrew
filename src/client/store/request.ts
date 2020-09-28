import { Store } from "/@client/store";
import axios from "axios";
import { computed } from "vue";

interface RequestState extends Object {
  pendingRequests: Map<string, boolean>;
}

export class RequestStore extends Store<RequestState> {
  protected data(): RequestState {
    return {
      pendingRequests: new Map(),
    };
  }

  public set(key: string) {
    return this.state.pendingRequests.set(key, true);
  }

  public unset(key: string) {
    return this.state.pendingRequests.delete(key);
  }

  public pending(): computed<boolean> {
    const pendingRequests = this.getState().pendingRequests;
    return computed(
      pendingRequests.size &&
        [...pendingRequests.values()].every((v) => v === true)
    );
  }

  public async post<T>(url: string, body: unknown): T {
    this.set(url);
    const response = await axios.post(url, body);
    setTimeout(() => this.unset(url), 1000);
    return response;
  }
}

export const requestStore: RequestStore = new RequestStore();
