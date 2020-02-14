#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};
use holochain_wasm_utils::api_serialization::get_entry::GetEntryResultItem;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk::holochain_core_types::{crud_status::CrudStatus, time::Iso8601};
use hdk::prelude::*;
use hdk_proc_macros::zome;
use holochain_roles;

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn custom_entry() -> ValidatingEntryType {
        entry!(
            name: "test",
            description: "a test entry to validate that roles are working",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<String>| {
                match _validation_data {
                    hdk::EntryValidationData::Create { validation_data, .. } => {
                        validate_required_role(&validation_data, &String::from(holochain_roles::ADMIN_ROLE_NAME))?;

                        Ok(())
                    },
                    _ => Err(String::from("Cannot modify roles"))
                }
            }
        )
    }
    #[zome_fn("hc_public")]
    fn create_test_entry(test: String) -> ZomeApiResult<Address> {
        let entry = Entry::App("test".into(), JsonString::from_json(test.as_str()));
        hdk::commit_entry(&entry)
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        holochain_roles::role_assignment_entry_def()
    }

    #[entry_def]
    fn anchors_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_role_agents(role_name: String) -> ZomeApiResult<Vec<Address>> {
        holochain_roles::handlers::get_role_agents(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Vec<String>> {
        holochain_roles::handlers::get_agent_roles(&agent_address)
    }
}

/**
 * Validates that the agent that have signed this entry had the given role at the time they commited the entry
 */
pub fn validate_required_role(
    validation_data: &hdk::ValidationData,
    role_name: &String,
) -> Result<(), String> {
    let agent_address = &validation_data.sources()[0];

    let progenitor_address = holochain_roles::progenitor::get_progenitor_address()?;

    if role_name.clone() == String::from("Admin") && progenitor_address == agent_address.clone() {
        return Ok(());
    }

    let timestamp = &validation_data.package.chain_header.timestamp();
    let entry_address = &validation_data.package.chain_header.entry_address();

    match had_agent_role(&agent_address, &role_name, &timestamp)? {
        true => Ok(()),
        false => Err(format!(
            "Agent {} did not have the role {} when committing entry {}",
            agent_address, role_name, entry_address
        )),
    }
}

/**
 * Returns whether the given agent had been assigned to a certain role in the given time
 */
pub fn had_agent_role(
    agent_address: &Address,
    role_name: &String,
    timestamp: &Iso8601,
) -> ZomeApiResult<bool> {
    let role = holochain_roles::RoleAssignment::from(role_name.clone(), agent_address.clone());

    let role_address = role.address()?;
    match get_entry_history_with_meta(&role_address)? {
        None => Ok(false),
        Some(history) => {
            let maybe_item_index = history.items.iter().position(|item| {
                let timestamps: Vec<&Iso8601> = item
                    .headers
                    .iter()
                    .map(|header| header.timestamp())
                    .collect();

                timestamps.iter().min().unwrap().clone() > timestamp
            });

            match maybe_item_index {
                None => Ok(is_item_alive(history.items.last())),
                Some(item_index) => {
                    let item = history.items.get(item_index - 1);
                    Ok(is_item_alive(item))
                }
            }
        }
    }
}

fn is_item_alive(maybe_item: Option<&GetEntryResultItem>) -> bool {
    if let Some(item) = maybe_item {
        let result = match item.meta.clone().unwrap().crud_status {
            CrudStatus::Deleted => false,
            CrudStatus::Rejected => false,
            CrudStatus::Locked => false,
            _ => true,
        };
        return result;
    }

    return false;
}

fn get_entry_history_with_meta(address: &Address) -> ZomeApiResult<Option<EntryHistory>> {
    let entry_result = hdk::get_entry_result(
        address,
        GetEntryOptions::new(StatusRequestKind::All, true, true, Default::default()),
    )?;
    if !entry_result.found() {
        return Ok(None);
    }
    match entry_result.result {
        GetEntryResultType::All(history) => Ok(Some(history)),
        _ => Err(ZomeApiError::from("shouldn't happen".to_string())),
    }
}
