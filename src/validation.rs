use holochain_wasm_utils::api_serialization::get_entry::GetEntryResultItem;
use crate::progenitor;
use crate::RoleAssignment;
use crate::ADMIN_ROLE_NAME;
use hdk::holochain_core_types::{crud_status::CrudStatus, time::Iso8601};
use hdk::prelude::*;

/**
 * Validates that the agent that have signed this entry had the given role at the time they commited the entry
 */
pub fn validate_required_role(
    validation_data: &hdk::ValidationData,
    role_name: &String,
) -> Result<(), String> {
    let agent_address = &validation_data.sources()[0];

    let progenitor_address = progenitor::get_progenitor_address()?;

    if role_name == crate::ADMIN_ROLE_NAME && progenitor_address == agent_address.clone() {
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
    let role = RoleAssignment::from(role_name.clone(), agent_address.clone());

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

/**
 * Returns whether the given agent has been assigned to the given role
 */
pub fn has_agent_role(agent_address: &Address, role_name: &String) -> ZomeApiResult<bool> {
    let role = RoleAssignment::from(role_name.clone(), agent_address.clone());

    let role_address = role.address()?;

    match hdk::get_entry(&role_address)? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

/**
 * Returns whether the given agent is an administrator and, as such,
 * can create, assign  and unassign roles
 */
pub fn is_agent_admin(agent_address: &Address) -> ZomeApiResult<bool> {
    let progenitor_address = progenitor::get_progenitor_address()?;

    if progenitor_address == agent_address.clone() {
        return Ok(true);
    }
    let result = has_agent_role(&agent_address, &String::from(ADMIN_ROLE_NAME))?;

    Ok(result)
}
