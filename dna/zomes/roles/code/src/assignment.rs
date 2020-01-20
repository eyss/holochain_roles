use hdk::{holochain_persistence_api::cas::content::Address, prelude::*, AGENT_ADDRESS};

use serde_derive::{Deserialize, Serialize};

use crate::role::Role;
use crate::{utils, validation};
use crate::{ADMINISTRATOR_ROLE, AGENT_ASSIGNMENT_LINK_TYPE, ASSIGNMENT_TYPE, ROLE_TYPE};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Assignment {
    pub role_address: Address,
    pub agent_address: Address,
    pub metadata: Option<JsonString>,
}

impl Assignment {
    pub fn from(role_address: &Address, agent_address: &Address) -> Assignment {
        Assignment {
            role_address: role_address.clone(),
            agent_address: agent_address.clone(),
            metadata: None,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(ASSIGNMENT_TYPE.into(), self.into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        let entry = self.entry();
        hdk::entry_address(&entry)
    }
}

pub fn assignment_entry_definition() -> ValidatingEntryType {
    entry!(
        name: ASSIGNMENT_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: | _validation_data: hdk::EntryValidationData<Assignment>| {
            match _validation_data {
                hdk::EntryValidationData::Create { validation_data, entry } => {
                    let role = Role::from(String::from(ADMINISTRATOR_ROLE));

                    if entry.role_address == role.address()? {
                        if let Some(_) = entry.metadata {
                            return Err(String::from("Cannot put metadata in an administrator assignment"));
                        }
                    }

                    let chain_entries = validation_data.clone().package.source_chain_entries.unwrap();

                    let role: Option<Role> = utils::find_entry_with_address(&chain_entries, ROLE_TYPE, entry.role_address)?;
                    if let None = role {
                        return Err(String::from("The role entry should always accompany the assignment entry"));
                    }

                    let chain_headers = validation_data.clone().package.source_chain_headers.unwrap();
                    let agents_addresses = validation_data.sources();

                    let admin = validation::is_some_agent_admin(&agents_addresses, &chain_entries, &chain_headers)?;

                    match admin {
                        Some(_) => Ok(()),
                        _ => Err(String::from("Only admins can create roles"))
                    }
                },
                _ => Err(String::from("Cannot modify or delete role assignments"))
            }
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

pub fn receive_own_assignment(name: String, metadata: Option<JsonString>) -> ZomeApiResult<()> {
    let role = Role::from(name);

    let role_address = role.address()?;

    let assignment = Assignment {
        agent_address: AGENT_ADDRESS.clone(),
        role_address: role_address.clone(),
        metadata,
    };

    let assignment_address = assignment.address()?;

    let maybe_role_entry = hdk::get_entry(&role_address)?;
    let maybe_assignment_entry = hdk::get_entry(&assignment_address)?;

    match (maybe_role_entry, maybe_assignment_entry) {
        (Some(role_entry), Some(assignment_entry)) => {
            hdk::commit_entry(&role_entry)?;

            hdk::commit_entry(&assignment_entry)?;

            Ok(())
        }
        _ => Err(ZomeApiError::from(String::from(
            "Cannot received an assignment that has not been created yet",
        ))),
    }
}
