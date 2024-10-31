import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/controllers/app/app.module';

describe('AppController (e2e)', () => {
  let app: INestApplication;
  beforeEach(async () => {
    const moduleRef: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    }).compile();

    app = moduleRef.createNestApplication();
    await app.init();
  });

  it('/ (GET) template - success flow with standard response', () => {
    const response = {
      success: true,
      message: 'sampleStep Message',
      count: 0,
      data: [],
    };
    return request(app.getHttpServer())
      .get('/template')
      .expect(200)
      .expect(response);
  });
});
