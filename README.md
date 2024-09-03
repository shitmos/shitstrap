# Cw-Shit-Strap

 st tx wasm i 4536 '{"owner": "stars14vdzxywqxlwm2mqnxmrxmat34jx854esjq4zqm","accepted":[{"token":{"native": "ustars"}, "shit_rate": "0500000"}],"cutoff":"100","shitmos":"ustars"}' --from test1 --gas auto --gas-adjustment 1.3 --fees 50000ustars --no-admin --label="test shitmos" --chain-id elgafar-1
## Description 

## Deployments 

Network | code-id | ||
--- | --- | --- | --- | 
Stargaze `elgafar-1` | 4536 | 


## Scripts
build contract
```sh
  docker run --rm -t -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer-arm64:0.16.0
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