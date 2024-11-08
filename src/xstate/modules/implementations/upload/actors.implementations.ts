import { Injectable, Logger } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/upload/upload.schema';
import { CreateActorsImplementations } from '../create/actors.implementations';
import { VerifyActorsImplementations } from '../verify';
import * as Minio from 'minio';
import * as path from 'path';
import * as https from 'https';
import * as fs from 'fs';
const {
  STORAGE_ENDPOINT,
  STORAGE_ACCESS_KEY,
  STORAGE_SECRET_KEY,
  NODE_ENV,
  STORAGE_BUCKET_NAME = 'test',
  STORAGE_REGION = 'us-east-1',
  STORAGE_PORT = '9000',
  STORAGE_TIMEOUT = '10000',
  STORAGE_TRANSPORT_KEEPALIVE,
  SSL_CA = '',
  SSL_CERT = '',
  SSL_SECRET_KEY = '',
} = process.env;
@Injectable()
export class UploadActorsImplementations {
  public client: Minio.Client;
  constructor(
    private readonly createActorsImplementations: CreateActorsImplementations,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly logger: Logger,
  ) {}

  async onModuleInit() {
    if (
      !STORAGE_ENDPOINT ||
      !STORAGE_ACCESS_KEY ||
      !STORAGE_SECRET_KEY ||
      !STORAGE_BUCKET_NAME ||
      !STORAGE_REGION
    ) {
      throw new Error('Upload credentials not found');
    }

    try {
      this.client = new Minio.Client({
        endPoint: STORAGE_ENDPOINT,
        port: +STORAGE_PORT,
        useSSL: NODE_ENV === 'production',
        accessKey: STORAGE_ACCESS_KEY,
        secretKey: STORAGE_SECRET_KEY,
        ...(NODE_ENV === 'production' && {
          transportAgent: new https.Agent({
            timeout: +STORAGE_TIMEOUT,
            ca: fs.readFileSync(SSL_CA),
            cert: fs.readFileSync(SSL_CERT),
            key: fs.readFileSync(SSL_SECRET_KEY),
            keepAlive: STORAGE_TRANSPORT_KEEPALIVE === 'true',
          }),
        }),
        // pathStyle: true,
        // region: STORAGE_REGION,
      });

      // Check if the bucket exists
      // If it doesn't, create it
      const exists = await this.client.bucketExists(STORAGE_BUCKET_NAME);
      if (exists) {
        this.logger.warn(
          `Bucket ${STORAGE_BUCKET_NAME} already exists in ${STORAGE_REGION}.`,
        );
      } else {
        await this.client.makeBucket(STORAGE_BUCKET_NAME, STORAGE_REGION, {
          // ObjectLocking: true,
        });
        this.logger.log(
          `Bucket ${STORAGE_BUCKET_NAME} created successfully in ${STORAGE_REGION}.`,
        );
      }
    } catch (e) {
      this.logger.error(`[ERROR][MinioClient]:`, e);
    }
  }
  /**
   * Implementation of actors for the upload machine.
   */
  public readonly actors: IActors = {
    /**
     * Sample step actor implementation.
     * @param input - The input object containing the context.
     * @returns A promise that resolves to an IResponse object.
     */
    verify: this.verifyActorImplementations.actors.verify,
    create: this.createActorsImplementations.actors.create,
    upload: fromPromise(async ({ input }): Promise<IResponse> => {
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

      const { controller_args, responsible_account } = context;
      const [_res, _req, _file] = controller_args;
      // Set the object metadata
      const metadata = {
        'Content-Type': 'text/plain',
        'X-Amz-Meta-Testing': 1234,
        example: 5678,
      };

      const filepath = path.join(process.cwd(), _file.path);
      const uploaded = await this.client
        .fPutObject(STORAGE_BUCKET_NAME, _file.originalname, filepath, metadata)
        .catch((err) => {
          this.logger.error(`[ERROR][fPutObject]: ${err.message}`);
          return null;
        });

      this.logger.debug(`[UPLOADED]: ${JSON.stringify(uploaded)}`);
      // TODO: create a file copy of the record in the database that has uploaded_by key
      return Promise.resolve({
        payload: {
          success: true,
          message: `File uploaded successfully to ${_req.url}.`,
          count: 1,
          data: [
            {
              ..._file,
              uploaded_by: responsible_account.contact.id,
            },
          ],
        },
      });
    }),
  };
}
