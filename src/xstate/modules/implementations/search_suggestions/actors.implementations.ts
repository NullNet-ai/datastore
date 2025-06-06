import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/search_suggestions/search_suggestions.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { Utility } from '../../../../utils/utility.service';
import { sql } from 'drizzle-orm';
import Bluebird from 'bluebird';
const pluralize = require('pluralize');

@Injectable()
export class SearchSuggestionsActorsImplementations {
  private db;
  constructor(
    private readonly drizzleService: DrizzleService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    searchSuggestions: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'No controller args found',
            count: 0,
            data: [],
          },
        });
      const { controller_args, responsible_account } = context;
      const [_res, _req] = controller_args;
      const { organization_id = '' } = responsible_account;
      const { table, type } = _req.params;
      const { time_zone } = _req.headers;

      const {
        offset = 0,
        limit = 50,
        advance_filters,
        joins,
        pluck_object,
        concatenate_fields = [],
        group_advance_filters = [],
        date_format,
        encrypted_fields = [],
      } = _req.body;

      if (!advance_filters?.length && !group_advance_filters?.length)
        return Promise.resolve({
          payload: {
            success: true,
            message: 'No advance or group filters provided',
            count: 0,
            data: [],
          },
        });
      const { table_schema } = Utility.checkTable(table);
      let aliased_joined_entities: Array<Record<string, any>> = [];
      joins.forEach(({ field_relation, type }) => {
        let to_entity = field_relation.to.entity;
        let to_alias = field_relation.to.alias;
        if (type === 'self') {
          to_entity = field_relation.from.entity;
          to_alias = field_relation.from.alias;
        }
        if (to_alias) {
          aliased_joined_entities.push({ alias: to_alias, to_entity });
        }
      });

      let filtered_fields = {};
      let search_term = '';
      let formatted_advance_filters: Array<Record<string, any>> = [];
      let formatted_group_advance_filters: Array<Record<string, any>> = [];
      if (group_advance_filters?.length) {
        // format entity names of group advance filters and get the filtered fields and search term
        formatted_group_advance_filters = group_advance_filters.map(
          (grouped_filters) => {
            const {
              formatted_filters,
              search_term: _search_term,
              filtered_fields: _filtered_fields,
            } = this.formatFilters({
              filters: grouped_filters.filters,
              aliased_joined_entities,
              table,
              filtered_fields,
              search_term,
            });

            filtered_fields = _filtered_fields;
            search_term = _search_term;
            return {
              ...grouped_filters,
              filters: formatted_filters,
            };
          },
        );
      } else {
        // format entity names of advance filters and get the filtered fields and search term
        const {
          formatted_filters,
          search_term: _search_term,
          filtered_fields: _filtered_fields,
        } = this.formatFilters({
          filters: advance_filters,
          aliased_joined_entities,
          table,
          filtered_fields,
          search_term,
        });
        formatted_advance_filters = formatted_filters;
        filtered_fields = _filtered_fields;
        search_term = _search_term;
      }

      const concatenated_field_expressions =
        Utility.generateConcatenatedExpressions(
          concatenate_fields,
          date_format,
          table,
        );
      // default FilterAnalyzer params
      const filter_analyzer_params = {
        table_schema,
        pluck_object,
        organization_id,
        joins,
        type,
        time_zone,
        table,
        date_format,
        concatenate_fields,
        concatenated_field_expressions,
      };
      const json_build_object_query = await Bluebird.reduce(
        Object.keys(filtered_fields),
        async (acc, entity) => {
          const field_object_agg = await Bluebird.map(
            await filtered_fields[entity],
            async (field: string) => {
              let entity_field = `${entity}.${field}`;
              let add_alias = false;
              let db_field_group = this.db;
              let db_field = this.db;
              // Generate the subquery for the field group
              db_field_group = db_field_group
                .select({ count: sql.raw(`COUNT(*) OVER()`) })
                .from(table_schema);

              let all_field_filters: Array<Record<string, any>> = [];
              let field_filter: Record<string, any> = {};
              let all_field_group_filters: Array<Record<string, any>> = [];
              if (group_advance_filters?.length) {
                // get all field search and the default filter from the group_advance_filters
                all_field_group_filters = formatted_group_advance_filters.map(
                  (grouped_filters) => {
                    const { all_field_filters, field_filter: _field_filter } =
                      this.getFieldFilters({
                        filters: grouped_filters.filters,
                        field,
                        entity,
                        search_term,
                      });
                    field_filter = Object.keys(field_filter)?.length
                      ? field_filter
                      : _field_filter;
                    return {
                      ...grouped_filters,
                      filters: all_field_filters,
                    };
                  },
                );
              } else {
                // get all field search and the default filter from the advance_filters
                const {
                  all_field_filters: _all_field_filters,
                  field_filter: _field_filter,
                } = this.getFieldFilters({
                  filters: formatted_advance_filters,
                  field,
                  entity,
                  search_term,
                });
                all_field_filters = _all_field_filters;
                field_filter = _field_filter;
              }

              const field_group_subquery = this.generateFieldSubquery(
                db_field_group,
                {
                  ...filter_analyzer_params,
                  // Pass the filter specific for the field and all default filters from portal
                  advance_filters: all_field_filters,
                  group_advance_filters: all_field_group_filters,
                },
              );

              // Concatenated expression for field
              const concatenated_field_exp =
                concatenated_field_expressions?.[entity]?.[field]?.expression;

              if (field.endsWith('_date')) {
                add_alias = true;
                entity_field = Utility.formatDate({
                  table: entity,
                  field,
                  date_format,
                  time_zone,
                  fields: pluck_object[entity],
                  encrypted_fields,
                }) as any;
              } else if (concatenated_field_exp) {
                add_alias = true;
                entity_field = concatenated_field_exp;
              }

              // Generate the subquery for the field
              db_field = db_field
                .select({
                  [entity]: sql.raw(
                    `${entity_field}${add_alias ? ` AS ${field}` : ''}`,
                  ),
                  count: sql.raw(`COUNT(*)`),
                })
                .from(table_schema);

              const field_subquery = this.generateFieldSubquery(db_field, {
                ...filter_analyzer_params,
                // Pass the filter specific for the field and all the default filters
                advance_filters: all_field_filters,
                group_advance_filters: all_field_group_filters,
              });

              // Handle grouping
              let group_by_entity_field = entity_field;

              group_by_entity_field = concatenated_field_exp
                ? concatenated_field_exp
                : group_by_entity_field;

              const group_count_query = `
                '${field}_group', (
                  SELECT COALESCE(
                    JSON_OBJECT_AGG('count', count),
                    JSON_BUILD_OBJECT('count', 0)
                  )
                  FROM (
                    ${field_group_subquery}
                    GROUP BY ${group_by_entity_field}
                  ) AS ${field}_group
                )`;

              const {
                operator,
                field: filtered_field,
                values,
                entity: filtered_entity,
                case_sensitive = false,
                parse_as,
              } = field_filter || {};

              // Generate the filter specific for the field to exclude the other filters
              const field_filter_query = Utility.evaluateFilter({
                operator,
                table_schema,
                field: filtered_field,
                values,
                entity: filtered_entity,
                aliased_entities: aliased_joined_entities.map(
                  ({ alias }) => alias,
                ),
                case_sensitive,
                parse_as,
                time_zone,
                date_format,
                concatenated_field_expressions,
                dz_filter_queue: [],
              });

              // Query for field with all the subquery and filters applied
              const db_field_obj_agg = this.db
                .select({
                  jsonObjectAgg: sql.raw(
                    `JSON_OBJECT_AGG(COALESCE(${field}::TEXT, 'null'), count)`,
                  ),
                })
                .from(
                  sql.raw(
                    `(${field_subquery} GROUP BY ${group_by_entity_field} OFFSET ${offset} LIMIT ${limit}) AS ${field}`,
                  ),
                )
                .where(field_filter_query);
              // Assigning query to field and replacing all filter value placeholder
              let field_query = `
                '${field}', (${Utility.replacePlaceholders(
                db_field_obj_agg.toSQL().sql,
                db_field_obj_agg.toSQL().params,
              )})`;

              // Modify the raw query filter to use the field alias instead of the entity field
              field_query = this.modifyQueryFilterString(
                field_query,
                field,
                parse_as,
              );
              return group_count_query + ', ' + field_query;
            },
          );
          return (
            acc +
            `${
              acc ? ', ' : ''
            }'${entity}', (SELECT JSON_BUILD_OBJECT(${field_object_agg.join(
              ', ',
            )}))`
          );
        },
        '',
      );

      const raw_query = sql.raw(
        `SELECT JSON_BUILD_OBJECT(${json_build_object_query}) AS results`,
      );
      const { rows } = await this.db.execute(raw_query);
      const [{ results }] = rows;

      return Promise.resolve({
        payload: {
          success: true,
          message: 'searchSuggestions Message',
          count: 0,
          data: [results],
        },
      });
    }),
  };

  private formatFilters({
    filters,
    aliased_joined_entities,
    table,
    filtered_fields,
    search_term,
  }) {
    const formatted_filters = filters.map((filter) => {
      const { type, entity, field, values, is_search = false } = filter;
      let filtered_entity = entity;
      const is_aliased = aliased_joined_entities?.find(
        ({ alias }) => alias === filtered_entity,
      );
      if (type === 'criteria' && is_search) search_term = values?.[0] || '';
      filtered_entity = is_aliased
        ? filtered_entity
        : pluralize(filtered_entity || table);

      if (type === 'criteria' && is_search)
        filtered_fields = {
          ...filtered_fields,
          [filtered_entity]: filtered_fields[filtered_entity]
            ? [...new Set([...filtered_fields[filtered_entity], field])]
            : [field],
        };
      return {
        ...filter,
        entity: filtered_entity,
      };
    });
    return { formatted_filters, search_term, filtered_fields };
  }

  private getFieldFilters({
    filters = [],
    field,
    entity,
    search_term,
  }: {
    filters: Array<Record<string, any>>;
    field: string;
    entity: string;
    search_term: string;
  }) {
    let all_field_filters: Array<Record<string, any>> = [];
    let field_filter: Record<string, any> = {};
    filters.forEach((filter, index) => {
      const {
        type,
        entity: filtered_entity,
        field: filtered_field,
        values,
        is_search = false,
      } = filter;

      const filtered_value = JSON.stringify(values);
      // if or/and operation and the last pushed was criteria
      // and (if next is criteria and not search term or previous is criteria and not search term)
      if (
        type === 'operator' &&
        all_field_filters[all_field_filters.length - 1]?.type === 'criteria' &&
        ((filters[index + 1]?.type === 'criteria' &&
          !filters[index + 1]?.is_search) ||
          (all_field_filters[all_field_filters.length - 1]?.type ===
            'criteria' &&
            !all_field_filters[all_field_filters.length - 1]?.is_search))
      )
        all_field_filters.push(filter);
      // if filter for the current field iterated
      else if (
        type === 'criteria' &&
        entity === filtered_entity &&
        field === filtered_field &&
        is_search
      ) {
        field_filter = filter;
        all_field_filters.push(filter);
      }
      // if not part of the or operation or not the search term
      // (possible additional filter on portal side during search)
      else if (
        type === 'criteria' &&
        filtered_value !== JSON.stringify([search_term]) &&
        !is_search
      )
        all_field_filters.push(filter);
    });
    return {
      all_field_filters,
      field_filter,
    };
  }

  private generateFieldSubquery(
    db,
    {
      table_schema,
      advance_filters,
      pluck_object,
      organization_id,
      joins,
      group_advance_filters,
      type,
      time_zone,
      table,
      date_format,
      concatenated_field_expressions,
    }: Record<string, any>,
  ) {
    db = Utility.FilterAnalyzer({
      db,
      table_schema,
      advance_filters,
      pluck_object,
      organization_id,
      joins,
      client_db: this.db,
      group_advance_filters,
      type,
      time_zone,
      table,
      date_format,
      concatenated_field_expressions,
    });
    return Utility.replacePlaceholders(db.toSQL().sql, db.toSQL().params);
  }

  private modifyQueryFilterString(query: string, field, parse_as: string) {
    const query_agg = query.split(` AS ${field} where `);
    let [filtered_field = '', value] = query_agg[1]?.split(' ilike ') || [];
    filtered_field = field;
    return (
      query_agg[0] +
      ` AS ${field} where ${filtered_field}${
        parse_as === 'text' ? '::TEXT' : ''
      } ilike ${value}`
    );
  }
}
