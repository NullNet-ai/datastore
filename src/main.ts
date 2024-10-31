import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { MainModule } from './main.module';
import { LoggerService, HttpExceptionFilter } from '@dna-platform/common';
import {
  ArgumentsHost,
  BadRequestException,
  Catch,
  ExceptionFilter,
} from '@nestjs/common';
import { ZodError } from 'zod';
import * as fs from 'fs';
// const cookieParser = require('cookie-parser');

const { PORT = '3060', DB_FILE_DIR = '', DEBUG = 'false' } = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });

@Catch(BadRequestException)
export class BadExceptionFilter implements ExceptionFilter {
  catch(exception: BadRequestException, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const request = ctx.getRequest();
    const table = request.params.table;
    const response = ctx.getResponse();
    const status = 400;
    response.status(status).json({
      status,
      table,
      message: exception.message,
    });
  }
}

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

  const app = await NestFactory.create(MainModule);
  app.useGlobalFilters(new HttpExceptionFilter());
  await app.listen(+(PORT || '3000')).then((server) => {
    logger.log(
      `Application is listening ${server._connectionKey} [${process.env.NODE_ENV}]`,
    );
  });
}

bootstrap();
