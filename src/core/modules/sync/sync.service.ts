import * as bluebird from 'bluebird';
import { ulid } from 'ulid';

import { Inject, Injectable, Logger } from '@nestjs/common';
import { MerklesService } from './merkles.service';
import { MessagesService } from './messages.service';

import { HLCService } from '../../modules/sync/hlc/hlc.service';
import { TransactionsService } from '../../modules/sync/transactions/transactions.service';
import { QueueService } from '../../modules/sync/transactions/queue.service';

import { ConfigSyncService } from '../config/config_sync.service';
import { StoreDriver } from '../drivers/store/store.driver';
import { StoreDriverInterface } from '../drivers/store/enums';
import { TransportDriver } from '../drivers/transport/transport.driver';
import { TransportDriverInterface } from '../drivers/transport/enums';
import { ExistingTransactionError } from './transactions/dto/ExistingTransaction.error';

import { SyncEndpointsService } from './sync_endpoints.service';
@Injectable()
export class SyncService {
  private _timer: any = null;
  private _sync_timer_ms = 20000;

  private GROUP_ID: string = '';
  private SYNC_ENABLED: string = '';
  private SYNC_TIMER_MS: string = '';

  @Inject(StoreDriverInterface) private readonly storeService: StoreDriver;
  @Inject(TransportDriverInterface)
  private readonly transportService: TransportDriver;

  private logger = new Logger(this.constructor.name);

  constructor(
    private readonly syncEndpointsService: SyncEndpointsService,
    private readonly configSyncService: ConfigSyncService,
    private readonly clockService: HLCService,
    private readonly merklesService: MerklesService,
    private readonly messagesService: MessagesService,
    private readonly transactionService: TransactionsService,
    private readonly queueService: QueueService,
  ) {}

  async onModuleInit() {
    this.GROUP_ID = await this.configSyncService.getConfig('GROUP_ID');
    this.SYNC_ENABLED = await this.configSyncService.getConfig('SYNC_ENABLED');
    this.logger.debug(`Sync Service Initialized`);
    await this.bgSync();
  }

  async insert(table: string, row: Record<string, any>) {
    const id = row.id ?? ulid();
    const messages = await this.messagesService.createMessages(table, row, id);
    await this.sendMessages(messages);
    return {
      ...row,
      id,
    };
  }

  async update(table: string, row: any, id) {
    const messages = await this.messagesService.createMessages(table, row, id);
    await this.sendMessages(messages);
    return {
      ...row,
      id,
    };
  }

  async delete(table: string, id) {
    const messages = await this.messagesService.createMessages(
      table,
      { tombstone: 1 },
      id,
    );
    await this.sendMessages(messages);
    return {
      id,
    };
  }

  async *iterateQueue(
    endpoints: Array<{ url: string; username: string; password: string }>,
  ) {
    while ((await this.queueService.size()) > 0) {
      const pack = ((await this.queueService.dequeue()) as {
        messages: Array<any>;
        since?: any | null;
        transaction_id?: string;
      }) ?? {
        messages: [],
        since: null,
      };

      await bluebird
        .mapSeries(
          endpoints,
          async (endpoint) =>
            await this.sync(
              pack.messages as any,
              pack.since,
              pack.transaction_id,
              endpoint,
            ).then(() => {
              return this.queueService.ack();
            }),
        )
        .catch(() => {
          return new Promise((resolve) =>
            setTimeout(resolve, this._sync_timer_ms),
          );
        });
      yield pack.messages;
    }
  }

  async bgSync() {
    const endpoints =
      (await this.syncEndpointsService.getSyncEndpoints()) as Array<any>;
    this.logger.debug('endpoints', endpoints);
    if (endpoints.length !== 0) {
      if ((await this.queueService.size()) == 0) {
        await bluebird.mapSeries(endpoints, async (endpoint) => {
          await this.sync([], null, undefined, endpoint).catch((error) => {
            if (error instanceof ExistingTransactionError) {
              this.logger.error(
                `Error in bgSync: Existing Transaction Detected`,
              );
            } else {
              this.logger.error(`Error in bgSync: ${error.message}`);
            }
          });
        });
      } else {
        for await (const _result of this.iterateQueue(endpoints)) {
        }
      }
    }

    this.SYNC_TIMER_MS =
      (await this.configSyncService.getConfig('SYNC_TIMER_MS')) || '60000';

    this._sync_timer_ms = +this.SYNC_TIMER_MS;

    this._timer = setTimeout(
      this.bgSync.bind(this),
      Math.floor(this._sync_timer_ms),
    );
  }

  async applyMessages(messages) {
    this.GROUP_ID = await this.configSyncService.getConfig('GROUP_ID');

    const existingMessages = await this.messagesService.compareMessages(
      messages,
    );
    await this.merklesService.startTransaction(async (tx) => {
      for (const msg of messages) {
        const existingMsg = existingMessages.get(msg);

        if (!existingMsg || existingMsg.timestamp < msg.timestamp) {
          await this.storeService.apply(msg);
        }

        if (!existingMsg || existingMsg.timestamp !== msg.timestamp) {
          const clock = await this.clockService.insertTimestamp(
            msg.timestamp,
            tx,
          );

          await this.messagesService.insertMessage(
            {
              group_id: this.GROUP_ID,
              client_id: clock.timestamp.node(),
              ...msg,
            },
            tx,
          );
        }
      }
    });
  }

  async sendMessages(messages) {
    if (this._timer) clearTimeout(this._timer);
    await this.applyMessages(messages);
    await this.queueService.enqueue({ messages, since: null });
    this._timer = setTimeout(
      this.bgSync.bind(this),
      Math.floor(this._sync_timer_ms * 0.25),
    );
  }

  async receiveMessages(messages) {
    const inner_messages = await this.merklesService.startTransaction(
      async (tx) => {
        return bluebird.mapSeries(messages, async (msg) => {
          await this.clockService.recv(msg.message.timestamp, tx);
          return msg.message;
        });
      },
    );

    await this.applyMessages(inner_messages);
  }

  async sync(
    initialMessages = [],
    since = null,
    existing_transaction_id?: string,
    opts?: any,
  ) {
    // check if sync is enabled
    this.SYNC_ENABLED = await this.configSyncService.getConfig('SYNC_ENABLED');
    if (this.SYNC_ENABLED !== 'true') {
      this.logger.debug(`Sync is disabled`);
      return;
    }

    this.GROUP_ID = await this.configSyncService.getConfig('GROUP_ID');

    const transaction_id = await this.transactionService.startTransaction(
      existing_transaction_id,
    );

    const clock = await this.clockService.getClock();
    this.logger.log(
      `Sync Attempt at ${new Date().toISOString()} since:${
        since || 'null'
      } messages:${initialMessages.length} transaction_id:${transaction_id}`,
    );

    let messages = initialMessages as any[];

    if (since) {
      const timestamp = this.clockService
        .createTimestamp(since, 0, '0')
        .toString();
      messages = await this.messagesService.getMessageSince(timestamp);
      this.logger.debug(
        `Since:${since} - ${timestamp} messages:${messages.length}`,
      );
    }

    let result;
    try {
      result = await this.transportService.post(
        {
          group_id: this.GROUP_ID,
          client_id: clock.timestamp.node(),
          messages,
          merkle: clock.merkle,
        },
        {
          url: opts.url,
          username: opts.username,
          password: opts.password,
        },
      );
    } catch (e) {
      this.logger.error(`Network Failure - ${e.message}`);
      return {
        messages: [],
        error: e,
      };
    }

    if (result.error) {
      this.logger.error(`Error in syncing to server`);
      await this.transactionService.stopTransaction(transaction_id);

      return;
    }
    if (result.messages.length > 0) {
      this.logger.debug(`${result.messages.length} updates recieved.`);
      await this.receiveMessages(result.messages);
      this.logger.log(
        `Synced ${result.messages.length} at ${new Date().toISOString()}`,
      );
    } else {
      this.logger.debug(`No new remote updates`);
    }

    const diffTime: any = await this.clockService.diff(result.merkle);
    if (diffTime) {
      this.logger.debug(
        `Timeline lag detected: since:${since} diff:${diffTime}`,
      );
      if (since && since === diffTime) {
        /*
          This error occurs when the time "since" is the same as the time "diffTime".
          This means that even after syncing up messages to server and syncing down messages from server, 
          the Merkle Tree is still not in sync. This is a bug that should not happen.


        */
        this.logger.error(
          `Clock Drift Detected - Adjusting Clocks and Retrying Sync`,
        );

        /*
         Given the message exchange has already occured, a workaround is disregard the local clock in favor of the server clock (result.merkle).
         Sync is then attempted again with the new clock, and the error should go away. 
        */
        await this.clockService.commitTree(result.merkle);
      }

      await this.sync([], diffTime, transaction_id, opts);
    }
    this.logger.log(`Sync done -  transaction_id:${transaction_id}`);
    await this.transactionService.stopTransaction(transaction_id);
  }
}
