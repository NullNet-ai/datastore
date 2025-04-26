import { PromiseActorLogic } from 'xstate';
import {
  IPayload,
  TRootEvent,
  IRootContext,
  IActorInput,
  IRootActors,
  IRootActions,
  IRootGuards,
} from '@dna-platform/common';
import { GuardArgs } from 'xstate/dist/declarations/src/guards';

export enum EEvents {
  RESTART = 'RESTART',
}

export interface IFindContext extends IRootContext {
  [key: string]: any;
}

export type IFindEvent =
  | TRootEvent
  | {
      type: EEvents.RESTART;
      payload?: IPayload;
    };

export interface IActors extends IRootActors {
  [key: string]: any;
  find: PromiseActorLogic<Record<string, any>, IActorInput>;
}

export interface IActions extends IRootActions {
  findEntry: () => void;
}

export interface IGuards extends IRootGuards {
  [key: string]: any;
  hasControllerArgs: (input: GuardArgs<IFindContext, IFindEvent>) => boolean;
}

export enum EOperator {
  EQUAL = 'equal',
  NOT_EQUAL = 'not_equal',
  GREATER_THAN = 'greater_than',
  GREATER_THAN_OR_EQUAL = 'greater_than_or_equal',
  LESS_THAN = 'less_than',
  LESS_THAN_OR_EQUAL = 'less_than_or_equal',
  CONTAINS = 'contains',
  NOT_CONTAINS = 'not_contains',
  IS_EMPTY = 'is_empty',
  IS_NOT_EMPTY = 'is_not_empty',
  IS_NULL = 'is_null',
  IS_NOT_NULL = 'is_not_null',
  IS_BETWEEN = 'is_between',
  IS_NOT_BETWEEN = 'is_not_between',
  AND = 'and',
  OR = 'or',
  LIKE = 'like',
  NOT_LIKE = 'not_like',
}

export enum EOrderDirection {
  DESC = 'desc',
  DESCENDING = 'descending',
  ASC = 'asc',
  ASCENDING = 'ascending',
}

export interface IGroupAdvanceFilters<f = string> {
  type: 'criteria' | 'operator';
  operator: EOperator;
  filters: IAdvanceFilters<f>[];
}
export enum EAllowedMutation {
  INSERT = 'INSERT',
  UPDATE = 'UPDATE',
}
/**
 * Determines if the operator is a single value operator.
 * Single value operators include:
 * - EOperator.EQUAL
 * - EOperator.NOT_EQUAL
 * - EOperator.GREATER_THAN
 * - EOperator.GREATER_THAN_OR_EQUAL
 * - EOperator.LESS_THAN
 * - EOperator.LESS_THAN_OR_EQUAL
 */
export const is_single_value_operators = (operator: EOperator) =>
  [
    EOperator.EQUAL,
    EOperator.NOT_EQUAL,
    EOperator.GREATER_THAN,
    EOperator.GREATER_THAN_OR_EQUAL,
    EOperator.LESS_THAN,
    EOperator.LESS_THAN_OR_EQUAL,
  ].includes(operator);
/**
 * Determines if the operator is a range value operator.
 * Single value operators include:
 * - EOperator.IS_BETWEEN
 * - EOperator.IS_NOT_BETWEEN
 */
export const is_range_value_operators = (operator: EOperator) =>
  [EOperator.IS_BETWEEN, EOperator.IS_NOT_BETWEEN].includes(operator);
/**
 * Represents an interface for advanced filters.
 *
 * @template f - The type of the field.
 */
export interface IAdvanceFilters<f = string> {
  field: f;
  operator: EOperator;
  values?: string[] | number[] | boolean[] | Date[];
  logical_operator?: 'AND' | 'OR';
  type: 'criteria' | 'operator';
  entity?: string;
  case_sensitive?: boolean;
  parse_as?: 'text';
}
export interface IWhereClauses {
  and: Array<IAdvanceFilters>;
  or: Array<IAdvanceFilters>;
}

export interface IOrder<fs = Record<string, any>> {
  /**
   * @description - limit is the number of records to return.
   */
  limit: number;
  /**
   * @description - start_at is the starting point of the query.
   */
  start_at?: number;
  /**
   * @description - ends_at is the ending point of the query.
   */
  ends_at?: number;
  by_direction?: EOrderDirection;
  by_field?: keyof fs;
}

export interface IJoins {
  type: 'inner' | 'left' | 'right' | 'full' | 'self';
  field_relation: {
    from: {
      alias?: string;
      entity: string;
      field: string;
    };
    to: {
      alias?: string;
      entity: string;
      field: string;
    };
  };
}
