import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import {
  StoreModule as MachineStoreModule,
  shared_imports,
} from '../../controllers/store/store.module';
import { GlobalModule } from '../../providers/global/global.module';
import { CoreModule, DriversModule } from '@dna-platform/crdt-lww-postgres';
import { QueryDriverInterface } from '@dna-platform/crdt-lww-postgres/build/modules/drivers/query/enums';
import {
  InitializerService,
  StoreQueryDriver,
} from '../../providers/store/store.service';
import { TimelineService } from '../../providers/timeline/timeline.service';
import { LoggerService, HelperService } from '@dna-platform/common';
import { ConfigModule } from '@nestjs/config';
import { AppService } from '../../providers/app/app.service';
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
          ],
        }),
        // StoreModule,
      ],
    }),
  ],
  controllers: [AppController],
  providers: [InitializerService, AppService, HelperService, TimelineService],
})
export class AppModule {}
