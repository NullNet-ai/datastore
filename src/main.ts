import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { MainModule } from './main.module';
import { LoggerService, HttpExceptionFilter } from '@dna-platform/common';
import { ArgumentsHost, Catch, ExceptionFilter } from '@nestjs/common';
import { ZodError } from 'zod';
import * as fs from 'fs';
import * as cookieParser from 'cookie-parser';
import { OrganizationsService } from '@dna-platform/crdt-lww';
import { MinioService } from './providers/files/minio.service';

const {
  PORT = '3060',
  DB_FILE_DIR = '',
  DEBUG = 'false',
  NODE_ENV = 'local',
  DEFAULT_ORGANIZATION_NAME = 'super-organization',
} = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });

@Catch(ZodError)
export class ZodFilter<T extends ZodError> implements ExceptionFilter {
  catch(exception: T, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const request = ctx.getRequest();
    const table = request.params.table;
    const response = ctx.getResponse();
    const status = 400;
    response.status(status).json({
      status,
      message: 'Invalid Schema',
      table,
      errors: exception.errors,
    });
  }
}

async function bootstrap() {
  const logger = new LoggerService(process.env.npm_package_name ?? 'unknown', {
    timestamp: DEBUG === 'true',
  });

  const app = await NestFactory.create(MainModule, {
    logger: ['local', 'development', 'dev'].includes(NODE_ENV) ? logger : false,
  });
  app.use(cookieParser());
  app.useLogger(logger);
  app.useGlobalFilters(new HttpExceptionFilter());

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
  // default for super admin
  await organization.initialize();
  await storage.makeBucket(DEFAULT_ORGANIZATION_NAME);
}

bootstrap();
