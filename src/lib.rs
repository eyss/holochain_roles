use hdk::{
    error::ZomeApiResult,
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::{Address, AddressableContent},
    prelude::*,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Role {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Assignment<T> {
    role_address: Address,
    agent_address: Address,
    metadata: T,
}

pub fn role_entry_definition() -> ValidatingEntryType {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
