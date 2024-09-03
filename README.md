# Cw-Shit-Strap

## Description 

## Deployments 

Network | code-id | ||
--- | --- | --- | --- | 
Stargaze `elgafar-1` | 4532 | 


## Scripts
build contract
```sh
  docker run --rm -t -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.16.0
```

instantiate contract
```json
{
    "owner":"<owner_addr>",
    "accepted": [
        {
            "token":{"native": "token-addr"} , 
            "shit_rate": 150,  // for every 1 token sent, get 1.5 shitmos30
        },
        {
            "token":{"cw20": "cw20-contract-addr"} , 
            "shit_rate": 75, 
        },
    ],
    "cutoff": "222",
    "shitmos": "<shitmos-addr-to-deploy>"
}
```

participate in shitmos
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

### Fund Shitstrap with expected token
```sh
st tx bank send stars1lzaedvps5jzwua87ph05v8cpzpfgf5yyy75ap9 stars1qps4yd23n6vmmulemc7lcglls8mnlpu90vfy9khsszlnf99l87ys5432p8\
56000000ustars --from test1 --gas auto --gas-adjustment 1.3 --fees 1000ustars --chain-id elgafar-1
# confirmed balance: st q bank balances stars1qps4yd23n6vmmulemc7lcglls8mnlpu90vfy9khsszlnf99l87ys5432p8 
```

### Send 150 stars to shitstrap
```sh
st tx wasm e stars1qps4yd23n6vmmulemc7lcglls8mnlpu90vfy9khsszlnf99l87ys5432p8 \
'{"shit_strap":{"shit":{"denom":{"native":"ustars"},"amount":"150000000"}}}' \
--amount 150000000ustars --from test1 --gas auto --gas-adjustment 1.3 --fees 25000ustars --chain-id elgafar-1
# hash: 6879C2901D6E8AF5DA49C80B9BDE707A9523C7A1C39D01F9E1417EC7290AAEA3
```

### Confirm contract is now full of shit
```json
// st q wasm contract-state smart stars1qps4yd23n6vmmulemc7lcglls8mnlpu90vfy9khsszlnf99l87ys5432p8 '{"full_of_shit":{}}'
{"data":true}
```