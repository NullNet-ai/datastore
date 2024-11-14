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
  // DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
} = process.env;
fs.mkdirSync(DB_FILE_DIR, { recursive: true });
const logger = new LoggerService(process.env.npm_package_name ?? 'unknown', {
  timestamp: DEBUG === 'true',
});

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

async function bootstrap() {
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

  // cleanup the temporary files every 1 minute in remote environment
  cleanupTemporaryFiles();
}

bootstrap();
