// import { integer, pgTable, serial, text } from 'drizzle-orm/pg-core';
// import * as path from 'path';
// import * as schema from '../../schema';
// import {
//   fileRegex,
//   getConfigDefaults,
// } from '@dna-platform/crdt-lww-postgres/build/schema/system';
// const filename = path.basename(__filename).replace(fileRegex, '');
// const fields = {
//   id: text('id').primaryKey(),
//   entity_field_id: text('entity_field_id')
//     .references(() => (schema.entity_fields as Record<string, any>).id)
//     .notNull(),
//   permission_id: text('permission_id').references(
//     () => (schema.permissions as Record<string, any>).id,
//   ),
//   inherited_permission_id: text('inherited_permission_id')
//     .references(() => (schema.permissions as Record<string, any>).id)
//     .notNull(),
//   account_organization_id: text('account_organization_id')
//     .references(() => (schema.account_organizations as Record<string, any>).id)
//     .notNull(),
//   version: serial('version'),
//   created_by: text('created_by'),
//   updated_by: text('updated_by'),
//   deleted_by: text('deleted_by'),
//   timestamp: text('timestamp'),
//   tombstone: integer('tombstone').default(0),
// };
// const config = getConfigDefaults.byIndex(filename, fields);
// export const table = pgTable(filename, fields, config);
export {};
