import { Inject, Injectable, OnModuleInit } from '@nestjs/common';
import { LoggerService } from '@dna-platform/common';
import {
  HelperService,
  //  IImplementationFunctions,
  IImplementationProviders,
  IMachineProperties,
  MachineInit,
} from '@dna-platform/common';
import {
  IActions,
  IActors,
  IGuards,
} from '../../schemas/create_hypertables/create_hypertables.schema';
import {
  CreateHypertablesActionsImplementations,
  CreateHypertablesActorsImplementations,
  CreateHypertablesGuardsImplementations,
} from '../../implementations/create_hypertables';
import * as path from 'path';
@Injectable()
export class CreateHypertablesMachine
  extends MachineInit
  implements IMachineProperties, OnModuleInit
{
  public readonly name = this.constructor.name;
  public readonly actions: IActions;
  //public readonly delays: IImplementationFunctions<any>;
  public readonly guards: IGuards;
  public readonly actors: IActors;
  constructor(
    @Inject('IMPLEMENTATIONS')
    implementation_providers: IImplementationProviders[],
    logger: LoggerService,
    private ai: CreateHypertablesActionsImplementations,
    private si: CreateHypertablesActorsImplementations,
    private gi: CreateHypertablesGuardsImplementations,
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
    // Define the correct path to the machine create_hypertables config
    this.machine_config_path = path.resolve(
      __dirname,
      `../../configs/${this.helper.snakeCase(
        this.name.replace('Machine', ''),
      )}.config`,
    );
  }
}
