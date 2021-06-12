export interface Fermentable {
  name: string;
  country: string;
  category: string;
  kind: string;
  color: number;
  ppg: number;
}

export interface SearchQuery {
  query?: string;
  ids?: number[];
}
