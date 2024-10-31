import { Global, Module } from '@nestjs/common';
import { StoreDriverInterface } from './store/enums';
import { TransportDriverInterface } from './transport/enums';
import { HttpTransportDriver } from './transport/http.driver';
import { QueryDriverInterface } from './query/enums';
import { DrizzleQueryDriver } from './query/drizzle.query.driver';
import { DrizzleStoreDriver } from './store/drizzle.store.driver';
const providers = [
  {
    useClass: HttpTransportDriver,
    provide: TransportDriverInterface,
  },
  {
    useClass: DrizzleStoreDriver,
    provide: StoreDriverInterface,
  },
  {
    useClass: DrizzleQueryDriver,
    provide: QueryDriverInterface,
  },
];

@Global()
@Module({
  providers,
  exports: [...providers],
})
export class DriversModule {
  static forRoot() {
    return {
      module: DriversModule,
      providers,
      exports: [...providers],
    };
  }
}
