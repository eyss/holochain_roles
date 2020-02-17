#![feature(vec_remove_item)]

use hdk::prelude::*;
use holochain_anchors;
use serde_derive::{Deserialize, Serialize};

pub mod handlers;
pub mod progenitor;
pub mod validation;

pub const ROLE_ASSIGNMENT_TYPE: &'static str = "role_assignment";
pub const AGENT_TO_ASSIGNMENT_LINK_TYPE: &'static str = "agent->role_assignment";
pub const ROLE_TO_ASSIGNMENT_LINK_TYPE: &'static str = "role->role_assignment";
pub const ADMIN_ROLE_NAME: &'static str = "Admin";
pub const ANCHOR_LINK_TYPE: &'static str = concat!("holochain_anchors", "::", "anchor_link");

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct RoleAssignment {
    pub role_name: String,
    pub agent_address: Address,
    pub previous_assignment_address: Option<Address>,
    pub assigned: bool,
}

impl RoleAssignment {
    pub fn initial(role_name: String, agent_address: Address) -> RoleAssignment {
        RoleAssignment {
            role_name,
            agent_address,
            previous_assignment_address: None,
            assigned: true,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App(ROLE_ASSIGNMENT_TYPE.into(), self.into())
    }

    pub fn initial_address(&self) -> ZomeApiResult<Address> {
        let initial_role_entry =
            RoleAssignment::initial(self.role_name.clone(), self.agent_address.clone());

        hdk::entry_address(&initial_role_entry.entry())
    }
}

/**
 * Role assignment entry definition
 * This function must be called from the zome entry point for this mixin to be setup properly
 */
pub fn role_assignment_entry_def() -> ValidatingEntryType {
    entry!(
        name: ROLE_ASSIGNMENT_TYPE,
        description: "role assignment entry that contains a role name and the members of that role",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<RoleAssignment>| {
            match _validation_data {
                hdk::EntryValidationData::Create { validation_data, entry } => {
                    if let Some(_) = entry.previous_assignment_address {
                        return Err(String::from("Cannot create a role with a previous entry address"));
                    }

                    validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                },
                hdk::EntryValidationData::Modify { validation_data, .. } => {
                    validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                },
                hdk::EntryValidationData::Delete { validation_data, .. } => {
                    validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                },
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: AGENT_TO_ASSIGNMENT_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    match _validation_data {
                        hdk::LinkValidationData::LinkAdd { validation_data, .. } => {
                            validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                        },
                        hdk::LinkValidationData::LinkRemove { validation_data, .. } => {
                            validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                        },
                    }
                }
            ),
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: ROLE_TO_ASSIGNMENT_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    match _validation_data {
                        hdk::LinkValidationData::LinkAdd { validation_data, .. } => {
                            validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                        },
                        hdk::LinkValidationData::LinkRemove { validation_data, .. } => {
                            validation::validate_required_role(&validation_data, &String::from(ADMIN_ROLE_NAME))
                        },
                    }
                }
            )
        ]
    )
}
