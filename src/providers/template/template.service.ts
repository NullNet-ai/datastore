import { Injectable } from '@nestjs/common';
import { Response, Request } from 'express';
import { Machine } from '@dna-platform/common';
@Injectable()
export class TemplateService {
  @Machine('template')
  getTemplate(_res: Response, _req: Request) {}

  // @Machine('helloWorld', { disableApiBehavior: true, overrideReturn: true })
  // async helloWorldMessageFn(_arg1: string) {
  //   return {
  //     message: 'Hello World!' + _arg1,
  //   };
  // }
}
