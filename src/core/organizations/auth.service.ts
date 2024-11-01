import { BadRequestException, Injectable } from '@nestjs/common';
import { and, eq } from 'drizzle-orm/sql';
import * as jsonwebtoken from 'jsonwebtoken';
import { DrizzleService } from '../modules/drizzle/drizzle.service';
import {
  contacts,
  organization_contact_accounts,
  organization_contacts,
  organizations,
} from '../../xstate/modules/schemas/drizzle';
const { JWT_SECRET = 'Ch@ng3m3Pl3@s3!!', JWT_EXPIRES_IN = '2d' } = process.env;

@Injectable()
export class AuthService {
  constructor(private readonly drizzleService: DrizzleService) {}

  async sign(payload): Promise<string> {
    return jsonwebtoken.sign(payload, JWT_SECRET, {
      expiresIn: JWT_EXPIRES_IN,
    });
  }

  async verify(token: string): Promise<any> {
    return jsonwebtoken.verify(token, JWT_SECRET);
  }

  async auth(email: string, password: string) {
    const db = this.drizzleService.getClient();
    const [organization_contact_account = null] = await db
      .select()
      .from(organization_contact_accounts)
      .where(
        and(
          eq(organization_contact_accounts.tombstone, 0),
          eq(organization_contact_accounts.email, email),
          eq(organization_contact_accounts.status, 'Active'),
        ),
      )
      .catch(() => {
        return [];
      });

    if (!organization_contact_account) {
      return null;
    }

    const { password: saved_password } = organization_contact_account;
    if (!saved_password) throw new BadRequestException('Passwrod is required');

    const verified = await Bun.password.verify(password, saved_password);

    if (!verified) {
      return null;
    }

    const [account] = await db
      .select({
        contact: contacts,
        organization: organizations,
        organization_id: organization_contacts.organization_id,
      })
      .from(organization_contacts)

      .where(
        and(
          eq(organization_contacts.tombstone, 0),
          // @ts-ignore
          eq(
            organization_contacts.id,
            organization_contact_account.organization_contact_id,
          ),
        ),
      )
      .leftJoin(contacts, eq(contacts.id, organization_contacts.contact_id))
      .leftJoin(
        organizations,
        eq(organizations.id, organization_contacts.organization_id),
      );

    return this.sign({ account });
  }
}
