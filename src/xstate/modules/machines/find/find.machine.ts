import { Inject, Injectable, OnModuleInit } from '@nestjs/common';
import {
  HelperService,
  //  IImplementationFunctions,
  IImplementationProviders,
  IMachineProperties,
  LoggerService,
  MachineInit,
} from '@dna-platform/common';
import { IActions, IActors, IGuards } from '../../schemas/find/find.schema';
import {
  FindActionsImplementations,
  FindActorsImplementations,
  FindGuardsImplementations,
} from '../../implementations/find';
import * as path from 'path';
@Injectable()
export class FindMachine
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
    private ai: FindActionsImplementations,
    private si: FindActorsImplementations,
    private gi: FindGuardsImplementations,
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
    // Define the correct path to the machine find config
    this.machine_config_path = path.resolve(
      __dirname,
      `../../configs/${this.helper.snakeCase(
        this.name.replace('Machine', ''),
      )}.config`,
    );
  }
}
