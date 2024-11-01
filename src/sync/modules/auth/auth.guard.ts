import { Injectable, CanActivate, ExecutionContext } from '@nestjs/common';
import { AuthService } from './auth.service';

@Injectable()
export class AuthGuard implements CanActivate {
  constructor(private readonly authService: AuthService) {}

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const request = context.switchToHttp().getRequest();

    const authorization = request.headers.authorization;
    const cookie_token = request?.cookies?.token;

    if (!authorization && !cookie_token) {
      return false;
    }

    const token = cookie_token ? cookie_token : authorization.split(' ')[1];

    if (!token) {
      return false;
    }

    const session = await this.authService.verify(token);

    if (!session) {
      return false;
    }

    request.account_session = session;

    return true;
  }
}
