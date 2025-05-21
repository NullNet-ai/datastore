import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { HttpModule } from './http.module';
import { LoggerService } from '@dna-platform/common';
import * as fs from 'fs';
import cookieParser from 'cookie-parser';
import { MicroserviceOptions, Transport } from '@nestjs/microservices';
import { join } from 'path';
import { BatchSyncModule } from './batch_sync/batch_sync.module';
import { GrpcModule } from './grpc.module';
import { cleanupTemporaryFiles, initializers } from './init';
const {
  PORT = '3060',
  DB_FILE_DIR = '',
  DEBUG = 'false',
  NODE_ENV = 'local',
  GRPC_PORT = '6000',
} = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });
fs.mkdirSync('./tmp', { recursive: true });
fs.mkdirSync('./upload', { recursive: true });
const logger = new LoggerService(process.env.npm_package_name ?? 'unknown');

// @ts-ignore
async function bootstrap() {
  const app = await NestFactory.create(HttpModule, {
    // !TODO: causes an issue with winston transport for lowdb reading a file from debug.json
    logger: process.env.NODE_ENV === 'production' ? false : logger,
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
  if (process.env.SCRIPT_INIT === 'true') process.exit(0);

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
      logger: process.env.NODE_ENV === 'production' ? false : logger,
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
      logger: process.env.NODE_ENV === 'production' ? false : logger,
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
