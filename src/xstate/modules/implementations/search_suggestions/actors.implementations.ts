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
      } = _req.body;

      if (!advance_filters.length)
        return Promise.resolve({
          payload: {
            success: true,
            message: 'No advance filters provided',
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
      // format entity names of advance filters and get the filtered fields and search term
      const formatted_advance_filters: Array<Record<string, any>> =
        advance_filters.map((filter) => {
          const { type, entity, field, values, is_searched = false } = filter;
          let filtered_entity = entity;
          const is_aliased = aliased_joined_entities?.find(
            ({ alias }) => alias === filtered_entity,
          );
          if (type === 'criteria' && is_searched)
            search_term = values?.[0] || '';
          filtered_entity = is_aliased
            ? filtered_entity
            : pluralize(filtered_entity || table);
          if (type === 'criteria')
            filtered_fields = {
              ...filtered_fields,
              [filtered_entity]: filtered_fields[filtered_entity]
                ? [...filtered_fields[filtered_entity], field]
                : [field],
            };
          return {
            ...filter,
            entity: filtered_entity,
          };
        });

      const concatenated_field_expressions =
        Utility.generateConcatenatedExpressions(
          concatenate_fields,
          date_format,
          table,
        );
      const filter_analyzer_params = {
        table_schema,
        formatted_advance_filters,
        pluck_object,
        organization_id,
        joins,
        group_advance_filters,
        type,
        time_zone,
        table,
        date_format,
        concatenated_field_expressions,
      };
      const json_build_object_query = await Bluebird.reduce(
        Object.keys(filtered_fields),
        async (acc, entity) => {
          const field_object_agg = await Bluebird.map(
            await filtered_fields[entity],
            async (field: string) => {
              const entity_field = `${entity}.${field}`;
              let db_field_group = this.db;
              let db_field = this.db;
              // Generate the subquery for the field group
              db_field_group = db_field_group
                .select({ count: sql.raw(`COUNT(*) OVER()`) })
                .from(table_schema);

              let all_field_filters: Array<Record<string, any>> = [];

              let field_filter: Record<string, any> = {};
              formatted_advance_filters.forEach((filter) => {
                const {
                  type,
                  entity: filtered_entity,
                  field: filtered_field,
                  values,
                  is_searched = false,
                } = filter;
                const filtered_value = values?.[0];
                // if or/and operation and the last pushed was criteria
                if (
                  type === 'operator' &&
                  all_field_filters[all_field_filters.length - 1]?.type ===
                    'criteria'
                )
                  all_field_filters.push(filter);
                // if filter for the current field iterated
                else if (
                  type === 'criteria' &&
                  entity === filtered_entity &&
                  field === filtered_field &&
                  is_searched
                ) {
                  field_filter = filter;
                  all_field_filters.push(filter);
                }
                // if not part of the or operation or not the search term
                // (possible additional filter on portal side during search)
                else if (
                  type === 'criteria' &&
                  filtered_value !== search_term
                ) {
                  all_field_filters.push(filter);
                }
              });

              const field_group_subquery = this.generateFieldSubquery(
                db_field_group,
                {
                  ...filter_analyzer_params,
                  // Only pass the filter specific for the field and no other else to give the correct count
                  advance_filters: [field_filter],
                },
              );

              // Generate the subquery for the field
              db_field = db_field
                .select({
                  [entity]: sql.raw(entity_field),
                  count: sql.raw(`COUNT(*)`),
                })
                .from(table_schema);
              const field_subquery = this.generateFieldSubquery(db_field, {
                ...filter_analyzer_params,
                // Pass the filter specific for the field and all the default filters
                advance_filters: all_field_filters,
              });

              const group_count_query = `
                '${field}_group', (
                  SELECT COALESCE(
                    JSON_OBJECT_AGG('count', count),
                    JSON_BUILD_OBJECT('count', 0)
                  )
                  FROM (
                    ${field_group_subquery}
                    GROUP BY ${entity}.${field}
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
                    `(${field_subquery} GROUP BY ${entity}.${field} OFFSET ${offset} LIMIT ${limit}) AS ${field}`,
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
              field_query = this.modifyQueryFilterString(field_query, field);
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

      console.log("%c 🥚: SearchSuggestionsActorsImplementations -> json_build_object_query ", "font-size:16px;background-color:#f10bbe;color:white;", json_build_object_query)
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

  private modifyQueryFilterString(query: string, field) {
    const query_agg = query.split(` AS ${field} where `);
    let [filtered_field = '', value] = query_agg[1]?.split(' ilike ') || [];
    filtered_field = field;
    return (
      query_agg[0] + ` AS ${field} where ` + filtered_field + ' ilike ' + value
    );
  }
}
