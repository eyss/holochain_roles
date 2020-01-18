use hdk::{
    error::ZomeApiResult,
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::{Address, AddressableContent},
    prelude::*,
};

use serde_derive::{Deserialize, Serialize};

use crate::{ASSIGNMENT_TYPE, ROLE_ASSIGNMENT_LINK_TYPE, ROLE_TYPE};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Role {
    name: String,
}

pub fn role_entry_definition() -> ValidatingEntryType {
    entry!(
        name: ROLE_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Role>| {
            Ok(())
        },
        links: [
            to!(
                ASSIGNMENT_TYPE,
                link_type: ROLE_ASSIGNMENT_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
