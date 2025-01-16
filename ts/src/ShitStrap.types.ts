/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Uint128 = string;
export type UncheckedDenom = {
  native: string;
} | {
  cw20: string;
};
export interface InstantiateMsg {
  accepted: PossibleShit[];
  cutoff: Uint128;
  description: string;
  owner: string;
  shitmos: UncheckedDenom;
  title: string;
}
export interface PossibleShit {
  shit_rate: Uint128;
  token: UncheckedDenom;
}
export type ExecuteMsg = {
  shit_strap: {
    shit: AssetUnchecked;
  };
} | {
  flush: {};
} | {
  receive: Cw20ReceiveMsg;
} | {
  refund_shitter: {};
};
export type Binary = string;
export interface AssetUnchecked {
  amount: Uint128;
  denom: UncheckedDenom;
}
export interface Cw20ReceiveMsg {
  amount: Uint128;
  msg: Binary;
  sender: string;
}
export type QueryMsg = {
  config: {};
} | {
  shit_pile: {};
} | {
  full_of_shit: {};
} | {
  shit_rate: {
    asset: string;
  };
} | {
  shit_rates: {};
};
export type Addr = string;
export type CheckedDenom = {
  native: string;
} | {
  cw20: Addr;
};
export interface Config {
  accepted: PossibleShit[];
  cutoff: Uint128;
  description: string;
  full_of_shit: boolean;
  owner: Addr;
  shitmos_addr: CheckedDenom;
  title: string;
}
export type Boolean = boolean;
export type NullableUint128 = Uint128 | null;
export type NullableArrayOfPossibleShit = PossibleShit[] | null;