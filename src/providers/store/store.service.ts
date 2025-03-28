import {
  BadRequestException,
  Injectable,
  NotFoundException,
} from '@nestjs/common';
import { Response, Request, Express } from 'express';
import * as fs from 'fs/promises';
import { each, mapSeries } from 'bluebird';
import { numeric, sqliteTable, text } from 'drizzle-orm/sqlite-core';
import { LoggerService, Machine } from '@dna-platform/common';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { getConfigDefaults } from '@dna-platform/crdt-lww-postgres/build/schema/system';
// import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
// import {
//   locale,
//   date_options,
//   timezone,
// } from '@dna-platform/crdt-lww-postgres/build/modules/constants';
import {
  EInitializer,
  IinitializerParams,
} from '../../xstate/modules/schemas/create/create.schema';
import { Utility } from '../../utils/utility.service';
import {
  counters,
  messages,
  organization_accounts,
  // contacts,
  // organizations,
  // external_contacts,
} from '../../schema';
import * as app_schema from '../../schema/application';
import * as schema from '../../schema';
import { desc, sql } from 'drizzle-orm';
import * as cache from 'memory-cache';
const pluralize = require('pluralize');
const {
  DEBUG = 'false',
  SCHEMA_RELATED_FIELD_DEPTH = '3',
  REDIS_CACHE_PORT = '6379',
  REDIS_CACHE_ENDPOINT = 'localhost',
  PORTAL_REDIS_CACHE_INDEX = '1',
} = process.env;
@Injectable()
export class StoreMutationDriver {
  @Machine('create')
  async create(_res: Response, _req: Request) {}

  @Machine('update')
  async update(_res: Response, _req: Request) {}

  @Machine('delete')
  async delete(_res: Response, _req: Request) {}

  @Machine('verify')
  async verify(_res: Response, _req: Request) {}

  @Machine('batchInsert')
  async batchInsert(_res: Response, _req: Request) {}

  @Machine('batchUpdate')
  async batchUpdate(_res: Response, _req: Request) {}

  @Machine('upload')
  async upload(_res: Response, _req: Request, _file: Express.Multer.File) {}

  @Machine('uploads')
  async uploads(
    _res: Response,
    _req: Request,
    _files: Array<Express.Multer.File>,
  ) {}

  @Machine('download')
  async download(_res: Response, _req: Request) {}

  @Machine('transactions')
  async transactions(_res: Response, _req: Request) {}

  @Machine('createHypertables')
  async createHypertables(_res: Response, _req: Request) {}
}

@Injectable()
export class StoreQueryDriver {
  @Machine('get')
  async get(_res: Response, _req: Request) {}

  @Machine('aggregationFilter')
  async aggregationFilter(_res: Response, _req: Request) {}

  @Machine('find')
  async find(_res: Response, _req: Request) {}

  @Machine('getFileById')
  async getFileById(_res: Response, _req: Request) {}

  @Machine('count')
  async getCount(_res: Response, _req: Request) {}
}

@Injectable()
export class InitializerService {
  private db;
  constructor(
    private drizzleService: DrizzleService,
    private logger: LoggerService, // private authService: AuthService,
  ) {
    this.db = this.drizzleService.getClient();
  }

  async create(initializer_type: EInitializer, params: IinitializerParams) {
    const { entity = '' } = params;
    let _params = params[initializer_type];
    switch (initializer_type) {
      case EInitializer.SYSTEM_CODE_CONFIG:
        if (!_params)
          throw new BadRequestException('Invalid System Code config');
        const system_config_result = await this.db
          .insert(counters)
          .values({ entity, counter: 1, ..._params })
          .returning({
            prefix: counters.prefix,
            default_code: counters.default_code,
            counter: counters.counter,
            digits_number: counters.digits_number,
          })
          .then(([{ prefix, default_code, counter, digits_number }]) => {
            const getDigit = (num: number) => {
              return num.toString().length;
            };
            if (digits_number) {
              digits_number = digits_number - getDigit(counter);
              const zero_digits =
                digits_number > 0 ? '0'.repeat(digits_number) : '';
              return prefix + (zero_digits + counter);
            }
            return prefix + (default_code + counter);
          })
          .catch(() => null);

        this.logger.debug(
          `System code config created: ${system_config_result}`,
        );
        break;
      // case EInitializer.ROOT_ACCOUNT_CONFIG:
      //   if (!entity)
      //     throw new BadRequestException(
      //       'Indicate entity for Root Account Configuration',
      //     );
      //   const root_account_id = '01JM3GTWCHR3CM2NP85C0Q2KN1';
      //   const account_id = 'root';
      //   const account_secret =
      //     process.env.ROOT_ACCOUNT_PASSWORD || 'pl3@s3ch@ng3m3!!';
      //   const hashed_password = await this.authService.passwordHash(
      //     account_secret,
      //   );
      //   const date = new Date();

      //   const [organization_accounts_counter = null] = await this.db
      //     .select()
      //     .from(counters)
      //     .where(sql`${counters.entity} = 'organization_accounts'`);
      //   const generateRootAccountCode = (entity_code: Record<string, any>) => {
      //     const root_count = 0;
      //     const { prefix, default_code } = entity_code;
      //     let { digits_number } = entity_code as Record<string, any>;
      //     const getDigit = (num: number) => {
      //       return num.toString().length;
      //     };

      //     if (digits_number) {
      //       digits_number = digits_number - getDigit(root_count);
      //       const zero_digits =
      //         digits_number > 0 ? '0'.repeat(digits_number) : '';
      //       return prefix + (zero_digits + root_count);
      //     }
      //     return prefix + (default_code + root_count);
      //   };

      //   const root_organization_account = {
      //     id: root_account_id,
      //     ...(organization_accounts_counter
      //       ? { code: generateRootAccountCode(organization_accounts_counter) }
      //       : {}),
      //     categories: ['Root'],
      //     account_id,
      //     email: account_id,
      //     password: hashed_password,
      //     account_secret: hashed_password,
      //     tombstone: 0,
      //     status: 'Active',
      //     timestamp: date.toISOString(),
      //     created_date: date.toLocaleDateString(locale, date_options),
      //     created_time: Utility.convertTime12to24(
      //       date.toLocaleTimeString(locale, { timeZone: timezone }),
      //     ),
      //     updated_date: date.toLocaleDateString(locale, date_options),
      //     updated_time: Utility.convertTime12to24(
      //       date.toLocaleTimeString(locale, { timeZone: timezone }),
      //     ),
      //     is_new_user: 0,
      //   };
      //   const result = await this.db
      //     .insert(organization_accounts)
      //     .values(root_organization_account)
      //     .returning({
      //       id: organization_accounts.id,
      //       account_id: organization_accounts.account_id,
      //       categories: organization_accounts.categories,
      //       status: organization_accounts.status,
      //     })
      //     .then(([account]) => account)
      //     .catch(() => null);

      //   this.logger.debug(`Root Account created: ${JSON.stringify(result)}`);
      //   break;
      default:
        throw new Error('Invalid initializer type');
    }
  }

  async generateSchema({
    include_crdt_tables = [],
    exclude_formatting_fields = [],
  }: {
    include_crdt_tables?: string[];
    exclude_formatting_fields?: string[];
  } = {}) {
    const tables = [...Object.keys(app_schema), ...include_crdt_tables];
    this.logger.log(`Generating application schema.`);

    const extractForeignKeys = (create_table_sql: string) => {
      const fk_regex = /FOREIGN KEY \(`(\w+)`\) REFERENCES `(\w+)`\(`(\w+)`\)/g;
      const foreign_keys: Record<string, any>[] = [];
      let match;
      while ((match = fk_regex.exec(create_table_sql)) !== null) {
        const [_, column, referenced_table] = match;
        foreign_keys.push({ column, referenced_table });
      }
      return foreign_keys;
    };

    const formatTableFields = async (
      table,
      field_name: string,
      exclude_formatting_fields: string[],
    ): Promise<Array<string>> => {
      let parent_field_name = field_name;
      if (parent_field_name.split('.').length > +SCHEMA_RELATED_FIELD_DEPTH)
        return [];
      if (!parent_field_name) parent_field_name = pluralize.singular(table);
      const table_schema = app_schema[table] ?? schema[table];
      if (!table_schema) {
        this.logger.error(
          `[generateSchema]: Table ${table} not found in schema`,
        );
        return [];
      }
      const fields = Object.keys(table_schema).filter(
        (table) => !exclude_formatting_fields.includes(table),
      );
      const cache_key = `${table}_schema_foreign_keys`;
      let foreign_keys = JSON.parse(cache.get(cache_key) || 'null');
      if (!foreign_keys) {
        const [table_sql] = await this.db.all(
          sql.raw(
            `SELECT sql FROM sqlite_master WHERE type='table' AND name='${table}'`,
          ),
        );
        const stringified_schema = table_sql.sql;
        foreign_keys = await extractForeignKeys(stringified_schema);

        cache.put(cache_key, JSON.stringify(foreign_keys), 5000);
      }
      const table_fields = await mapSeries(fields, async (field) => {
        let stringified_fields: Array<string> = [];
        const foreign = foreign_keys.find((fk) => fk.column === field);
        const new_field_name =
          parent_field_name.replace(/_id$/, '') +
          '.' +
          table_schema[field].name;
        if (foreign && foreign.referenced_table) {
          stringified_fields = [
            ...stringified_fields,
            new_field_name,
            ...(await formatTableFields(
              foreign.referenced_table,
              new_field_name,
              exclude_formatting_fields,
            )),
          ];
        } else {
          stringified_fields.push(new_field_name);
        }
        return stringified_fields;
      });
      return table_fields.flat();
    };

    const saveSchemaToRedis = async (schema: Record<string, any>) => {
      const { table_name } = schema;
      const hash_key = `schema:${table_name}`;

      const entries = Object.entries(schema).reduce((acc, [key, value]) => {
        return acc + `${key} '${value}' `;
      }, '');

      const redis_cli_connection_cmd = `redis-cli -p ${REDIS_CACHE_PORT} -h ${REDIS_CACHE_ENDPOINT} -n ${PORTAL_REDIS_CACHE_INDEX}`;
      const hash_set_cmd = `HSET ${hash_key} ${entries}`;
      const success = Utility.execCommand(
        redis_cli_connection_cmd + ' ' + hash_set_cmd,
      );
      if (success && DEBUG === 'true')
        this.logger.debug(`Successfully saved ${table_name} schema to Redis`);
    };

    await each(tables, async (table) => {
      await saveSchemaToRedis({
        table_name: table,
        column: JSON.stringify({}),
        constraint: JSON.stringify({}),
        index: JSON.stringify({}),
        formatted_with_related_fields: JSON.stringify(
          (await formatTableFields(table, '', exclude_formatting_fields)) || [],
        ),
      });
    });
  }
}

@Injectable()
export class SystemService {
  private db;
  constructor(private drizzleService: DrizzleService) {
    this.db = this.drizzleService.getClient();
  }

  async getPreviousStatus(dataset, record_id) {
    const result = await this.db
      .select({
        value: messages.value,
      })
      .from(messages)
      .where(
        sql`${messages.dataset} = ${dataset} AND ${messages.row} = ${record_id} AND column = 'status'`,
      )
      //! Fix issue
      //@ts-ignore - drizzle-orm inference issue
      .orderBy(desc(messages.timestamp))
      .offset(1)
      .limit(1);
    return {
      success: true,
      message: `Successfully got previous status of [${record_id}] from ${dataset}`,
      count: 1,
      data: result,
    };
  }
}

@Injectable()
export class CustomCreateService {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
  }

  async createContactEmail(body) {
    const table = 'contact_emails';
    const { schema }: any = Utility.checkCreateSchema(
      table,
      undefined as any,
      body,
    );

    body.code = await this.db
      .insert(counters)
      .values({ entity: table, counter: 1 })
      .onConflictDoUpdate({
        target: [counters.entity],
        set: {
          counter: sql`${counters.counter} + 1`,
        },
      })
      .returning({
        prefix: counters.prefix,
        default_code: counters.default_code,
        counter: counters.counter,
        // digits_number: counters.digits_number,
      })
      .then(([{ prefix, default_code, counter, digits_number }]) => {
        const getDigit = (num: number) => {
          return num.toString().length;
        };

        if (digits_number) {
          digits_number = digits_number - getDigit(counter);
          const zero_digits =
            digits_number > 0 ? '0'.repeat(digits_number) : '';
          return prefix + (zero_digits + counter);
        }
        return prefix + (default_code + counter);
      })
      .catch(() => null);

    const super_admin_id = '01JCSAG79KQ1WM0F9B47Q700P1';
    body.created_by = super_admin_id;
    const result = await this.syncService.insert(
      table,
      Utility.createParse({ schema, data: body }),
    );
    return Promise.resolve({
      payload: {
        success: true,
        message: `Successfully created in ${table}`,
        count: 1,
        data: [result],
      },
    });
  }
}

@Injectable()
export class DatabaseService {
  private db;
  private schema;
  private db_migrations_table = '__drizzle_migrations';
  private env = process.env.NODE_ENV || 'local';
  private drizzle_path = `drizzle`;
  private drizzle_meta_path = `${this.drizzle_path}/${this.env}/meta`;
  private drizzle_meta_journal = `${this.drizzle_meta_path}/_journal.json`;
  constructor(
    private logger: LoggerService,
    private drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
    this.schema = sqliteTable(
      this.db_migrations_table,
      {
        id: text().primaryKey(),
        hash: text().notNull(),
        created_at: numeric(),
      },
      getConfigDefaults.byIndex(this.db_migrations_table),
    );
  }

  async checkMigration(): Promise<{
    success: boolean;
    message: string;
    data: any;
  }> {
    try {
      let _db = this.db;
      await fs.readdir(this.drizzle_path).catch((err) => {
        throw new NotFoundException(err.message);
      });

      const journal_data = await fs
        .readFile(`${this.drizzle_meta_journal}`, 'utf8')
        .then((data) => {
          return JSON.parse(data);
        })
        .catch((err) => {
          throw new NotFoundException(err.message);
        });

      _db = _db.select().from(this.schema);
      this.logger.debug(`Query: ${JSON.stringify(_db.toSQL())}`);
      const results = await _db;
      if (!results || !results.length) {
        this.logger.debug('No drizzle migration records found.');
        return {
          success: false,
          message: 'No drizzle migration records found.',
          data: [],
        };
      }

      return {
        success: true,
        message: 'Successfully checked drizzle migrations',
        data: [
          {
            db_drizzle_migrations: results,
            [`${this.env}_drizzle_meta_journal`]: journal_data,
          },
        ],
      };
    } catch (error: any) {
      this.logger.error(error);
      return {
        success: false,
        message: error?.message,
        data: [],
      };
    }
  }

  async fixMigration() {
    const { data = [] } = await this.checkMigration();
    const {
      db_drizzle_migrations = [],
      [`${this.env}_drizzle_meta_journal`]: journal_data = {},
    } = data[0];

    db_drizzle_migrations.forEach(async (migration, index) => {
      const index_prefix = `${index}`.padStart(4, '0');
      if (!journal_data.entries[index]) {
        journal_data.entries.push({
          idx: index,
          version: '6',
          when: migration.created_at,
          tag: `${index_prefix}_store`,
          breakpoints: true,
        });
      } else if (journal_data.entries[index].when !== migration.created_at) {
        journal_data.entries[index].when = migration.created_at;
      }
    });
    await fs
      .writeFile(
        this.drizzle_meta_journal,
        JSON.stringify(journal_data, null, 2),
      )
      .catch((err) => {
        this.logger.error(err);
      });
  }

  checkMissingParams(required_params: string[], params: Record<string, any>) {
    const missing_params: string[] = [];
    required_params.forEach((param) => {
      if (!params[param]) {
        missing_params.push(param);
      }
    });
    return missing_params;
  }

  async updatePassword(params: Record<string, any>) {
    const { id, password } = params;
    return this.db
      .insert(organization_accounts)
      .values({ id, password })
      .onConflictDoUpdate({
        target: organization_accounts.id,
        set: { password, account_secret: password },
      });
  }
}
