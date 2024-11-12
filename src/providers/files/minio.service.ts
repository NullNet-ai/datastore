import { BadRequestException, Injectable, Logger } from '@nestjs/common';
import * as https from 'https';
import * as fs from 'fs';
import * as Minio from 'minio';
// import storage_policy from './storage_policy';
const {
  STORAGE_ENDPOINT,
  STORAGE_ACCESS_KEY,
  STORAGE_SECRET_KEY,
  NODE_ENV = 'local',
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
export class MinioService {
  public client: Minio.Client;
  constructor(private readonly logger: Logger) {}

  async onModuleInit() {
    // TODO - create bucket during organization where parent is null
    if (
      !STORAGE_ENDPOINT ||
      !STORAGE_ACCESS_KEY ||
      !STORAGE_SECRET_KEY ||
      !STORAGE_REGION
    ) {
      throw new Error('Upload credentials not found');
    }
    if (['local'].includes(NODE_ENV)) return;
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
      });
    } catch (e) {
      this.logger.error(`[ERROR][MinioClient]:`, e);
    }
  }

  public async makeBucket(bucketName: string = STORAGE_BUCKET_NAME) {
    if (['local'].includes(NODE_ENV)) return;
    this.isValidBucketName(bucketName);

    // Check if the bucket exists
    // If it doesn't, create it
    const exists = await this.client.bucketExists(bucketName);
    if (exists) {
      this.logger.warn(
        `Bucket ${bucketName} already exists in ${STORAGE_REGION}.`,
      );
    } else {
      // TODO - move this to organization creation
      await this.client.makeBucket(bucketName, STORAGE_REGION, {
        // ObjectLocking: true,
      });

      // Set the bucket policy of `my-bucketname`
      // await this.client.setBucketPolicy(
      //   bucketName,
      //   JSON.stringify(storage_policy(bucketName)),
      // );

      this.logger.log(
        `Bucket ${bucketName} created successfully in ${STORAGE_REGION}.`,
      );
    }
  }

  private isString(arg) {
    return typeof arg === 'string';
  }

  private isValidBucketName(bucket) {
    if (!this.isString(bucket)) {
      throw new BadRequestException('Bucket name should be a string');
    }

    // bucket length should be less than and no more than 63
    // characters long.
    if (bucket.length < 3 || bucket.length > 63) {
      throw new BadRequestException(
        `Bucket name should be between 3 and 63 characters long`,
      );
    }
    // bucket with successive periods is invalid.
    if (bucket.indexOf('..') > -1) {
      throw new BadRequestException(
        `Bucket name cannot have successive periods`,
      );
    }
    // bucket cannot have ip address style.
    if (bucket.match(/[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+/)) {
      throw new BadRequestException(`Bucket name cannot be an IP address`);
    }
    // bucket should begin with alphabet/number and end with alphabet/number,
    // // with alphabet/number/.- in the middle.
    if (bucket.match(/^[a-z0-9][a-z0-9.-]+[a-z0-9]$/)) {
      this.logger.log(`Bucket name ${bucket} is valid`);
    } else
      throw new BadRequestException(
        `Bucket name ${bucket} is invalid. Please check the bucket name. Please try ${bucket
          .trim()
          .toLowerCase()
          .replace(' ', '-')}`,
      );
  }
}
