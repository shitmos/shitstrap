use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

/// Temporarily holds the address of the instantiator for use in submessages
pub const TMP_INSTANTIATOR_INFO: Item<Addr> = Item::new("tmp_instantiator_info");
pub const SHITSTRAP_CODE_ID: Item<u64> = Item::new("pci");

#[cw_serde]
pub struct ShitstrapContract {
    pub contract: String,
    pub instantiator: String,
    pub shit: String,
}

pub struct TokenIndexes<'a> {
    pub instantiator: MultiIndex<'a, String, ShitstrapContract, String>,
    pub shit: MultiIndex<'a, String, ShitstrapContract, String>,
}

impl IndexList<ShitstrapContract> for TokenIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<ShitstrapContract>> + '_> {
        let v: Vec<&dyn Index<ShitstrapContract>> = vec![&self.instantiator, &self.shit];
        Box::new(v.into_iter())
    }
}

pub fn shitstrap_contracts<'a>() -> IndexedMap<'a, &'a str, ShitstrapContract, TokenIndexes<'a>> {
    let indexes = TokenIndexes {
        instantiator: MultiIndex::new(
            |_pk: &[u8], d: &ShitstrapContract| d.instantiator.clone(),
            "shitstrap_contracts",
            "shitstrap_contracts__instantiator",
        ),
        shit: MultiIndex::new(
            |_pk: &[u8], d: &ShitstrapContract| d.shit.clone(),
            "shitstrap_contracts",
            "shitstrap_contracts__shit",
        ),
    };
    IndexedMap::new("shitstrap_contracts", indexes)
}
