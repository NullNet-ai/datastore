import { Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/transactions/transactions.schema';
import { DrizzleService } from '@dna-platform/crdt-lww-postgres';
import * as schema from '../../../../schema';
import { eq } from 'drizzle-orm';

@Injectable()
export class TransactionsActorsImplementations {
  constructor(private readonly drizzleService: DrizzleService) {}
  /**
   * Implementation of actors for the transactions machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    transactions: fromPromise(async ({ input }): Promise<IResponse> => {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: `No controller args found`,
            count: 0,
            data: [],
          },
        });

      const [_res, _req] = context?.controller_args;
      const table = schema['organizations'];
      const transactions = ['update'];
      await this.drizzleService.transaction(
        this.drizzleService.getClient(),
        async (_tx) => {
          for (const transaction of transactions) {
            await _tx[transaction](table)
              .set({ code: '1234' })
              .where(eq(table.id, '01JBHKXHYSKPP247HZZWHA3JCT'));
          }
        },
      );
      // const {} = _req.body;
      return Promise.resolve({
        payload: {
          success: true,
          message: 'transactions Message',
          count: 0,
          data: [_req.body],
        },
      });
    }),
  };
}
