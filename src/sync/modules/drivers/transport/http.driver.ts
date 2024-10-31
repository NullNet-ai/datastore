import axios from 'axios';
import { BadRequestException, Injectable, Logger } from '@nestjs/common';
import { PostDto } from '../../drivers/dto/post.dto';
import { PostOpts, TransportDriver } from './transport.driver';

/*
  @class HttpTransportDriver
  @implements TransportDriver
  @description
    This class is an implementation of the TransportDriver abstract class.
    It is responsible for sending messages to the sync service over HTTP.
*/
@Injectable()
export class HttpTransportDriver implements TransportDriver {
  private logger = new Logger(this.constructor.name);

  async onModuleInit() {}

  private async *getChunks(client_id, opts: PostOpts) {
    let start = 0;
    let items = 0;

    const { username, password, url } = opts;
    if (!username || !password)
      throw new BadRequestException('Missing username or password');

    const SYNC_ENDPOINT = url;
    const client = axios.create({
      baseURL: SYNC_ENDPOINT,
      auth: {
        username,
        password,
      },
    });
    while (true) {
      const response = await client.get(`/app/sync/chunk`, {
        params: {
          client_id,
          start,
        },
      });
      const { messages = [], size = 0 } = response.data.data;
      items = items + messages.length;
      this.logger.debug(
        `Got Chunk of client_id${client_id} size:${items}/${size}`,
      );

      if (messages.length === 0) {
        break;
      }
      yield messages;
      start = start + messages.length;
    }
    this.logger.debug(`Got all chunks of client_id${client_id} - deleting`);

    await client.delete(`/app/sync/chunk`, {
      params: {
        client_id,
      },
    });
    this.logger.debug(`Got all chunks of client_id${client_id} - deleted`);
  }
  async post(data: PostDto, opts: PostOpts) {
    const SYNC_ENDPOINT = opts.url;
    const { username, password } = opts;
    if (!username || !password)
      throw new BadRequestException('Missing username or password');

    const client = axios.create({
      baseURL: SYNC_ENDPOINT,
      auth: {
        username,
        password,
      },
    });

    const res = await client
      .post('/app/sync', data)
      .then((res) => {
        return {
          data: res.data.data,
          status: 'ok',
          reason: 'ok',
        };
      })
      .catch((error) => {
        return {
          status: 'error',
          reason: error.message,
          data: {
            messages: [],
          },
        };
      });

    if (res.status !== 'ok') {
      throw new Error('API error: ' + res.reason);
    }

    if (res.data.incomplete) {
      this.logger.warn(`Chunk transfer requested`);
      for await (const chunk of this.getChunks(data.client_id, opts)) {
        res.data.messages = res.data.messages.concat(chunk);
      }
      this.logger.warn(`Chunk transfer done`);
    }

    return res.data;
  }
}
