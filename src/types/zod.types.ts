import { z } from 'zod';

export const aggregationValidation = z.object({
  aggregation: z.enum(['SUM', 'AVG', 'COUNT']),
  aggregate_on: z.string().min(1, 'aggregate_on is required'),
  bucket_name: z.string().min(1, 'bucket_name is required'),
});

export const advanceFilterValidation = z.union([
  z.object({
    type: z.literal('criteria').default('criteria'),
    field: z.string().min(1, 'field is required'),
    operator: z.enum([
      'equal',
      'not_equal',
      'greater_than',
      'greater_than_or_equal',
      'less_than',
      'less_than_or_equal',
      'is_null',
      'is_not_null',
      'is_empty',
      'is_not_empty',
      'contains',
      'not_contains',
      'is_between',
      'is_not_between',
      'like',
    ]),
    values: z.array(z.union([z.string(), z.number()])), // Optional because some operators like 'is_null' or 'is_not_null' may not require a value
  }),
  z.object({
    type: z.literal('operator'),
    operator: z.enum(['and', 'or']),
  }),
]);

export const orderValidation = z.object({
  order_by: z.string(),
  order_direction: z.enum(['asc', 'desc']),
});

export const schema = z.object({
  entity: z.string().min(1, 'entity is required'),
  aggregations: z
    .array(aggregationValidation)
    .min(1, 'At least one aggregation is required'),
  advance_filters: z
    .array(advanceFilterValidation)
    .min(1, 'At least one filter is required'),
  bucket_size: z.string().min(1, 'bucket_size is required'),
  order: orderValidation,
});
