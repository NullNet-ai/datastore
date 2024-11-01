export abstract class QueryDriver {
  abstract get(table: string, id: string, query?: any): Promise<any | null>;
  abstract find(table: string, query: any): Promise<{ query; data }>;
}
