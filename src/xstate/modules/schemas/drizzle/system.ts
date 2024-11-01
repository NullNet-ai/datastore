import { index, text, integer, primaryKey } from 'drizzle-orm/sqlite-core';
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
export const fileRegex = new RegExp(/.(ts|js)$/, 'gi');

const getDefaultIndices = (table_name: string) => {
  return (table) => {
    return Object.keys(system_fields).reduce((acc, field) => {
      const index_name = `${table_name}_${field}_idx`;
      if (index_name.includes('_time')) return acc;
      return {
        ...acc,
        [index_name]: index(index_name).on(table[field]),
      };
    }, {});
  };
};

const getPrimaryKey = (field_keys: string[]) => {
  return (table) => {
    return {
      pk: primaryKey({
        columns: field_keys.map((key) => table[key]) as any,
      }),
    };
  };
};

export const getConfigDefaults = {
  byIndex: getDefaultIndices,
  byPrimaryKey: getPrimaryKey,
};
