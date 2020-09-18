import axios from "axios";
import { reactive, ref, readonly } from "vue";
import { Store } from "@/client/store";

interface Brews extends Object {
  brews: [];
}

export class BrewsStore {
  state: any;

  constructor() {
    this.state = reactive({ brews: ref([]) });
  }

  public getState() {
    return readonly(this.state);
  }

  async getBrews() {
    const { data } = await axios.post("/api/brew.list", {});
    this.state.brews = data;
  }

  async newBrew() {
    const { data } = await axios.post("/api/brew.new", {});
    return data;
  }
}

export const brewsStore: BrewsStore = new BrewsStore();
