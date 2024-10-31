import {
  BadRequestException,
  Body,
  Controller,
  ForbiddenException,
  Post,
  Res,
} from '@nestjs/common';
import type { Response } from 'express';
import { OrganizationsService } from './organizations.service';
import { RegisterDto } from './dto/register.dto';
import { RegisterPipe } from './pipes/register.pipe';
import { AuthDto } from './dto/auth.dto';
import { AuthService } from './auth.service';

@Controller('organizations')
export class OrganizationsController {
  constructor(
    private readonly organizationsService: OrganizationsService,
    private readonly authService: AuthService,
  ) {}

  @Post('register')
  async register(@Body('data', RegisterPipe) data: RegisterDto) {
    if (!data) {
      throw new BadRequestException('Invalid Input');
    }
    return this.organizationsService.register(data);
  }

  @Post('auth')
  async auth(
    @Body('data') data: AuthDto,
    @Res({ passthrough: true }) response: Response,
  ) {
    const token = await this.authService.auth(data.email, data.password);

    if (!token) {
      throw new ForbiddenException('Invalid Credentials');
    }

    response.cookie('token', token);

    response.send({
      token,
    });
  }
}
