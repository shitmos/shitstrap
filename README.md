# Cw-Shit-Strap

## Description 

## Build 
```sh
  docker run --rm -t -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.16.0
```

## Deployments 


Stargaze 

Network | code-id | ||
--- | --- | --- | --- | 
Stargaze `elgafar-1` | 4530 | 
## Scripts

instantiate contract
```json
{
    "owner":"<owner_addr>",
    "accepted": [
        {
            "token":{"native": "token-addr"} , 
            "shit_rate": 150, 
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
    "shitstrap": {
        "shit": {
            "denom": {"native": "your_denom_here"},
            "amount": "69"
        }
    }
}
```

