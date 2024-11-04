import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import {
  StoreModule as MachineStoreModule,
  shared_imports,
} from '../../controllers/store/store.module';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww';
import { QueryDriverInterface } from '@dna-platform/crdt-lww/build/modules/drivers/query/enums';
import { StoreQueryDriver } from '../../providers/store/store.service';
@Module({
  imports: [
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
          ],
        }),
        // StoreModule,
      ],
    }),
  ],
  controllers: [AppController],
})
export class AppModule {}
