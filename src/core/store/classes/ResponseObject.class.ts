export class ResponseObject {
  data: null | any;
  status: 'ok' | 'error';
  message?: string;
  params?: any;
  query?: any;
}
