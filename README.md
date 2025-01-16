# Shitstraps

Shitstrap is a disposiable (single use) smart contract, for permissionless, non-custodial OTC token swaps. In this workspace, you will find the following contracts:

- cw-shitstrap
- cw-shitstrap-factory

## Description 
Each shitstrap instance has an `owner` that will recieve all assets accumulated during a shitstrap. There can be a list of `accepted` tokens (cw20 & native tokens are supported), each with their own specific `shit_rate`, which is the ratio uses to determine the amount of tokens that will be sent to a shitstrap participant, specific to each token eligible. There is a `cutoff` limit that serves to end a shitstrap, once this amount of the tokens the contracts distributs to shitstrap participants has been sent. Any excess funds send resulting in the `cutoff` limit to be reached will be able to be redeemed by the rightful participant. 

## Conversion Ratios  
Each `shit_rate` is focused on how many tokens the contract wil send to the shitstrapper, in exchange for 1 `possible_shit` token. For example, if i would like to accept 2 OSMO for 1 SHITMOS, therefore i would set the `shit_rate` for `"uosmo"` to: `500000000000000000`. We  define the decimal precision to 18. All this means is that a 1:1 conversion can be defined as `1000000000000000000`.

## Participating in a Shitstrap
 When a participate calls the  `execute_shit_strap` entry point, the contract first checks if it is still eligible. Then, we find if any accepted token for this shitstrap was sent in the message. We verify the correct amount is sent, and then calculate the converted amount of shitstrap tokens to send in exchange for the tokens sent with `calculate_shit_value`. This takes the ratio and multiplies it with the amount of tokens. If this value added to the `CURRENT_SHITSTRAP_VALUE` is greater than the cutoff set for this shitstrap, the shitstrap is complete and we determine the overflow amount of tokens sent to the contract the last shitter is eligible to redeem.
 
`CURRENT_SHITSTRAP_VALUE` stores the calculated tokens ratio from tokens sent to the contract, and is referenced & updated every time an eligible token is sent to a shitstrap.

## Shitstrapping Funds
When a shitstrap is active, each time a shitstrap payment is made, the tokens are held by the shitstrap until it is set to `full_of_shit`, either manually by the admin or by the cutoff limit being reached. 

## Deployments 