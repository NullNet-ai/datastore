// import {
//   fileRegex,
//   system_fields,
// } from '@dna-platform/crdt-lww-postgres/build/schema/system';
// import { index, sqliteTable, text, real } from 'drizzle-orm/sqlite-core';
// import * as path from 'path';

// const filename = path.basename(__filename).replace(fileRegex, '');

// const fields = {
//   address: text(),
//   address_line_one: text(),
//   address_line_two: text(),
//   latitude: real(),
//   longitude: real(),
//   place_id: text(),
//   street_number: text(),
//   street: text(),
//   region: text(),
//   region_code: text(),
//   country_code: text(),
//   postal_code: text(),
//   country: text(),
//   state: text(),
//   city: text(),
// };

// export const table = sqliteTable(
//   filename,
//   {
//     ...system_fields,
//     ...fields,
//   },
//   (table: Record<string, any>) => ({
//     ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
//       const index_name = `addresses_${field}_idx`;
//       if (index_name.includes('_time')) return acc;
//       return {
//         ...acc,
//         [index_name]: index(index_name).on(table[field]),
//       };
//     }, {}),
//   }),
// );

export {};
