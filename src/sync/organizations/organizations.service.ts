import { Injectable, Logger } from '@nestjs/common';
import { StoreService } from '../store/store.service';
import { RegisterDto } from './dto/register.dto';
import { ulid } from 'ulid';
import { DrizzleService } from '../modules/drizzle/drizzle.service';
import * as schema from '../schema';
import { and, eq } from 'drizzle-orm';

const {
  DEFAULT_ORGANIZATION_ID = '01JBHKXHYSKPP247HZZWHA3JCT',
  DEFAULT_ORGANIZATION_NAME = 'Super Organization',
  DEFAULT_ORGANIZATION_ADMIN_EMAIL = 'admin@dnamicro.com',
  DEFAULT_ORGANIZATION_ADMIN_PASSWORD = 'ch@ng3m3Pl3@s3!!',
} = process.env;

@Injectable()
export class OrganizationsService {
  private readonly logger = new Logger(this.constructor.name);

  constructor(
    private readonly storeService: StoreService,
    private readonly drizzleService: DrizzleService,
  ) {}

  async createNewOrganization({ name, id }: RegisterDto) {
    const organization_id = id ?? ulid();
    const date = new Date();
    const organization = {
      id: organization_id,
      parent_organization_id:
        DEFAULT_ORGANIZATION_ID === organization_id
          ? undefined
          : DEFAULT_ORGANIZATION_ID,
      tombstone: 0,
      status: 'Active',
      created_date: date.toLocaleDateString(),
      created_time: date.toLocaleTimeString(),
      updated_date: date.toLocaleDateString(),
      updated_time: date.toLocaleTimeString(),
    };
    this.logger.log(
      `Creating Organization: ${name} with ID: ${organization_id}`,
    );
    await this.storeService.insert('organizations', organization);
    this.logger.log(
      `Created Organization: ${name} with ID: ${organization_id}`,
    );
    return organization_id;
  }

  async createOrganizationAccount(dto: RegisterDto, organization_id: string) {
    const { email, first_name, last_name = '', password } = dto;
    const date = new Date();

    const contact = {
      id: ulid(),
      categories: ['Contact'],
      email,
      first_name,
      last_name,
      created_date: date.toLocaleDateString(),
      created_time: date.toLocaleTimeString(),
      updated_date: date.toLocaleDateString(),
      updated_time: date.toLocaleTimeString(),
    };

    const organization_contact = {
      id: ulid(),
      organization_id,
      contact_id: contact.id,
      tombstone: 0,
      status: 'Active',
      created_date: date.toLocaleDateString(),
      created_time: date.toLocaleTimeString(),
      updated_date: date.toLocaleDateString(),
      updated_time: date.toLocaleTimeString(),
    };

    const organization_contact_accounts = {
      id: ulid(),
      organization_id,
      organization_contact_id: organization_contact.id,
      email,
      first_name,
      last_name,
      password: await Bun.password.hash(password),
      tombstone: 0,
      status: 'Active',
      created_date: date.toLocaleDateString(),
      created_time: date.toLocaleTimeString(),
      updated_date: date.toLocaleDateString(),
      updated_time: date.toLocaleTimeString(),
    };

    this.logger.log(`Creating Account: ${email}`);
    await this.storeService.insert('contacts', contact);
    this.logger.log(`Created Account: ${email}`);

    await this.storeService.insert(
      'organization_contacts',
      organization_contact,
    );
    await this.storeService.insert(
      'organization_contact_accounts',
      organization_contact_accounts,
    );
  }

  async register(dto: RegisterDto) {
    const db = this.drizzleService.getClient();
    const { name } = dto;
    const [existing_orgs = null] = await db
      .select()
      .from(schema.organizations)
      .where(
        and(
          eq(schema.organizations.tombstone, 0),
          eq(schema.organizations.name, name),
        ),
      );

    let organization_id = existing_orgs?.id;
    if (!existing_orgs) {
      organization_id = await this.createNewOrganization(dto);
    }

    const [existing_account = null] = await db
      .select()
      .from(schema.organization_contact_accounts)
      .where(
        and(
          eq(schema.organization_contact_accounts.tombstone, 0),
          eq(schema.organization_contact_accounts.email, dto.email),
          eq(
            schema.organization_contact_accounts.organization_id,
            organization_id,
          ),
        ),
      );

    if (!existing_account) {
      await this.createOrganizationAccount(dto, organization_id);
    }

    return {
      organization_id,
    };
  }

  async onModuleInit() {
    if (!DEFAULT_ORGANIZATION_ID) {
      return;
    }

    const db = this.drizzleService.getClient();
    const [organization = null] = await db
      .select()
      .from(schema.organizations)
      .where(eq(schema.organizations.id, DEFAULT_ORGANIZATION_ID));

    // console.log(organization);
    if (organization) {
      this.logger.log(`Organization ${organization.name} already exists`);
      return;
    }

    this.logger.log(`Creating Organization: ${DEFAULT_ORGANIZATION_NAME}`);

    await this.register({
      id: DEFAULT_ORGANIZATION_ID,
      name: DEFAULT_ORGANIZATION_NAME,
      email: DEFAULT_ORGANIZATION_ADMIN_EMAIL,
      password: DEFAULT_ORGANIZATION_ADMIN_PASSWORD,
      first_name: 'Super',
      last_name: 'Admin',
    });
  }
}
