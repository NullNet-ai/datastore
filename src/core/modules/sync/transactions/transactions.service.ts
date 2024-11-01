import { Injectable } from '@nestjs/common';
import * as schema from '../../../../xstate/modules/schemas/drizzle';
import { v4 as uuidv4 } from 'uuid';
import { eq } from 'drizzle-orm';
import { ExistingTransactionError } from './dto/ExistingTransaction.error';
import { DrizzleService } from '../../../modules/drizzle/drizzle.service';

@Injectable()
export class TransactionsService {
  constructor(private readonly drizzleService: DrizzleService) {}

  async onModuleInit() {
    const db = this.drizzleService.getClient();
    const transactions = await db
      .select()
      .from(schema.transactions)
      .where(eq(schema.transactions.status, 'active'))
      .orderBy(schema.transactions.timestamp);

    await Promise.all(
      transactions.map(async (transaction) => {
        await this.expireTransaction(transaction.id);
      }),
    );
  }

  async expireTransaction(transaction_id: string) {
    const db = this.drizzleService.getClient();
    await db
      .delete(schema.transactions)
      .where(eq(schema.transactions.id, transaction_id));
  }

  async startTransaction(id?: string) {
    const db = this.drizzleService.getClient();

    const now = new Date().getTime();

    const transactions = await db
      .select()
      .from(schema.transactions)
      .where(eq(schema.transactions.status, 'active'))
      .orderBy(schema.transactions.timestamp)
      .limit(1);

    if (!transactions) throw new ExistingTransactionError();

    if (transactions.length > 0) {
      const transaction = transactions[0];

      if (id && transaction?.id === id) {
        // Extend the transaction
        await db
          .update(schema.transactions)
          .set({
            //@ts-ignore
            expiry: transaction.expiry + 2000,
          })
          .where(eq(schema.transactions.id, id));
        return transaction.id;
      }

      if (now > (transaction?.expiry as number)) {
        await this.expireTransaction(transaction?.id as string);
      } else {
        throw new ExistingTransactionError();
      }
    }

    const timestamp = new Date().toISOString();
    const expiry = now + 30000;

    const new_id = uuidv4() as string;
    //@ts-ignore
    await db.insert(schema.transactions).values({
      id: new_id,
      timestamp,
      status: 'active',
      expiry,
    });

    return new_id;
  }

  async stopTransaction(transaction_id: string) {
    await this.expireTransaction(transaction_id);
  }
}
