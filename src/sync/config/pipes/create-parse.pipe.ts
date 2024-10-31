import type { Request } from 'express';
import { REQUEST } from '@nestjs/core';
import { Injectable, Inject, PipeTransform } from '@nestjs/common';
import { ulid } from 'ulid';
@Injectable()
export class CreateParsePipe implements PipeTransform {
  constructor(@Inject(REQUEST) protected readonly request: Request) {}

  convertTime12to24(time12h: string) {
    const [time = '', modifier] = time12h.split(' ');

    let [hours = '', minutes] = time.split(':');

    if (hours === '12') {
      hours = '00';
    }

    if (modifier === 'PM') {
      hours = `${parseInt(hours, 10) + 12}`;
    }

    if (parseInt(hours) < 10) {
      hours = `0${hours}`;
    }

    return `${hours}:${minutes}`;
  }
  format(data: any) {
    const date = new Date();
    const _data = {
      id: data.id ? data.id : ulid(),
      tombstone: 0,
      status: 'Active',
      updated_date: date.toLocaleDateString(),
      updated_time: this.convertTime12to24(date.toLocaleTimeString()),
      ...data,
    };
    return _data;
  }

  transform({
    schema,
    data,
  }: {
    schema: { parse: any };
    data: any;
    meta?: any;
  }) {
    const _data = this.format(data);
    return schema.parse(_data);
  }
}
