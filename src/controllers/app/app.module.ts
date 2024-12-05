import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import {
  StoreModule as MachineStoreModule,
  shared_imports,
} from '../../controllers/store/store.module';
import { GlobalModule } from 'src/providers/global/global.module';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww-postgres';
import { QueryDriverInterface } from '@dna-platform/crdt-lww-postgres/build/modules/drivers/query/enums';
import { StoreQueryDriver } from '../../providers/store/store.service';
import * as schema from '../../schema';
import { LoggerService } from '@dna-platform/common';
import { ConfigModule } from '@nestjs/config';
@Module({
  imports: [
    GlobalModule,
    MachineStoreModule,
    CoreModule.register({
      imports: [
        ConfigModule.forRoot({
          isGlobal: true,
        }),
        DriversModule.forRoot({
          imports: [...shared_imports],
          providers: [
            LoggerService,
            {
              useClass: StoreQueryDriver,
              provide: QueryDriverInterface,
            },
            {
              useValue: schema,
              provide: 'LOCAL_SCHEMA',
            },
          ],
        }),
        // StoreModule,
      ],
    }),
  ],
  controllers: [AppController],
})
export class AppModule {}
