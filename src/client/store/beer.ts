import { Store } from "/@client/store";
import { requestStore } from "/@client/store/request";

interface Beer {
  id: number;
  name: string | undefined;
  style: string | undefined;
}

interface Beers extends Object {
  beers: Beer[];
}

export class BeerStore extends Store<Beers> {
  protected data(): Beers {
    return {
      beers: [],
      batches: [],
    };
  }

  public async getBeers() {
    const { data } = await requestStore.post<Beer[]>("/api/beer.list", {});
    this.state.beers.splice(0, this.state.beers.length, ...data);
  }

  public async getBeer(id: number) {
    const { data } = await requestStore.post<Beer>("/api/beer.get", { id });
    return data;
  }

  public async newBeer() {
    const { data } = await requestStore.post("/api/beer.new", {});
    return data;
  }

  public async update(beer: Beer) {
    const { data } = await requestStore.post("/api/beer.update", beer);
    return data;
  }

  public async createBatch(beerId: number) {
    await requestStore.post("/api/beer.batch.new", { beerId });
    await this.getBatches(beerId);
  }

  public async getBatches(beerId: number) {
    const { data } = await requestStore.post("/api/beer.batch.list", {
      beerId,
    });
    this.state.batches.splice(0, this.state.batches.length, ...data);
  }

  public async updateBatchDate(beerId: number, batchId: number, date: string) {
    const payload = { batchId, date };
    await requestStore.post("/api/beer.batch.date.update", payload);
    await this.getBatches(beerId);
  }

  public async deleteBatch(batchId: number) {
    const payload = { batchId };
    await requestStore.post("/api/beer.batch.date.delete", payload);
  }
}

export const beerStore: BeerStore = new BeerStore();
