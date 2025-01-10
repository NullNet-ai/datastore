import { Logger } from '@nestjs/common';
import { z } from 'zod';
import { Utility } from '../utils/utility.service';
import { Request, Response } from 'express';
const { DEBUG = 'false' } = process.env;
const debug = DEBUG === 'true';
export function ValidateZod(zodObject: any) {
  return function (_target: any, _propertyKey: string, descriptor: any) {
    const originalMethod = descriptor.value;
    descriptor.value = async function (...args: any[]) {
      if (debug)
        Logger.debug(
          `[ValidateZod][${_propertyKey}]: ${JSON.stringify(
            _propertyKey,
            args,
          )}`,
        );
      const request: Request = args[1];
      const response: Response = args[0];
      function transformer(acc: any, arg: any, _index: number) {
        return {
          ...acc,
          [arg]: request[arg],
        };
      }
      try {
        Utility.validateZodSchema([
          {
            zod: z.object(zodObject),
            params: Object.keys(zodObject).reduce(transformer, {}),
          },
        ]);
      } catch (error: any) {
        return response.status(400).json({
          status: 400,
          message: JSON.parse(error.message),
        });
      }

      return originalMethod.apply(this, args);
    };
    return descriptor;
  };
}
