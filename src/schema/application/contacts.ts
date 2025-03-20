// import * as path from 'path';

// import { addresses } from './index';
// import {
//   fileRegex,
//   system_fields,
// } from '@dna-platform/crdt-lww-postgres/build/schema/system';
// import { index, pgTable, text } from 'drizzle-orm/pg-core';

// const filename = path.basename(__filename).replace(fileRegex, '');

// const fields = {
//   first_name: text('first_name'),
//   middle_name: text('middle_name'),
//   last_name: text('last_name'),
//   date_of_birth: text('date_of_birth'),
//   address_id: text('address_id').references(() => (addresses as any).id),
// };

// export const table = pgTable(
//   filename,
//   {
//     ...system_fields,
//     ...fields,
//   },
//   (table: Record<string, any>) => ({
//     ...Object.keys({ ...system_fields, ...fields }).reduce((acc, field) => {
//       const index_name = `contacts_${field}_idx`;
//       if (index_name.includes('_time')) return acc;
//       return {
//         ...acc,
//         [index_name]: index(index_name).on(table[field]),
//       };
//     }, {}),
//   }),
// );
export {};
