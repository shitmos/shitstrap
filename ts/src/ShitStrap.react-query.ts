/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from "@tanstack/react-query";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee, Coin } from "@cosmjs/amino";
import { Uint128, UncheckedDenom, InstantiateMsg, PossibleShit, ExecuteMsg, Binary, AssetUnchecked, Cw20ReceiveMsg, QueryMsg, Addr, CheckedDenom, Config, Boolean, NullableUint128, NullableArrayOfPossibleShit } from "./ShitStrap.types";
import { ShitStrapQueryClient, ShitStrapClient } from "./ShitStrap.client";
export const shitStrapQueryKeys = {
  contract: ([{
    contract: "shitStrap"
  }] as const),
  address: (contractAddress: string) => ([{ ...shitStrapQueryKeys.contract[0],
    address: contractAddress
  }] as const),
  config: (contractAddress: string, args?: Record<string, unknown>) => ([{ ...shitStrapQueryKeys.address(contractAddress)[0],
    method: "config",
    args
  }] as const),
  shitPile: (contractAddress: string, args?: Record<string, unknown>) => ([{ ...shitStrapQueryKeys.address(contractAddress)[0],
    method: "shit_pile",
    args
  }] as const),
  fullOfShit: (contractAddress: string, args?: Record<string, unknown>) => ([{ ...shitStrapQueryKeys.address(contractAddress)[0],
    method: "full_of_shit",
    args
  }] as const),
  shitRate: (contractAddress: string, args?: Record<string, unknown>) => ([{ ...shitStrapQueryKeys.address(contractAddress)[0],
    method: "shit_rate",
    args
  }] as const),
  shitRates: (contractAddress: string, args?: Record<string, unknown>) => ([{ ...shitStrapQueryKeys.address(contractAddress)[0],
    method: "shit_rates",
    args
  }] as const)
};
export const shitStrapQueries = {
  config: <TData = Config,>({
    client,
    options
  }: ShitStrapConfigQuery<TData>): UseQueryOptions<Config, Error, TData> => ({
    queryKey: shitStrapQueryKeys.config(client?.contractAddress),
    queryFn: () => client.config(),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  shitPile: <TData = Uint128,>({
    client,
    options
  }: ShitStrapShitPileQuery<TData>): UseQueryOptions<Uint128, Error, TData> => ({
    queryKey: shitStrapQueryKeys.shitPile(client?.contractAddress),
    queryFn: () => client.shitPile(),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  fullOfShit: <TData = Boolean,>({
    client,
    options
  }: ShitStrapFullOfShitQuery<TData>): UseQueryOptions<Boolean, Error, TData> => ({
    queryKey: shitStrapQueryKeys.fullOfShit(client?.contractAddress),
    queryFn: () => client.fullOfShit(),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  shitRate: <TData = NullableUint128,>({
    client,
    args,
    options
  }: ShitStrapShitRateQuery<TData>): UseQueryOptions<NullableUint128, Error, TData> => ({
    queryKey: shitStrapQueryKeys.shitRate(client?.contractAddress, args),
    queryFn: () => client.shitRate({
      asset: args.asset
    }),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  }),
  shitRates: <TData = NullableArrayOfPossibleShit,>({
    client,
    options
  }: ShitStrapShitRatesQuery<TData>): UseQueryOptions<NullableArrayOfPossibleShit, Error, TData> => ({
    queryKey: shitStrapQueryKeys.shitRates(client?.contractAddress),
    queryFn: () => client.shitRates(),
    ...options,
    enabled: !!client && (options?.enabled != undefined ? options.enabled : true)
  })
};
export interface ShitStrapReactQuery<TResponse, TData = TResponse> {
  client: ShitStrapQueryClient;
  options?: Omit<UseQueryOptions<TResponse, Error, TData>, "'queryKey' | 'queryFn' | 'initialData'"> & {
    initialData?: undefined;
  };
}
export interface ShitStrapShitRatesQuery<TData> extends ShitStrapReactQuery<NullableArrayOfPossibleShit, TData> {}
export function useShitStrapShitRatesQuery<TData = NullableArrayOfPossibleShit>({
  client,
  options
}: ShitStrapShitRatesQuery<TData>) {
  return useQuery<NullableArrayOfPossibleShit, Error, TData>(shitStrapQueryKeys.shitRates(client.contractAddress), () => client.shitRates(), options);
}
export interface ShitStrapShitRateQuery<TData> extends ShitStrapReactQuery<NullableUint128, TData> {
  args: {
    asset: string;
  };
}
export function useShitStrapShitRateQuery<TData = NullableUint128>({
  client,
  args,
  options
}: ShitStrapShitRateQuery<TData>) {
  return useQuery<NullableUint128, Error, TData>(shitStrapQueryKeys.shitRate(client.contractAddress, args), () => client.shitRate({
    asset: args.asset
  }), options);
}
export interface ShitStrapFullOfShitQuery<TData> extends ShitStrapReactQuery<Boolean, TData> {}
export function useShitStrapFullOfShitQuery<TData = Boolean>({
  client,
  options
}: ShitStrapFullOfShitQuery<TData>) {
  return useQuery<Boolean, Error, TData>(shitStrapQueryKeys.fullOfShit(client.contractAddress), () => client.fullOfShit(), options);
}
export interface ShitStrapShitPileQuery<TData> extends ShitStrapReactQuery<Uint128, TData> {}
export function useShitStrapShitPileQuery<TData = Uint128>({
  client,
  options
}: ShitStrapShitPileQuery<TData>) {
  return useQuery<Uint128, Error, TData>(shitStrapQueryKeys.shitPile(client.contractAddress), () => client.shitPile(), options);
}
export interface ShitStrapConfigQuery<TData> extends ShitStrapReactQuery<Config, TData> {}
export function useShitStrapConfigQuery<TData = Config>({
  client,
  options
}: ShitStrapConfigQuery<TData>) {
  return useQuery<Config, Error, TData>(shitStrapQueryKeys.config(client.contractAddress), () => client.config(), options);
}
export interface ShitStrapRefundShitterMutation {
  client: ShitStrapClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useShitStrapRefundShitterMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ShitStrapRefundShitterMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ShitStrapRefundShitterMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.refundShitter(fee, memo, funds), options);
}
export interface ShitStrapReceiveMutation {
  client: ShitStrapClient;
  msg: {
    amount: Uint128;
    msg: Binary;
    sender: string;
  };
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useShitStrapReceiveMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ShitStrapReceiveMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ShitStrapReceiveMutation>(({
    client,
    msg,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.receive(msg, fee, memo, funds), options);
}
export interface ShitStrapFlushMutation {
  client: ShitStrapClient;
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useShitStrapFlushMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ShitStrapFlushMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ShitStrapFlushMutation>(({
    client,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.flush(fee, memo, funds), options);
}
export interface ShitStrapShitStrapMutation {
  client: ShitStrapClient;
  msg: {
    shit: AssetUnchecked;
  };
  args?: {
    fee?: number | StdFee | "auto";
    memo?: string;
    funds?: Coin[];
  };
}
export function useShitStrapShitStrapMutation(options?: Omit<UseMutationOptions<ExecuteResult, Error, ShitStrapShitStrapMutation>, "mutationFn">) {
  return useMutation<ExecuteResult, Error, ShitStrapShitStrapMutation>(({
    client,
    msg,
    args: {
      fee,
      memo,
      funds
    } = {}
  }) => client.shitStrap(msg, fee, memo, funds), options);
}