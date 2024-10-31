import { NestFactory } from '@nestjs/core';
import { MainModule } from './main.module';
import { LoggerService, HttpExceptionFilter } from '@dna-platform/common';

async function bootstrap() {
  const logger = new LoggerService(process.env.npm_package_name ?? 'unknown', {
    timestamp: process.env.DEBUG === 'true',
  });

  const app = await NestFactory.create(MainModule);
  app.useGlobalFilters(new HttpExceptionFilter());
  await app.listen(+(process.env.PORT || '3000')).then((server) => {
    logger.log(
      `Application is listening ${server._connectionKey} [${process.env.NODE_ENV}]`,
    );
  });
}

bootstrap();
