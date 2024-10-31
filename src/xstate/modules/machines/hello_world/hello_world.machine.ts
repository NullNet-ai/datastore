import { Inject, Injectable, Logger, OnModuleInit } from '@nestjs/common';
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
} from '../../schemas/hello_world/hello_world.schema';
import {
  HelloWorldActionsImplementations,
  HelloWorldActorsImplementations,
  HelloWorldGuardsImplementations,
} from '../../implementations/hello_world';
import * as path from 'path';
@Injectable()
export class HelloWorldMachine
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
    logger: Logger,
    private ai: HelloWorldActionsImplementations,
    private si: HelloWorldActorsImplementations,
    private gi: HelloWorldGuardsImplementations,
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
    // Define the correct path to the machine hello_world config
    this.machine_config_path = path.resolve(
      __dirname,
      `../../configs/${this.helper.snakeCase(
        this.name.replace('Machine', ''),
      )}.config`,
    );
  }
}
