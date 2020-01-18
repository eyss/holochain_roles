use hdk::{
    error::ZomeApiResult,
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::{Address, AddressableContent},
    prelude::*,
};

use serde_derive::{Deserialize, Serialize};

use crate::{AGENT_ASSIGNMENT_LINK_TYPE, ASSIGNMENT_TYPE};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Assignment {
    role_address: Address,
    agent_address: Address,
    metadata: Option<JsonString>,
}

pub fn assignment_entry_definition() -> ValidatingEntryType {
    entry!(
        name: ASSIGNMENT_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Assignment>| {
            Ok(())
        },
        links: [
            from!(
                "%agent_id",
                link_type: AGENT_ASSIGNMENT_LINK_TYPE,
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
