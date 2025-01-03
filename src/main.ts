import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { MainModule } from './main.module';
import { LoggerService } from '@dna-platform/common';
import * as fs from 'fs';
import * as cookieParser from 'cookie-parser';
import { OrganizationsService } from '@dna-platform/crdt-lww-postgres';
import { MinioService } from './providers/files/minio.service';
import { Transport } from '@nestjs/microservices';
import { join } from 'path';

const {
  PORT = '3060',
  DB_FILE_DIR = '',
  DEBUG = 'false',
  NODE_ENV = 'local',
  GRPC_PORT = '6000',
  DEFAULT_ORGANIZATION_NAME = 'global-organization',
  // DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
} = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });
fs.mkdirSync('./tmp', { recursive: true });
fs.mkdirSync('./upload', { recursive: true });
const logger = new LoggerService(process.env.npm_package_name ?? 'unknown');

async function initialOrganization(
  organization: OrganizationsService,
  storage: MinioService,
) {
  // default for super admin
  await organization.initialize();
  await storage.makeBucket(DEFAULT_ORGANIZATION_NAME);

  // create own default organization here
  // await organization.initialize({
  //   id: 'company-id',
  //   name: 'company-name',
  //   // 01JBHKXHYSKPP247HZZWHA3JCT = super-organization's ID
  //   parent_organization_id: DEFAULT_ORGANIZATION_ID,
  //   email: 'sample-company@sample.com',
  //   password: 'sample-passwd',
  //   first_name: 'Company',
  //   last_name: 'Orgs',
  // });
}

async function cleanupTemporaryFiles() {
  if (['local'].includes(NODE_ENV)) return;
  if (['local'].includes(NODE_ENV)) return;
  let file_cleanup_interval: any = null;
  const time_in_ms = 60000;
  if (process.env.STORAGE_UPLOAD_PATH) {
    clearInterval(file_cleanup_interval);
    logger.warn('cleanupTemporaryFiles started running every 1 minute');
    file_cleanup_interval = setInterval(() => {
      try {
        logger.log('deleting files in upload and tmp path');
        fs.rmdirSync(process.env.STORAGE_UPLOAD_PATH || '', {
          recursive: true,
        });
        fs.rmdirSync('./tmp', { recursive: true });
        logger.log('recreating upload and tmp path');
        fs.mkdirSync(process.env.STORAGE_UPLOAD_PATH || '', {
          recursive: true,
        });
        fs.mkdirSync('./tmp', { recursive: true });
      } catch (error) {
        logger.error(error);
      }
    }, time_in_ms);
  }
}

// @ts-ignore
async function bootstrap() {
  const app = await NestFactory.create(MainModule, {
    logger,
  });
  app.use(cookieParser());
  app.useLogger(logger);

  await app.listen(+(PORT || '5001')).then(() => {
    logger.log(
      `Application is listening at ${PORT} [${NODE_ENV}]${
        DEBUG === 'true' ? ' with debugger ON' : ''
      }`,
    );
  });

  const storage = app.get(MinioService);
  // initialize the organization
  const organization = app.get(OrganizationsService);

  await initialOrganization(organization, storage);

  // cleanup the temporary files every 1 minute in remote environment
  cleanupTemporaryFiles();
}

async function bootstrapGrpc() {
  const app = await NestFactory.createMicroservice(MainModule, {
    transport: Transport.GRPC,
    options: {
      url: `localhost:${GRPC_PORT}`, // Expose gRPC on this port
      package: 'example', // Proto package name
      protoPath: [
        join(__dirname, './proto/example.proto'),
        join(__dirname, './proto/getById.proto'),
      ], // Path to proto file
    },
  });
  app.useLogger(logger);

  app.listen().then(() => {
    logger.log(`gRPC microservice is running on port ${GRPC_PORT}`);
  });

  // initialize the organization

  // cleanup the temporary files every 1 minute in remote environment
}
async function bootstrapAll() {
  // Start HTTP app
  // await bootstrap();

  // Start gRPC app
  await bootstrapGrpc();
}
bootstrapAll();
