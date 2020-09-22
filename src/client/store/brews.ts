import { Store } from "/@client/store";
import axios from "axios";

interface Brew {
  id: number;
  name: string | undefined;
}

interface Brews extends Object {
  brews: Brew[];
}

export class BrewsStore extends Store<Brews> {
  protected data(): Brews {
    return {
      brews: [],
    };
  }

  public async getBrews() {
    const { data } = await axios.post<Brew[]>("/api/brew.list", {});
    this.state.brews.splice(0, 0, ...data);
  }

  public async newBrew() {
    const { data } = await axios.post("/api/brew.new", {});
    return data;
  }

  public async update(brew: Brew) {
    console.log(brew);
    const { data } = await axios.post("/api/brew.update", brew);
    return data;
  }
}

export const brewsStore: BrewsStore = new BrewsStore();
