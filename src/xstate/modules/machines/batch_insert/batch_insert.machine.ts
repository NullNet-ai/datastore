
import { Inject, Injectable, OnModuleInit } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import {
  HelperService,
  IImplementationFunctions,
  IImplementationProviders,
  IMachineProperties,
  MachineInit,
} from '@dna-platform/common';
import {
  IActions,
  IActors,
  IGuards,
} from '../../schemas/batch_insert/batch_insert.schema';
import {
  BatchInsertActionsImplementations,
  BatchInsertActorsImplementations,
  BatchInsertGuardsImplementations,
} from '../../implementations/batch_insert';
import * as path from 'path';
@Injectable()
export class BatchInsertMachine
  extends MachineInit
  implements IMachineProperties, OnModuleInit
{
  public readonly name = this.constructor.name;
  public readonly actions: IActions;
  public readonly delays: IImplementationFunctions<any>;
  public readonly guards: IGuards;
  public readonly actors: IActors;
  constructor(
    @Inject('IMPLEMENTATIONS')
    implementation_providers: IImplementationProviders[],
    logger: LoggerService,
    private ai: BatchInsertActionsImplementations,
    private si: BatchInsertActorsImplementations,
    private gi: BatchInsertGuardsImplementations,
    private helper: HelperService,
  ) {
    // @ts-ignore
    super(implementation_providers, logger);
    /**
     * Can add implementations directly here
     * ! Warning: this will merged the common implementations
     */
    this.actions = this.ai.actions;
    this.actors = this.si.actors;
    this.guards = this.gi.guards;
    // this.delays = {};
    // Define the correct path to the machine batch_insert config
    this.machine_config_path = path.resolve(
      __dirname,
      `../../configs/${this.helper.snakeCase(
        this.name.replace('Machine', ''),
      )}.config`,
    );
  }
}
