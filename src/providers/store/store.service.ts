import { BadRequestException, Injectable } from '@nestjs/common';
import { Response, Request, Express } from 'express';
import { each, mapSeries } from 'bluebird';
import { LoggerService, Machine } from '@dna-platform/common';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import {
  locale,
  date_options,
  timezone,
  formatter,
} from '@dna-platform/crdt-lww-postgres/build/modules/constants';
import {
  EInitializer,
  IinitializerParams,
} from '../../xstate/modules/schemas/create/create.schema';
import { Utility } from '../../utils/utility.service';
import {
  counters,
  messages,
  organization_accounts,
  contacts,
  organizations,
  external_contacts,
  account_organizations,
  accounts,
  account_profiles,
} from '../../schema';
import * as app_schema from '../../schema/application';
import * as schema from '../../schema';
import { desc, sql, eq, and, isNotNull } from 'drizzle-orm';
import * as cache from 'memory-cache';
import * as argon2 from 'argon2';
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
    private logger: LoggerService,
    private authService: AuthService,
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
      case EInitializer.ROOT_ACCOUNT_CONFIG:
        if (!entity)
          throw new BadRequestException(
            'Indicate entity for Root Account Configuration',
          );
        const root_account_id = '9c3ad11b-5b69-4bba-9858-3974d091b335';
        const account_id = 'root';
        const account_secret =
          process.env.ROOT_ACCOUNT_PASSWORD || 'pl3@s3ch@ng3m3!!';
        const hashed_password = await this.authService.passwordHash(
          account_secret,
        );
        const date = new Date();

        const [organization_accounts_counter = null] = await this.db
          .select()
          .from(counters)
          .where(sql`${counters.entity} = 'organization_accounts'`);
        const generateRootAccountCode = (entity_code: Record<string, any>) => {
          const root_count = 0;
          const { prefix, default_code } = entity_code;
          let { digits_number } = entity_code as Record<string, any>;
          const getDigit = (num: number) => {
            return num.toString().length;
          };

          if (digits_number) {
            digits_number = digits_number - getDigit(root_count);
            const zero_digits =
              digits_number > 0 ? '0'.repeat(digits_number) : '';
            return prefix + (zero_digits + root_count);
          }
          return prefix + (default_code + root_count);
        };

        const root_organization_account = {
          id: root_account_id,
          ...(organization_accounts_counter
            ? { code: generateRootAccountCode(organization_accounts_counter) }
            : {}),
          categories: ['Root'],
          account_id,
          email: account_id,
          password: hashed_password,
          account_secret: hashed_password,
          tombstone: 0,
          status: 'Active',
          timestamp: date.toISOString(),
          created_date: formatter(
            date.toLocaleDateString(locale, date_options),
          ),
          created_time: Utility.convertTime12to24(
            date.toLocaleTimeString(locale, { timeZone: timezone }),
          ),
          updated_date: formatter(
            date.toLocaleDateString(locale, date_options),
          ),
          updated_time: Utility.convertTime12to24(
            date.toLocaleTimeString(locale, { timeZone: timezone }),
          ),
          is_new_user: 0,
        };
        const result = await this.db
          .insert(organization_accounts)
          .values(root_organization_account)
          .returning({
            id: organization_accounts.id,
            account_id: organization_accounts.account_id,
            categories: organization_accounts.categories,
            status: organization_accounts.status,
          })
          .then(([account]) => account)
          .catch(() => null);

        this.logger.debug(`Root Account created: ${JSON.stringify(result)}`);
        break;
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

// @Injectable()
// export class DatabaseService {
// private db;
// private schema;
// private db_migrations_table = '__drizzle_migrations';
// private env = process.env.NODE_ENV || 'local';
// private drizzle_path = `drizzle`;
// private drizzle_meta_path = `${this.drizzle_path}/${this.env}/meta`;
// private drizzle_meta_journal = `${this.drizzle_meta_path}/_journal.json`;
// constructor(
// private logger: LoggerService,
// private drizzleService: DrizzleService,
// ) {
// this.db = this.drizzleService.getClient();
// this.schema = sqliteTable(
//   this.db_migrations_table,
//   {
//     id: text().primaryKey(),
//     hash: text().notNull(),
//     created_at: numeric(),
//   },
//   getConfigDefaults.byIndex(this.db_migrations_table),
// );
// }

// async checkMigration(): Promise<{
//   success: boolean;
//   message: string;
//   data: any;
// }> {
//   try {
//     let _db = this.db;
//     await fs.readdir(this.drizzle_path).catch((err) => {
//       throw new NotFoundException(err.message);
//     });

//     const journal_data = await fs
//       .readFile(`${this.drizzle_meta_journal}`, 'utf8')
//       .then((data) => {
//         return JSON.parse(data);
//       })
//       .catch((err) => {
//         throw new NotFoundException(err.message);
//       });

//     _db = _db.select().from(this.schema);
//     this.logger.debug(`Query: ${JSON.stringify(_db.toSQL())}`);
//     const results = await _db;
//     if (!results || !results.length) {
//       this.logger.debug('No drizzle migration records found.');
//       return {
//         success: false,
//         message: 'No drizzle migration records found.',
//         data: [],
//       };
//     }

//     return {
//       success: true,
//       message: 'Successfully checked drizzle migrations',
//       data: [
//         {
//           db_drizzle_migrations: results,
//           [`${this.env}_drizzle_meta_journal`]: journal_data,
//         },
//       ],
//     };
//   } catch (error: any) {
//     this.logger.error(error);
//     return {
//       success: false,
//       message: error?.message,
//       data: [],
//     };
//   }
// }

// async fixMigration() {
//   const { data = [] } = await this.checkMigration();
//   const {
//     db_drizzle_migrations = [],
//     [`${this.env}_drizzle_meta_journal`]: journal_data = {},
//   } = data[0];

//   db_drizzle_migrations.forEach(async (migration, index) => {
//     const index_prefix = `${index}`.padStart(4, '0');
//     if (!journal_data.entries[index]) {
//       journal_data.entries.push({
//         idx: index,
//         version: '6',
//         when: migration.created_at,
//         tag: `${index_prefix}_store`,
//         breakpoints: true,
//       });
//     } else if (journal_data.entries[index].when !== migration.created_at) {
//       journal_data.entries[index].when = migration.created_at;
//     }
//   });
//   await fs
//     .writeFile(
//       this.drizzle_meta_journal,
//       JSON.stringify(journal_data, null, 2),
//     )
//     .catch((err) => {
//       this.logger.error(err);
//     });
// }

// checkMissingParams(required_params: string[], params: Record<string, any>) {
//   const missing_params: string[] = [];
//   required_params.forEach((param) => {
//     if (!params[param]) {
//       missing_params.push(param);
//     }
//   });
//   return missing_params;
// }

// }

@Injectable()
export class RootStoreService {
  private db;
  constructor(
    private logger: LoggerService,
    private drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
  }

  async getAccount({
    email,
    organization_id,
    account_organization_id,
    account_id,
  }: {
    email: string;
    account_id?: string;
    organization_id?: string;
    account_organization_id?: string;
  }) {
    const filters = [
      eq((account_organizations as Record<string, any>).tombstone, 0),
      eq((account_organizations as Record<string, any>).status, 'Active'),
      eq((account_organizations as Record<string, any>).email, email),
      isNotNull((account_organizations as Record<string, any>).account_id),
    ];
    if (organization_id)
      filters.push(
        eq(
          (account_organizations as Record<string, any>).organization_id,
          organization_id,
        ),
      );

    if (account_organization_id)
      filters.push(
        eq(
          (account_organizations as Record<string, any>).id,
          account_organization_id,
        ),
      );

    if (account_id)
      filters.push(
        eq(
          (account_organizations as Record<string, any>).account_id,
          account_id,
        ),
      );

    const pluckFields = (table_schema: any, fields: string[]) => {
      return {
        ...fields.reduce((acc, field) => {
          return {
            ...acc,
            [field]: (table_schema as Record<string, any>)[field],
          };
        }, {}),
      };
    };

    let account = await this.db
      .select({
        profile: pluckFields(account_profiles, [
          'id',
          'first_name',
          'last_name',
          'email',
          'account_id',
          'categories',
          'code',
          'status',
          'organization_id',
        ]),
        contact: pluckFields(contacts, [
          'id',
          'first_name',
          'last_name',
          'account_id',
          'code',
          'categories',
          'status',
          'organization_id',
          'date_of_birth',
        ]),
        // device: devices,
        organization: pluckFields(organizations, [
          'id',
          'name',
          'code',
          'categories',
          'status',
          'organization_id',
          'parent_organization_id',
        ]),
        id: (accounts as Record<string, any>).id,
        account_id: (accounts as Record<string, any>).account_id,
        organization_id: (account_organizations as Record<string, any>)
          .organization_id,
        account_organization_id: (account_organizations as Record<string, any>)
          .id,
        account_status: (account_organizations as Record<string, any>)
          .account_organization_status,
        role_id: (account_organizations as Record<string, any>).role_id,
      })
      .from(account_organizations)
      .where(and(...filters))
      .leftJoin(
        accounts,
        eq(
          (accounts as Record<string, any>).id,
          (account_organizations as Record<string, any>).account_id,
        ),
      )
      .leftJoin(
        account_profiles,
        eq(
          (account_profiles as Record<string, any>).account_id,
          (accounts as Record<string, any>).id,
        ),
      )
      .leftJoin(
        contacts,
        eq(
          (contacts as Record<string, any>).id,
          (account_organizations as Record<string, any>).contact_id,
        ),
      )
      // .leftJoin(
      //   devices,
      //   eq(
      //     (devices as Record<string, any>).id,
      //     (account_organizations as Record<string, any>).device_id,
      //   ),
      // )
      .leftJoin(
        organizations,
        eq(
          (organizations as Record<string, any>).id,
          (account_organizations as Record<string, any>).organization_id,
        ),
      )
      .then(([account]) => {
        return account;
      })
      .catch((err) => {
        if (DEBUG === 'true') this.logger.error(err);
        return {};
      });

    if (!account) {
      const filters = [
        eq((accounts as Record<string, any>).tombstone, 0),
        eq((accounts as Record<string, any>).status, 'Active'),
        eq((accounts as Record<string, any>).account_id, account_id),
      ];
      if (organization_id)
        filters.push(
          eq(
            (accounts as Record<string, any>).organization_id,
            organization_id,
          ),
        );
      account = await this.db
        .select({
          profile: pluckFields(account_profiles, [
            'id',
            'first_name',
            'last_name',
            'email',
            'account_id',
            'categories',
            'code',
            'status',
            'organization_id',
          ]),
          organization: pluckFields(organizations, [
            'id',
            'name',
            'code',
            'categories',
            'status',
            'organization_id',
            'parent_organization_id',
          ]),
          id: (accounts as Record<string, any>).id,
          account_id: (accounts as Record<string, any>).account_id,
          organization_id: (accounts as Record<string, any>).organization_id,
          account_status: (accounts as Record<string, any>).account_status,
        })
        .from(accounts)
        .where(and(...filters))
        .leftJoin(
          account_profiles,
          eq(
            (account_profiles as Record<string, any>).account_id,
            (accounts as Record<string, any>).id,
          ),
        )
        .leftJoin(
          organizations,
          eq(
            (organizations as Record<string, any>).id,
            (accounts as Record<string, any>).organization_id,
          ),
        )
        .then(([account]) => {
          return {
            ...account,
            contact: {},
            device: {},
            account_organization_id: null,
            role_id: null,
          };
        })
        .catch((err) => {
          if (DEBUG === 'true') this.logger.error(err);
          return {};
        });
    }

    return account;
  }

  async getAccountOld(params: {
    account_id: string;
    return_account_secret?: boolean;
    organization_id?: string;
    contact_id?: string;
    organization_account_id?: string;
    is_external_user?: boolean;
  }) {
    const {
      account_id,
      return_account_secret = false,
      organization_id = '',
      contact_id = '',
      organization_account_id = '',
      is_external_user = false,
    } = params;

    const filters = [
      eq((organization_accounts as Record<string, any>).tombstone, 0),
      eq((organization_accounts as Record<string, any>).status, 'Active'),
      eq((organization_accounts as Record<string, any>).account_id, account_id),
    ];
    if (organization_id)
      filters.push(
        eq(
          (organization_accounts as Record<string, any>).organization_id,
          organization_id,
        ),
      );
    if (contact_id)
      filters.push(
        eq(
          is_external_user
            ? (organization_accounts as Record<string, any>).external_contact_id
            : (organization_accounts as Record<string, any>).contact_id,
          contact_id,
        ),
      );
    if (organization_account_id)
      filters.push(
        eq(
          (organization_accounts as Record<string, any>).id,
          organization_account_id,
        ),
      );

    return this.db
      .select({
        contact: contacts,
        external_contact: external_contacts,
        organization: organizations,
        organization_account_id: (organization_accounts as Record<string, any>)
          .id,
        organization_id: (organization_accounts as Record<string, any>)
          .organization_id,
        account_status: (organization_accounts as Record<string, any>)
          .account_status,
        account_id: (organization_accounts as Record<string, any>).account_id,
        ...(return_account_secret
          ? {
              account_secret: (organization_accounts as Record<string, any>)
                .account_secret,
            }
          : {}),
        categories: (organization_accounts as Record<string, any>).categories,
      })
      .from(organization_accounts)
      .where(and(...filters))
      .leftJoin(
        external_contacts,
        eq(
          (external_contacts as Record<string, any>).id,
          (organization_accounts as Record<string, any>).external_contact_id,
        ),
      )
      .leftJoin(
        contacts,
        eq(
          (contacts as Record<string, any>).id,
          (organization_accounts as Record<string, any>).contact_id,
        ),
      )
      .leftJoin(
        organizations,
        eq(
          (organizations as Record<string, any>).id,
          (organization_accounts as Record<string, any>).organization_id,
        ),
      )
      .then(([{ categories, external_contact, contact, ...account }]) => {
        const is_external_user = categories
          ?.map((category) => category.toLowerCase())
          ?.includes('external user');
        return {
          contact: is_external_user ? external_contact : contact,
          ...account,
          is_external_user,
        };
      });
  }

  async updatePassword(entity, params: Record<string, any>) {
    const { id, password } = params;
    const generated_password = await argon2.hash(password);
    const date = new Date();
    const updated_date = date.toLocaleDateString(locale, date_options);
    const updated_time = Utility.convertTime12to24(
      date.toLocaleTimeString(locale, { timeZone: timezone }),
    );
    const result = await this.db
      .update(schema[entity])
      .set({
        id,
        account_secret: generated_password,
        is_new_user: false,
        updated_date,
        updated_time,
      })
      .where(eq((schema[entity] as Record<string, any>).id, id));

    if (result.changes === 0 || result.rowCount === 0)
      throw new BadRequestException({
        success: false,
        message: `[updatePassword:${entity}]: No record for Account updated.`,
        count: 0,
        data: [],
      });

    return {
      success: true,
      message: `[updatePassword:${entity}]: Successfully updated password.`,
      count: result.changes,
      data: [
        {
          id,
          updated_date,
          updated_time,
        },
      ],
    };
  }
}
