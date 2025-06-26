
import { BadRequestException, Injectable } from '@nestjs/common';
import { IResponse } from '@dna-platform/common';
import { fromPromise } from 'xstate';
import { IActors } from '../../schemas/register_device/register_device.schema';
import { DrizzleService, OrganizationsService, SyncService } from '@dna-platform/crdt-lww-postgres';
import { VerifyActorsImplementations } from '../verify';
import { and, eq } from 'drizzle-orm/sql';
import * as schema from '../../../../schema';
import { date_options, locale, timezone } from '@dna-platform/crdt-lww-postgres/build/modules/constants';
import { Utility } from '../../../../utils/utility.service';
import { ulid } from 'ulid';

@Injectable()
export class RegisterDeviceActorsImplementations {
  private db;
  constructor(
    private readonly syncService: SyncService,
    private readonly verifyActorImplementations: VerifyActorsImplementations,
    private readonly organizationService:OrganizationsService,
    private readonly drizzleService: DrizzleService,
  ) {
    this.db = this.drizzleService.getClient();
    this.actors.verify = this.verifyActorImplementations.actors.verify;
  }
  public readonly actors: IActors = {
    registerDevice: fromPromise(async ({ input }): Promise<IResponse> => {
      let errors: { message: string; stack: string; status_code: number }[] =
        [];
      let metadata: Record<string, any> = [];
      try {
      const { context } = input;
      if (!context?.controller_args)
        return Promise.reject({
          payload: {
            success: false,
            message: 'No controller args found',
            count: 0,
            data: [],
          },
        });
        const { controller_args, responsible_account } =
          context;

        const {
          organization_id = '',
          account_organization_id,
        } = responsible_account;
        const [_res, _req] = controller_args;
        const { body } = _req;
        let {
          organization_id: team_organization_id,
          account_id,
          account_secret,
          role_id,
          is_invited=false,
          is_new_user=false,
          device_id='',
          device_categories,
          account_organization_status,
          account_organization_categories,
        } = body.device;
        const device_organization_id=organization_id || team_organization_id;

        const date = new Date();
        const formatted_date = date.toLocaleDateString(locale, date_options);
        const formatted_time = Utility.convertTime12to24(
          date.toLocaleTimeString(locale, { timeZone: timezone }),
        );

      if (!device_organization_id)
          return Promise.reject({
            payload: {
              success: false,
              message: 'No organization id found, please pass team organization for assigning device to it',
              count: 0,
              data: [],
            },
          });

        const [existing_account] = await this.db
          .select()
          .from(schema.accounts)
          .where(
            and(
              eq(
                (schema.accounts as Record<string, any>).account_id,
                account_id,
              ),
              eq((schema.accounts as Record<string, any>).tombstone, 0),
            ),
          );

        if(existing_account?.organization_id === device_organization_id) {
          return Promise.reject({
            payload: {
              success: false,
              message: 'Device already registered in this organization',
              count: 0,
              data: [],
            },
          });
        }

        //create an account for this device and assign it the team organization
        let created_account_and_device_id=ulid()
       await this.organizationService.createAccount({
          id:created_account_and_device_id,
          account_id: account_id,
          account_secret: account_secret,
          personal_organization_id:device_organization_id,
         is_new_user,
         formatted_date,
         formatted_time,
         first_name:'',
         last_name:'',
         account_status: 'Inactive',
         status: 'Draft',
         responsible_account_organization_id: account_organization_id,
         create_profile:false
        })

        const [account_organizations_counter = null] = await this.db
          .select()
          .from(schema.counters)
          .where(eq(schema.counters.entity, 'account_organizations'));


        //create account organization


        if(device_id === '' || device_id === null) {

          const [devices_counter = null] = await this.db
            .select()
            .from(schema.counters)
            .where(eq(schema.counters.entity, 'devices'));

          const device_data = {
            id: created_account_and_device_id,
            organization_id: team_organization_id,
            categories: device_categories,
            ...(devices_counter && {
              code: await Utility.generateCode(this.db, 'devices'),
            }),
            tombstone: 0,
            status: 'Draft',
            created_date: formatted_date,
            created_time: formatted_time,
            updated_date: formatted_date,
            updated_time: formatted_time,
            created_by: account_organization_id,
          };
          let device = await this.syncService.insert('devices', device_data);
          device_id = device.id;
        }

        const account_organization = {
          id: ulid(),
          email: account_id,
          account_id: created_account_and_device_id,
          organization_id: team_organization_id,
          contact_id: "",
          account_organization_status: account_organization_status,
          role_id,
          is_invited,
          ...(account_organizations_counter && {
            code: await Utility.generateCode(this.db,'account_organizations'),
          }),
          tombstone: 0,
          status: 'Active',
          device_id,
          categories: account_organization_categories,
          created_date: formatted_date,
          created_time: formatted_time,
          updated_date: formatted_date,
          updated_time: formatted_time,
          created_by: account_organization_id,
        };

        await this.syncService.insert("account_organizations",account_organization );

        let response={
          device_id,
          account_id: created_account_and_device_id,
          organization_id: team_organization_id,
          account_organization_id: account_organization.id,
        }

      return Promise.resolve({
        payload: {
          success: true,
          message: `Device with id ${device_id} registered successfully in organization ${team_organization_id}`,
          count: 0,
          data: [response],
        },
      });

    }catch (error: any) {
      errors.push({
        message: error?.message,
        stack: error.stack,
        status_code: error.status_code,
      });
      if (error.status !== 400 && error.status < 500) throw error;
      throw new BadRequestException({
        success: false,
        message: `There was an error while creating the new record in registering of device. Please verify the entered information for completeness and accuracy. If the issue continues, contact your database administrator for further assistance.`,
        count: 0,
        data: [],
        metadata,
        errors,
      });
    }
    }),
  };
}
