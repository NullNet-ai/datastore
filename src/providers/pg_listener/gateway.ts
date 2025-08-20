import {
  WebSocketGateway,
  WebSocketServer,
  OnGatewayInit,
  OnGatewayConnection,
  OnGatewayDisconnect,
} from '@nestjs/websockets';
import { BadRequestException, Logger, OnModuleDestroy } from '@nestjs/common';
import { Server, Socket } from 'socket.io';
import { PgListenerService } from './pg_listener.service';
import { AuthService } from '@dna-platform/crdt-lww-postgres/build/modules/auth/auth.service';
import { CustomPassThrough } from './stream/CustomPassThrough';
import { QueueService } from './stream/queue.service';

@WebSocketGateway({
  cors: {
    origin: '*',
  },
})
export class NotificationsGateway
  implements
    OnGatewayInit,
    OnGatewayConnection,
    OnGatewayDisconnect,
    OnModuleDestroy
{
  @WebSocketServer() server!: Server;
  private readonly logger = new Logger(NotificationsGateway.name);
  private authenticatedClients = new Map<string, any>();
  private channelPipes = new Map<string, CustomPassThrough>();
  private backpressuredChannels = new Set<string>();
  private flushingChannels = new Set<string>();
  private isProcessing = false;
  private processingQueue: any[] = [];
  private savingPromises = new Map<string, Promise<void>>();
  private disconnected_clients = new Map<string, any[]>();

  constructor(
    private pgListenerService: PgListenerService,
    private readonly authService: AuthService,
    private readonly queueService: QueueService,
  ) {}

  async afterInit() {
    const mainPipe = this.pgListenerService.getMainStream();

    mainPipe.on('data', (payload) => {
      if (payload.event_name.startsWith('timeline_')) return;

      this.logger.debug(
        `Main stream data received: ${JSON.stringify(payload)}`,
      );

      this.processingQueue.push(payload);
      this.processNext(); // only starts if not already processing
    });

    this.logger.log('WebSocket Gateway initialized');
  }

  private async processNext() {
    if (this.isProcessing || this.processingQueue.length === 0) return;

    this.isProcessing = true;
    const payload: any = this.processingQueue.shift();

    if (!payload.event_name) {
      this.logger.error(
        'Received payload without event name, please revise your postgres function',
      );
    }

    /*
     * Receive a payload from the main stream, get its organization id and store it in the authenticatedClients map
     * AuthenticatedClients={
     * organization_id: {
     * client_ids: [client_id1, client_id2],
     * channels: [channel_name1, channel_name2]
     * }
     * }
     *
     * when client disconnects, we delete the client_id from the client_ids array but we don't delete the channels
     * if there is last channel remaining which disconnects, then we store the messages in the disconnected_clients map
     *  */

    try {
      const account_id = payload.organization_id;
      if (this.disconnected_clients.get(account_id)) {
        this.logger.debug(
          `Client ${account_id} is temporarily disconnected, adding message to the disconnected queue `,
        );
        this.disconnected_clients.set(account_id, [
          ...(this.disconnected_clients.get(account_id) || []),
          payload,
        ]);
        this.isProcessing = false;
        return;
      }

      if (!payload.type || !account_id) {
        this.logger.error(
          'Received payload without type or organization id, revise your postgres function',
        );
        return;
      }

      let channel_name = payload.event_name;

      const client = this.authenticatedClients.get(account_id);

      if (!client) {
        this.logger.debug(
          `No client connected to channel ${channel_name}, discarding message`,
        );
        return;
      }

      if (!client.channels.includes(channel_name)) {
        client.channels.push(channel_name);
      }
      const channelPipe = this.getOrCreateChannelPipe(channel_name);

      if (
        this.flushingChannels.has(channel_name) ||
        this.backpressuredChannels.has(channel_name)
      ) {
        const savePromise = this.saveToQueue(channel_name, payload);
        this.savingPromises.set(channel_name, savePromise);

        await savePromise;
        this.logger.debug(
          `Queued message for ${channel_name} (flushing or backpressured)`,
        );
      } else if (!channelPipe.write(payload)) {
        this.logger.warn(
          `Backpressure on ${channel_name}:${payload.id}, adding channel to backpressure set`,
        );
        this.backpressuredChannels.add(channel_name);
      }
    } catch (error: any) {
      this.logger.error(`Error processing data: ${error.message}`);
    } finally {
      this.isProcessing = false;
      setImmediate(() => this.processNext()); // Process the next item
    }
  }
  private async updateHighWaterMark(
    client: Socket,
    user_id: string,
    channel_name: string,
    highWaterMark: number,
  ) {
    const clientData = this.authenticatedClients.get(user_id);
    if (!clientData) {
      this.logger.warn(`Client ${client.id} is not authenticated`);
      return;
    }

    if (!clientData.channels.includes(channel_name)) {
      this.logger.warn(
        `Channel ${channel_name} does not exist for client ${client.id}`,
      );
      return;
    }

    const channelPipe = this.channelPipes.get(channel_name);
    if (channelPipe) {
      channelPipe.setHighWaterMark(highWaterMark);
      this.logger.log(
        `Updated highWaterMark for channel ${channel_name} to ${highWaterMark}`,
      );
    } else {
      this.logger.warn(`Channel pipe for ${channel_name} not found`);
    }
  }

  private async sendCurrentHighWaterMark(
    client: Socket,
    user_id: string,
    channel_name: string,
  ) {
    const clientData = this.authenticatedClients.get(user_id);
    if (!clientData) {
      this.logger.warn(`Client ${client.id} is not authenticated`);
      return;
    }

    if (!clientData.channels.includes(channel_name)) {
      this.logger.warn(
        `Channel ${channel_name} does not exist for client ${client.id}`,
      );
      return;
    }

    const channelPipe = this.channelPipes.get(channel_name);
    if (channelPipe) {
      const currentHighWaterMark = channelPipe.getHighWaterMark();
      client.emit('currentHighWaterMark', {
        channel_name,
        currentHighWaterMark,
      });
      this.logger.log(
        `Sent current highWaterMark for channel ${channel_name} to client ${client.id}`,
      );
    } else {
      this.logger.warn(`Channel pipe for ${channel_name} not found`);
    }
  }

  async handleConnection(client: Socket) {
    try {
      const token =
        client.handshake.auth.token || client.handshake.headers.authorization;

      if (!token) {
        this.logger.warn(
          `Client ${client.id} attempted connection without token`,
        );
        client.disconnect();
        return;
      }

      const { account: responsible_account } = await this.authService
        .verify(token?.replace('Bearer ', ''))
        .catch((err) => {
          this.logger.debug(err.message);
          throw new BadRequestException(
            `Token Verification Failed: ${err.message}`,
          );
        });

      let user_id = responsible_account.organization_id;

      this.authenticatedClients.set(user_id, {
        client_ids: [
          client.id,
          ...(this.authenticatedClients.get(user_id)?.client_ids || []),
        ],
        channels: this.authenticatedClients.get(user_id)?.channels || [],
      });
      //push everything to the start of processingQueue
      this.processingQueue.unshift(
        ...(this.disconnected_clients.get(user_id) || []),
      );

      this.disconnected_clients.delete(user_id);
      setImmediate(() => this.processNext());
      this.logger.log(
        `Client ${client.id} authenticated successfully (User: ${user_id})`,
      );

      client.on(
        'updateHighWaterMark',
        async ({ channel_name, highWaterMark }) => {
          await this.updateHighWaterMark(
            client,
            user_id,
            channel_name,
            highWaterMark,
          );
        },
      );

      // Listen for the event to get current highWaterMark
      client.on('getCurrentHighWaterMark', async ({ channel_name }) => {
        await this.sendCurrentHighWaterMark(client, user_id, channel_name);
      });
    } catch (error: any) {
      this.logger.error(`Authentication failed: ${error.message}`);
      client.disconnect();
    }
  }
  private async processChunkSequentially(channelName: string, chunk: any) {
    this.server.emit(channelName, chunk); // Assume promisified emit
    this.logger.debug(`Emitted chunk to channel ${channelName}: ${chunk.id}`);
  }

  private getOrCreateChannelPipe(channelName: string): CustomPassThrough {
    if (!this.channelPipes.has(channelName)) {
      const pipe = new CustomPassThrough({
        highWaterMark: 2,
        objectMode: true,
      });

      // Set up listeners for this pipe
      pipe.on('readable', () => {
        let chunk;
        while ((chunk = pipe.read()) !== null) {
          void this.processChunkSequentially(channelName, chunk);
        }
      });

      pipe.on('drain', async () => {
        this.logger.debug(`Channel pipe ${channelName} drained`);

        // Remove from backpressured set to resume receiving datra
        if (this.backpressuredChannels.has(channelName)) {
          this.backpressuredChannels.delete(channelName);
          this.logger.debug(`Resumed receiving data for ${channelName}`);
          await this.flushChannelQueue(channelName);
        }
      });

      this.channelPipes.set(channelName, pipe);
    }

    return this.channelPipes.get(channelName)!;
  }

  private async flushChannelQueue(channelName: string): Promise<void> {
    if (this.flushingChannels.has(channelName)) {
      return; // Already flushing
    }

    this.flushingChannels.add(channelName);

    const has_promise = this.savingPromises.get(channelName);

    try {
      const pipe = this.channelPipes.get(channelName);
      if (!pipe) {
        this.flushingChannels.delete(channelName);
        return;
      }
      if (has_promise) {
        this.logger.debug('Waiting for previous save to queue to finish');
        await has_promise;
        this.savingPromises.delete(channelName);
      }
      // Get items from the queue (with a reasonable batch size)
      const queueItems = await this.queueService.getQueueItems(
        channelName,
        5000,
      );
      if (queueItems.length === 0) {
        this.logger.debug(`No queued items for channel ${channelName}`);
        this.flushingChannels.delete(channelName);
        return;
      }

      this.logger.debug(
        `Flushing ${queueItems.length} messages for channel ${channelName}`,
      );

      let messagesProcessed = 0;
      const itemsToDelete: any = [];

      for (const item of queueItems) {
        const written = pipe.write(item.content);
        itemsToDelete.push(item.id);
        messagesProcessed++;
        await this.queueService.deleteFromQueue(item.id);
        if (!written) {
          // Backpressure detected
          this.backpressuredChannels.add(channelName);
          this.flushingChannels.delete(channelName);
          this.logger.warn(
            `Channel ${channelName} backpressured again on ${item.id} during queue flush after processing ${messagesProcessed} messages`,
          );
          return;
        }
      }

      // Delete all processed items
      await Promise.all(
        itemsToDelete.map((id) => this.queueService.deleteFromQueue(id)),
      );

      this.logger.debug(
        `Processed ${messagesProcessed} messages from queue for channel ${channelName}`,
      );

      // Check if there are more items to process
      const remainingItems = await this.queueService.getQueueItems(
        channelName,
        1,
      );

      if (remainingItems.length > 0) {
        this.logger.debug(
          `More messages remain in queue for channel ${channelName}, scheduling next batch`,
        );

        // Schedule the next batch to avoid blocking the event loop
        this.flushingChannels.delete(channelName);
        setImmediate(() => {
          this.logger.debug(
            `Scheduling flushChannelQueue for channel ${channelName}`,
          );
          this.flushChannelQueue(channelName);
        });
      } else {
        this.flushingChannels.delete(channelName);
        this.logger.debug(
          `Queue for channel ${channelName} completely flushed`,
        );
      }
    } catch (error: any) {
      this.logger.error(
        `Error flushing queue for channel ${channelName}: ${error.message}`,
      );
      this.flushingChannels.delete(channelName);
    }
  }

  private async saveToQueue(channelName: string, payload: any): Promise<void> {
    try {
      this.logger.debug(`Saving to queue ${channelName}}: ${payload.id}`);
      await this.queueService.insertToQueue(channelName, payload);
    } catch (error: any) {
      this.logger.error(
        `Failed to save to queue ${channelName}: ${error.message}`,
      );
    }
  }
  handleDisconnect(client: Socket) {
    this.logger.log(`Client disconnected: ${client.id}`);
    const userId: any = Array.from(this.authenticatedClients.entries()).find(
      ([, value]) => value.client_ids.includes(client.id),
    )?.[0];
    //delete the client.id from the client_ids array
    const clientData = this.authenticatedClients.get(userId);
    if (clientData) {
      clientData.client_ids = clientData.client_ids.filter(
        (id: string) => id !== client.id,
      );
    }

    const old_client_ids = this.authenticatedClients.get(userId)?.client_ids;
    if (old_client_ids?.length == 0) {
      this.disconnected_clients.set(userId, []);
    }

    if (!userId) return;

    const disconnectTimeout = setTimeout(async () => {
      // Check if the client has reconnected
      const clientData = this.authenticatedClients.get(userId);
      const new_client_ids = this.authenticatedClients.get(userId)?.client_ids;
      if (
        clientData &&
        !old_client_ids.includes(client.id) &&
        new_client_ids.length > old_client_ids.length
      ) {
        this.logger.log(`Client ${client.id} reconnected, skipping cleanup`);

        this.processingQueue.unshift(
          ...(this.disconnected_clients.get(userId) || []),
        );
        setImmediate(() => {
          this.processNext();
        });
        this.disconnected_clients.delete(userId);

        return;
      }

      // Perform cleanup
      if (clientData?.client_ids.length === 0) {
        try {
          for (const channelName of clientData.channels) {
            await this.queueService.deleteQueueByName(channelName);
            this.channelPipes.delete(channelName);
            this.backpressuredChannels.delete(channelName);
            this.flushingChannels.delete(channelName);
            this.logger.log(
              `Cleaned up channel ${channelName} for client ${client.id}`,
            );
          }
          this.authenticatedClients.delete(userId);
        } catch (error: any) {
          this.logger.error(
            `Failed to clean up channel for client ${client.id}: ${error.message}`,
          );
        }
      }
      this.logger.log(`Client ${client.id} removed from authenticated clients`);
    }, 5000);

    client.on('reconnect', () => {
      clearTimeout(disconnectTimeout);
      this.logger.log(`Client ${client.id} reconnected, cleanup cancelled`);
    });
  }

  async onModuleDestroy() {
    this.logger.log('Cleaning up resources on module destroy');

    // Disconnect all clients
    this.server.sockets.sockets.forEach((socket) => {
      socket.disconnect(true);
    });
    for (const [_, clientData] of this.authenticatedClients.entries()) {
      for (const channelName of clientData.channels) {
        await this.queueService.deleteQueueByName(channelName);
        this.logger.log(
          `Cleaned up channel ${channelName} for client ${[
            ...clientData.client_ids,
          ]}`,
        );
      }
    }

    this.authenticatedClients.clear();

    this.channelPipes.forEach((pipe, channelName) => {
      pipe.destroy();
      this.channelPipes.delete(channelName);
    });

    this.backpressuredChannels.clear();
    this.flushingChannels.clear();

    this.processingQueue = [];
    this.isProcessing = false;

    this.logger.log('Cleanup completed');
  }
}
