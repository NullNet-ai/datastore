export abstract class QueryDto {
  order_direction: 'asc' | 'desc';
  order_by: string;
  limit: string;
  offset: string;
  pluck: '';
  [key: string]: string;
}
