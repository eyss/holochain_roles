use hdk::{
    error::ZomeApiResult, holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address, prelude::*,
};

use serde_derive::{Deserialize, Serialize};

use crate::admins;
use crate::{ADMINISTRATOR_ROLE, ASSIGNMENT_TYPE, ROLE_ASSIGNMENT_LINK_TYPE, ROLE_TYPE};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Role {
    name: String,
}

impl Role {
    pub fn from(name: String) -> Role {
        Role { name }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(ROLE_TYPE.into(), self.into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        let entry = self.entry();
        hdk::entry_address(&entry)
    }
}

pub fn role_entry_definition() -> ValidatingEntryType {
    entry!(
        name: ROLE_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: | _validation_data: hdk::EntryValidationData<Role>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    if entry.name == ADMINISTRATOR_ROLE {
                        return Ok(());
                    }

                    let chain_entries = validation_data.package.source_chain_entries.unwrap();

                    let admin = admins::is_agent_admin(&chain_entries)?;

                    match admin {
                        true => Ok(()),
                        false => Err(String::from("Only admins can create roles"))
                    }
                },
                hdk::EntryValidationData::Modify {old_entry, new_entry,  .. } => {
                    if old_entry.name != new_entry.name {
                        return Err(ZomeApiError::from(String::from("Cannot modify the contents of the entry")))?;
                    }

                    Ok(())
                },
                _ => Err(String::from("Cannot delete roles"))
            }
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
