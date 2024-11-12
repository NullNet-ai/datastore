import 'dotenv/config';
import { NestFactory } from '@nestjs/core';
import { MainModule } from './main.module';
import { LoggerService, HttpExceptionFilter } from '@dna-platform/common';
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
  //   // @ts-ignore - set the company's organization parent
  //   parent_organization_id: DEFAULT_ORGANIZATION_ID,
  //   email: 'sample-company@sample.com',
  //   password: 'sample-passwd',
  //   first_name: 'Company',
  //   last_name: 'Orgs',
  // });
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

  await initialOrganization(organization, storage);
}

bootstrap();
