import { Injectable } from '@nestjs/common';

const { JWT_SECRET = 'Ch@ng3m3Pl3@s3!!' } = process.env;
import * as jsonwebtoken from 'jsonwebtoken';

@Injectable()
export class AuthService {
  constructor() {}

  async verify(token: string): Promise<any> {
    return jsonwebtoken.verify(token, JWT_SECRET);
  }
}
