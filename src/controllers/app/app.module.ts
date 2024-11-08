import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import {
  StoreModule as MachineStoreModule,
  shared_imports,
} from '../../controllers/store/store.module';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww';
import { QueryDriverInterface } from '@dna-platform/crdt-lww/build/modules/drivers/query/enums';
import { StoreQueryDriver } from '../../providers/store/store.service';
import * as schema from '../../schema';
import { AppViewModule } from 'src/views/app.view.module';
@Module({
  imports: [
    AppViewModule,
    MachineStoreModule,
    CoreModule.register({
      imports: [
        DriversModule.forRoot({
          imports: [...shared_imports],
          providers: [
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
