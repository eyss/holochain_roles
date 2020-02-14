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
) -> ZomeApiResult<()> {
    let agent_address = &validation_data.sources()[0];

    let progenitor_address = progenitor::get_progenitor_address()?;

    if progenitor_address == agent_address.clone() {
        return Ok(());
    }

    let timestamp = &validation_data.package.chain_header.timestamp();
    let entry_address = &validation_data.package.chain_header.entry_address();

    match had_agent_role(&agent_address, &role_name, &timestamp)? {
        true => Ok(()),
        false => Err(ZomeApiError::from(format!(
            "Agent {} did not have the role {} when committing entry {}",
            agent_address, role_name, entry_address
        ))),
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

    match hdk::get_entry_history(&role_address)? {
        None => Ok(false),
        Some(history) => {
            let maybe_item = history.items.iter().find(|item| {
                let timestamps: Vec<&Iso8601> = item
                    .headers
                    .iter()
                    .map(|header| header.timestamp())
                    .collect();

                timestamps.iter().min().unwrap().clone() > timestamp
            });

            match maybe_item {
                None => Ok(false),
                Some(item) => match item.meta.clone().unwrap().crud_status {
                    CrudStatus::Deleted => Ok(false),
                    CrudStatus::Rejected => Ok(false),
                    CrudStatus::Locked => Ok(false),
                    _ => Ok(true),
                },
            }
        }
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
