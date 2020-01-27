#![feature(vec_remove_item)]

use hdk::prelude::*;
use holochain_anchors;
use serde_derive::{Deserialize, Serialize};

pub mod progenitor;
pub mod validation;
pub mod handlers;

pub const ROLE_TYPE: &'static str = "role";
pub const AGENT_TO_ROLE_LINK_TYPE: &'static str = "agent->role";
pub const ANCHOR_TO_ROLE_LINK_TYPE: &'static str = "anchor->role";
pub const ADMIN_ROLE_NAME: &'static str = "Admin";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Role {
    pub role_name: String,
    pub members: Vec<Address>,
}

impl Role {
    pub fn from(role_name: String, members: Vec<Address>) -> Role {
        Role { role_name, members }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("role".into(), self.into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        let initial_role_entry = Role::from(self.role_name.clone(), vec![]);

        hdk::entry_address(&initial_role_entry.entry())
    }
}

pub fn role_entry_def() -> ValidatingEntryType {
    entry!(
        name: ROLE_TYPE,
        description: "role for ",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Role>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    if entry.members.len() != 0 {
                        return Err(String::from("The first role entry cannot have any members"));
                    }

                    if entry.role_name == String::from(ADMIN_ROLE_NAME) {
                        return Ok(());
                    }

                    let agent_address = &validation_data.sources()[0];

                    match validation::is_agent_admin(&agent_address)? {
                        true => Ok(()),
                        false => Err(String::from("Only admins can create roles"))
                    }
                },
                hdk::EntryValidationData::Modify { validation_data, .. } => {
                    let agent_address = &validation_data.sources()[0];

                    match validation::is_agent_admin(&agent_address)? {
                        true => Ok(()),
                        false => Err(String::from("Only admins can assign roles"))
                    }
                },
                _ => Err(String::from("Cannot delete roles"))
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: AGENT_TO_ROLE_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: ANCHOR_TO_ROLE_LINK_TYPE,

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
