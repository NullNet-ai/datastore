import { index, text, integer } from 'drizzle-orm/sqlite-core';
export const system_fields = {
  id: text().primaryKey(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  version: integer().default(1),
  created_date: text(),
  created_time: text(),
  updated_date: text(),
  updated_time: text(),
};
export const getDefaultIndices = (table_name: string) => {
  return (table) => {
    return Object.keys(system_fields).reduce((acc, field) => {
      const index_name = `${table_name}_${field}_idx`;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {});
  };
};
