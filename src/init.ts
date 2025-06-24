import 'dotenv/config';
import * as fs from 'fs';
import {
  AccountType,
  OrganizationsService,
  // AccountType,
} from '@dna-platform/crdt-lww-postgres';
import { MinioService } from './providers/files/minio.service';
import { InitializerService } from './providers/store/store.service';
import { EInitializer } from './xstate/modules/schemas/create/create.schema';
import { LoggerService } from '@dna-platform/common';
const {
  DB_FILE_DIR = '',
  NODE_ENV = 'local',
  DEFAULT_ORGANIZATION_NAME = 'global-organization',
  DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
  DEFAULT_DEVICE_ID = 'system_device',
  DEFAULT_DEVICE_SECRET = 'ch@ng3m3Pl3@s3!!',
  INITIALIZE_DEVICE= false
} = process.env;
const logger = new LoggerService(process.env.npm_package_name ?? 'unknown');
fs.mkdirSync(DB_FILE_DIR, { recursive: true });
fs.mkdirSync('./tmp', { recursive: true });
fs.mkdirSync('./upload', { recursive: true });
export async function initializers(app) {
  const storage = app.get(MinioService);
  const organization = app.get(OrganizationsService);
  const initializer: InitializerService = app.get(InitializerService);
  initializer.createEncryption();

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
      default_code: 100000,
      prefix: 'CO',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'user_roles',
    system_code_config: {
      default_code: 100000,
      prefix: 'RO',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'organizations',
    system_code_config: {
      default_code: 100000,
      prefix: 'OR',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'notifications',
    system_code_config: {
      default_code: 100000,
      prefix: 'NO',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'communication_templates',
    system_code_config: {
      default_code: 100000,
      prefix: 'CT',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'account_organizations',
    system_code_config: {
      default_code: 100000,
      prefix: 'AC',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'devices',
    system_code_config: {
      default_code: 100000,
      prefix: 'DV',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'device_remote_access_sessions',
    system_code_config: {
      default_code: 100000,
      prefix: 'RA',
      counter: 0,
      digits_number: 6,
    },
  });
  await initializer.create(EInitializer.SYSTEM_CODE_CONFIG, {
    entity: 'device_credentials',
    system_code_config: {
      default_code: 100000,
      prefix: 'DC',
      counter: 0,
      digits_number: 6,
    },
  });

  // ! This is a sample for the root account configuration
  await initializer.create(EInitializer.ROOT_ACCOUNT_CONFIG, {
    entity: 'account_organizations',
  });

  // default for super admin
  await organization.initialize();
  // ! Device Account Initialization (uncomment if needed)

  if(INITIALIZE_DEVICE){
    await organization.initialize({
      account_type: AccountType.DEVICE,
      organization_id: DEFAULT_ORGANIZATION_ID,
      organization_name: DEFAULT_ORGANIZATION_NAME,
      account_id: DEFAULT_DEVICE_ID,
      account_secret: DEFAULT_DEVICE_SECRET,
      first_name: '',
      last_name: '',
      is_new_user: true,
      account_status: 'Active',
      role_id: 'super_admin',
      account_organization_status: 'Active',
      account_organization_categories: ['Device'],
      device_categories: ['Device'],
    });
  }

  await storage.makeBucket(DEFAULT_ORGANIZATION_NAME, DEFAULT_ORGANIZATION_ID);
  await initializer.generateSchema();
}

export async function cleanupTemporaryFiles() {
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
