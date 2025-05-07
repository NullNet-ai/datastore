import { pgTable, serial, text } from 'drizzle-orm/pg-core';
import * as path from 'path';
import {
  fields as schema_fields,
  entities,
  permissions,
} from '@dna-platform/crdt-lww-postgres/build/schema';
import {
  fileRegex,
  getConfigDefaults,
} from '@dna-platform/crdt-lww-postgres/build/schema/system';
const filename = path.basename(__filename).replace(fileRegex, '');
const fields = {
  id: text('id').primaryKey(),
  entity_id: text('entity_id')
    .references(() => (entities as Record<string, any>).id)
    .notNull(),
  field_id: text('field_id')
    .references(() => (schema_fields as Record<string, any>).id)
    .notNull(),
  permission_id: text('permission_id').references(
    () => (permissions as Record<string, any>).id,
  ),
  inherited_permission_id: text('inherited_permission_id')
    .references(() => (permissions as Record<string, any>).id)
    .notNull(),
  // TODO: reference this to user with role tableei
  user_role_id: text('user_role_id').notNull(),
  version: serial('version').default(1),
  created_by: text('created_by'),
  updated_by: text('updated_by'),
  deleted_by: text('deleted_by'),
  timestamp: text('timestamp'),
};
const config = getConfigDefaults.byIndex(filename, fields);
export const table = pgTable(filename, fields, config);
