import { Injectable } from '@nestjs/common';
import { Response, Request, Express } from 'express';
import { Machine } from '@dna-platform/common';
import { Utility } from '../../utils/utility.service';
import { DrizzleService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { counters, messages } from '../../schema';
import { desc, sql } from 'drizzle-orm';

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
