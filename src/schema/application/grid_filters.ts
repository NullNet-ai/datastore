// import {
//   fileRegex,
//   system_fields,
// } from '@dna-platform/crdt-lww-postgres/build/schema/system';
// import { index, integer, pgTable, text } from 'drizzle-orm/pg-core';
// import * as path from 'path';

// const filename = path.basename(__filename).replace(fileRegex, '');

// const fields = {
//   name: text(),
//   grid_id: text(),
//   link: text().default(''),
//   is_current: integer({
//     mode: 'boolean',
//   }).default(false),
//   is_default: integer({
//     mode: 'boolean',
//   }).default(false),
//   contact_id: text().references(() => (contacts as Record<string, any>).id),
//   entity: text(),
//   columns: text('columns', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   groups: text('groups', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   sorts: text('sorts', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   default_sorts: text('default_sorts', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   advance_filters: text('advance_filters', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   group_advance_filters: text('group_advance_filters', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
//   filter_groups: text('filter_groups', {
//     mode: 'json',
//   })
//     .$type<object[]>()
//     .default([]),
// };

// export const table = pgTable(
//   filename,
//   {
//     ...system_fields,
//     ...fields,
//   },
//   (table: Record<string, any>) => ({
//     ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
//       const searchable_fields = [
//         ...Object.keys(system_fields),
//         'name',
//         'grid_id',
//         'link',
//         'is_current',
//         'is_default',
//         'contact_id',
//         'entity',
//       ];
//       if (!searchable_fields.includes(field)) return acc;
//       const index_name = `grid_filters_${field}_idx`;
//       if (index_name.includes('_time')) return acc;
//       return {
//         ...acc,
//         [index_name]: index(index_name).on(table[field]),
//       };
//     }, {}),
//   }),
// );

export {};
