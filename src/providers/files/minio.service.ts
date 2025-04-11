import { LoggerService } from '@dna-platform/common';
import { BadRequestException, Injectable } from '@nestjs/common';
// import * as https from 'https';
// import * as fs from 'fs';
import * as Minio from 'minio';
// import storage_policy from './storage_policy';
const {
  STORAGE_ENDPOINT,
  STORAGE_ACCESS_KEY,
  STORAGE_SECRET_KEY,
  NODE_ENV = 'local',
  // STORAGE_BUCKET_NAME = 'test',
  STORAGE_REGION = 'us-east-1',
  STORAGE_PORT = '9000',
  DEFAULT_ORGANIZATION_NAME = 'global-organization',
  DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
  // STORAGE_TIMEOUT = '10000',
  // STORAGE_TRANSPORT_KEEPALIVE,
  // SSL_CA = '',
  // SSL_CERT = '',
  // SSL_SECRET_KEY = '',
} = process.env;
@Injectable()
export class MinioService {
  public client: Minio.Client | null = null;
  constructor(private readonly logger: LoggerService) {}

  async onModuleInit() {
    if (['local'].includes(NODE_ENV)) return;
    // TODO - create bucket during organization where parent is null
    if (
      !STORAGE_ENDPOINT ||
      !STORAGE_ACCESS_KEY ||
      !STORAGE_SECRET_KEY ||
      !STORAGE_REGION
    ) {
      throw new Error('Upload credentials not found');
    }
    try {
      this.client = new Minio.Client({
        endPoint: STORAGE_ENDPOINT,
        port: +STORAGE_PORT,
        useSSL: +STORAGE_PORT === 443,
        accessKey: STORAGE_ACCESS_KEY,
        secretKey: STORAGE_SECRET_KEY,
        // ...(NODE_ENV === 'production' && {
        //   transportAgent: new https.Agent({
        //     timeout: +STORAGE_TIMEOUT,
        //     ca: fs.readFileSync(SSL_CA),
        //     cert: fs.readFileSync(SSL_CERT),
        //     key: fs.readFileSync(SSL_SECRET_KEY),
        //     keepAlive: STORAGE_TRANSPORT_KEEPALIVE === 'true',
        //   }),
        // }),
      });
    } catch (e) {
      this.logger.error(`[ERROR][MinioClient]:`, e);
    }
  }

  public getValidBucketName(bucketName: string, org_id?: string) {
    const org_pattern = org_id
      ? (
          org_id.substring(0, 2) +
          org_id.substring(
            Math.floor(org_id.length / 2) - 1,
            Math.floor(org_id.length / 2) + 1,
          ) +
          org_id.substring(org_id.length - 2)
        )
          .replace(/[^a-zA-Z]/g, '')
          .toLowerCase()
      : '';

    const _bname = bucketName
      .trim()
      .split(/\s+/)
      .map((word) => word.charAt(0))
      .join('')
      .toLowerCase()
      .replace(/[^a-z-]/g, '')
      .substring(0, 20);

    return _bname + org_pattern;
  }

  public async makeBucket(
    bucketName: string = DEFAULT_ORGANIZATION_NAME,
    org_id: string = DEFAULT_ORGANIZATION_ID,
  ) {
    if (['local'].includes(NODE_ENV)) return;
    const bucket_org_name = this.getValidBucketName(bucketName, org_id);
    this.isValidBucketName(bucket_org_name);

    // Check if the bucket exists
    // If it doesn't, create it
    const exists = await this.client?.bucketExists(bucket_org_name);
    if (exists) {
      this.logger.warn(
        `Bucket ${bucket_org_name} already exists in ${STORAGE_REGION}.`,
      );
    } else {
      // TODO - move this to organization creation
      await this.client?.makeBucket(bucket_org_name, STORAGE_REGION, {
        // ObjectLocking: true,
      });

      // Set the bucket policy of `my-_bname`
      // await this.client.setBucketPolicy(
      //   _bname,
      //   JSON.stringify(storage_policy(_bname)),
      // );

      this.logger.log(
        `Bucket ${bucket_org_name} created successfully in ${STORAGE_REGION}.`,
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
    if (bucket.length < 1 || bucket.length > 63) {
      throw new BadRequestException(
        `Bucket name [${bucket}] should be between 1 and 63 characters long`,
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
  }
}
