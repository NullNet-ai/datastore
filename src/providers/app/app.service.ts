import { Injectable } from '@nestjs/common';
import { HelperService } from '@dna-platform/common';
import * as crypto from 'crypto';

@Injectable()
export class AppService {
  constructor(private readonly helper: HelperService) {}
  generateSSOClientDetails(application_name: string) {
    const verifier = crypto.randomBytes(32).toString('base64url');
    // 2. Generate a code_challenge from the verifier
    const challenge = crypto
      .createHash('sha256')
      .update(verifier)
      .digest()
      .toString('base64url');
    const suffix = this.helper.snakeCase(
      `${application_name
        .split(/[aeiou]/i)
        .map((s) => s.slice(0, 3))
        .join('')}${new Date().getTime()}`
        .trim()
        .replace(/\s+/g, ''),
    );
    const client_id = `${verifier}${suffix}`;
    const client_secret = challenge;
    return { client_id, client_secret };
  }
}
