# Cw-Shit-Strap

Shitstrap is a single use contract, for permissionless, non-custodial OTC token swaps. 

## Description 
Each shitstrap instance has an owner that will recieve all assets accumulated during a shitstrap. There can be a list of accepted tokens (cw20 & native tokens are supported), each with their own specific `shit_rate`, which is the ratio uses to determine the amount of tokens that will be sent to a shitstrap participant, specific to each token eligible. There is a `cutoff` limit that serves to end a shitstrap, once this amount of tokens is distributed to shitstrap participants. Any excess funds send resulting in the `cutoff` limit to be reached will be able to be redeemed by the rightful participant. 

## Conversion Ratios  
Each `shit_rate` is focused on how many tokens the contract wil send to the shitstrapper, in exchange for 1 `possible_shit` token. For example, if i would like to accept 2 OSMO for 1 SHITMOS, therefore i would set the `shit_rate` for `"uosmo"` to: `500000000000000000`. We  define the decimal precision to 18. All this means is that a 1:1 conversion can be defined as `1000000000000000000`.
## Deployments 

Network | code-id | contract-addr ||
--- | --- | --- | --- | 
Stargaze `elgafar-1` | - | 
Osmosis `osmosis-test-5` | 11043 | osmo10jsnt4rhfsr7w50z3vg3ghfxy98fassnsxnmdypfuvnzzhscsegqsf9432


## Scripts
build contract
```sh
  docker run --rm -t -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer-arm64:0.16.0
```

## Create A New Shitstrap
```json
{
    "owner":"<owner_addr>",
    "accepted": [
        {
            "token":{"native": "token-addr"} , 
            "shit_rate": 1500000,  // for every 1 token sent, get 1.5 shitmos
        },
        {
            "token":{"cw20": "cw20-contract-addr"} , 
            "shit_rate": 500000,  // for every 1 token sent, get 0.5 shitmos
        },
    ],
    "cutoff": "222",
    "shitmos": "<shitmos-addr-to-deploy>"
}
```

## Participate In Shitstrap
```json
{
    "shit_strap": {
        "shit": {
            "denom": {"native": "your_denom_here"},
            "amount": "69"
        }
    }
}
```

## TEST
<!-- {
    "owner":"osmo1wrxvkvnu9fvucyaa9sxyecapgl7qft6kqmn8ft",
    "accepted": [
        {
            "token":{"native": "factory/osmo1vpudlpnuqwlpuc2q9yptjssp46snvf2twu3t2s/shitstrap2"} , 
            "shit_rate": "30000000",
        },
        {
            "token":{"native": "factory/osmo1vpudlpnuqwlpuc2q9yptjssp46snvf2twu3t2s/shitstrap3"} , 
            "shit_rate": "710000",  
        },
    {
            "token":{"native": "factory/osmo1vpudlpnuqwlpuc2q9yptjssp46snvf2twu3t2s/shitstrap4"} , 
            "shit_rate": "1000000",  
        },
    ],
    "cutoff": "500000000000",
    "shitmos": "factory/osmo1vpudlpnuqwlpuc2q9yptjssp46snvf2twu3t2s/shitstrap"
} -->

### Create contract with owner that does not have balance 
```sh
 st tx wasm i 4536 '{"owner": "stars14vdzxywqxlwm2mqnxmrxmat34jx854esjq4zqm","accepted":[{"token":{"native": "ustars"}, "shit_rate": "0500000"}],"cutoff":"100","shitmos":"ustars"}' --from test1 --gas auto --gas-adjustment 1.3 --fees 50000ustars --no-admin --label="test shitmos" --chain-id elgafar-1
 # hash: DAEAFC4CC24BF0001F8346F5181881EF75BB7B677BE03E01F6C372F81564EF6A
```

### Fund Shitstrap with 50 stars
```sh
st tx bank send stars1lzaedvps5jzwua87ph05v8cpzpfgf5yyy75ap9 stars15cpzs3cqm4taunc8c9ktpclgngmq5ea948txp4v08ejr7hs2llfss2qm6l  51000000ustars --from test1 --gas auto --gas-adjustment 1.3 --fees 1000ustars --chain-id elgafar-1
# hash: D1278D4FB60F71A9B9D44B5BA3247955BE597A7F406E5FA11559958158102017
```

### Send 200 stars to shitstrap, expect to get 50 back
```sh
st tx wasm e stars15cpzs3cqm4taunc8c9ktpclgngmq5ea948txp4v08ejr7hs2llfss2qm6l '{"shit_strap":{"shit":{"denom":{"native":"ustars"},"amount":"200000000"}}}' --amount 200000000ustars --from test5 --gas auto --gas-adjustment 1.3 --fees 25000ustars   --chain-id elgafar-1 
# hash: 3A70B62CF071FADD9C85C398114FEE91C882B463C10E149253F8DE5D39211BA8
```

### Confirm contract is now full of shit
```sh
st q wasm contract-state smart stars15cpzs3cqm4taunc8c9ktpclgngmq5ea948txp4v08ejr7hs2llfss2qm6l '{"full_of_shit":{}}' -o json
# returns: {"data":true}
```

### Confirm balances
```sh 

### Claim Refund if able to 
```sh

st q bank balances stars1exfqjt63s4v4msstw7fc2dgp90pg0e6v8sngy6 -o json # owner

st q bank balances stars1exfqjt63s4v4msstw7fc2dgp90pg0e6v8sngy6 -o json # shit strapper
# {"balances":[{"denom":"ustars","amount":"104975000"}],"pagination":{"next_key":null,"total":"0"}}
```