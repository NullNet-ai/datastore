import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { HttpModule } from './http.module';
import { LoggerService } from '@dna-platform/common';
import * as fs from 'fs';
import cookieParser from 'cookie-parser';
import { OrganizationsService } from '@dna-platform/crdt-lww-postgres';
import { MinioService } from './providers/files/minio.service';
import { MicroserviceOptions, Transport } from '@nestjs/microservices';
import { join } from 'path';
import { BatchSyncModule } from './batch_sync/batch_sync.module';
import { GrpcModule } from './grpc.module';
import { InitializerService } from './providers/store/store.service';
import { EInitializer } from './xstate/modules/schemas/create/create.schema';
const {
  PORT = '3060',
  DB_FILE_DIR = '',
  DEBUG = 'false',
  NODE_ENV = 'local',
  GRPC_PORT = '6000',
  DEFAULT_ORGANIZATION_NAME = 'global-organization',
  DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
} = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });
fs.mkdirSync('./tmp', { recursive: true });
fs.mkdirSync('./upload', { recursive: true });
const logger = new LoggerService(process.env.npm_package_name ?? 'unknown');

async function initializers(app) {
  const storage = app.get(MinioService);
  const organization = app.get(OrganizationsService);
  const initializer: InitializerService = app.get(InitializerService);
  initializer.createEncryption();
  // default for super admin
  await organization.initialize();
  await organization.initializeDevice();
  await storage.makeBucket(DEFAULT_ORGANIZATION_NAME, DEFAULT_ORGANIZATION_ID);
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

  // TODO: Define Auto generated code Prefixes
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'contacts',
    system_code_config: {
      default_code: 10,
      prefix: 'CO',
      counter: 1,
      digits_number: 1,
    },
  });

  // ! This is a sample for the root account configuration
  await initializer.create(EInitializer.ROOT_ACCOUNT_CONFIG, {
    entity: 'organization_accounts',
  });
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
        fs.rmSync(process.env.STORAGE_UPLOAD_PATH || '', {
          recursive: true,
        });
        fs.rmSync('./tmp', { recursive: true });
        logger.log('recreating upload and tmp path');
        fs.mkdirSync(process.env.STORAGE_UPLOAD_PATH || '', {
          recursive: true,
        });
        fs.mkdirSync('./tmp', { recursive: true });
      } catch (error: any) {
        logger.error(error);
      }
    }, time_in_ms);
  }
}

// @ts-ignore
async function bootstrap() {
  const app = await NestFactory.create(HttpModule, {
    // !TODO: causes an issue with winston transport for lowdb reading a file from debug.json
    // logger,
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

  await initializers(app).catch(console.error);

  // cleanup the temporary files every 1 minute in remote environment
  cleanupTemporaryFiles();
}
// @ts-ignore
async function bootstrapBatchSyncService() {
  const app = await NestFactory.createMicroservice<MicroserviceOptions>(
    BatchSyncModule,
    {
      transport: Transport.TCP, // Use TCP as the transport layer
      options: {
        host: '0.0.0.0', // Localhost (can be omitted for defaults)
      },
    },
  );

  await app.listen(); // Start the microservice
}
async function bootstrapGrpc() {
  const app = await NestFactory.createMicroservice(GrpcModule, {
    transport: Transport.GRPC,
    options: {
      url: `0.0.0.0:${GRPC_PORT}`, // Expose gRPC on this port
      maxReceiveMessageLength: 1024 * 1024 * 50,
      maxSendMessageLength: 1024 * 1024 * 50,
      package: 'datastore', // Proto package name
      protoPath: [join(__dirname, './proto/store.proto')], // Path to proto file
      loader: {
        keepCase: true, // Prevents snake_case to camelCase conversion
      },
    },
  });
  app.useLogger(logger);

  app.listen().then(() => {
    logger.log(`gRPC microservice is running on port ${GRPC_PORT}`);
  });
}

async function bootstrapAll() {
  // start HTTP app
  await bootstrap();

  // start batch sync microservice
  // await bootstrapBatchSyncService();

  // start gRPC app
  await bootstrapGrpc();
}
bootstrapAll();
