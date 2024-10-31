import { PipeTransform, Injectable } from '@nestjs/common';
import { RegisterDto } from '../dto/register.dto';
import { z } from 'zod';

const schema = z.object({
  name: z.string(),
  first_name: z.string(),
  last_name: z.string(),
  email: z.string(),
  password: z.string().min(8),
});

@Injectable()
export class RegisterPipe implements PipeTransform {
  async transform(value: RegisterDto) {
    return schema.parse(value);
  }
}
