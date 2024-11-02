import { Module } from '@nestjs/common';
import { AppModule } from './controllers/app/app.module';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww';
import { QueryDriverInterface } from '@dna-platform/crdt-lww/build/modules/drivers/query/enums';
import { DrizzleQueryDriver } from '@dna-platform/crdt-lww/build/modules/drivers/query/drizzle.query.driver';
import { StoreModule } from '@dna-platform/crdt-lww/build/store/store.module';
@Module({
  imports: [
    CoreModule.register({
      imports: [
        DriversModule.forRoot([
          {
            useClass: DrizzleQueryDriver,
            provide: QueryDriverInterface,
          },
        ]),
        StoreModule,
      ],
    }),
    AppModule,
  ],
})
export class MainModule {}
