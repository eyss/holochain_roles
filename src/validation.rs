use crate::progenitor;
use crate::RoleAssignment;
use crate::ADMIN_ROLE_NAME;
use crate::AGENT_TO_ASSIGNMENT_LINK_TYPE;
use hdk::holochain_core_types::time::{Iso8601, Timeout};
use hdk::prelude::*;
use holochain_wasm_utils::api_serialization::get_entry::GetEntryResultItem;
use std::convert::TryFrom;

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
    let result_history = hdk::get_links_result(
        &agent_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Exactly(role_name),
        GetLinksOptions::default(),
        GetEntryOptions::new(StatusRequestKind::Initial, true, true, Timeout::default()),
    )?;

    let history: Vec<GetEntryResultItem> = result_history
        .into_iter()
        .filter_map(|item| match item {
            Ok(i) => match i.result {
                GetEntryResultType::Single(result) => Some(result),
                _ => None,
            },
            _ => None,
        })
        .collect();

    let maybe_item_index = history.iter().position(|item| {
        let timestamps: Vec<&Iso8601> = item
            .headers
            .iter()
            .map(|header| header.timestamp())
            .collect();

        let min = timestamps.iter().min();
        match min {
            Some(m) => m.clone() > timestamp,
            None => false,
        }
    });

    match maybe_item_index {
        None => is_agent_assigned(history.last()),
        Some(item_index) => {
            let item = history.get(item_index - 1);
            is_agent_assigned(item)
        }
    }
}

fn is_agent_assigned(maybe_item: Option<&GetEntryResultItem>) -> ZomeApiResult<bool> {
    if let Some(item) = maybe_item {
        if let Some(Entry::App(_, entry_content)) = item.clone().entry {
            let role_assignment = RoleAssignment::try_from(entry_content)?;
            return Ok(role_assignment.assigned);
        }
    }

    return Ok(false);
}

/**
 * Returns whether the given agent has been assigned to the given role
 */
pub fn has_agent_role(agent_address: &Address, role_name: &String) -> ZomeApiResult<bool> {
    let role = RoleAssignment::initial(role_name.clone(), agent_address.clone());

    let role_address = role.initial_address()?;

    match hdk::get_entry(&role_address)? {
        Some(Entry::App(_, entry_content)) => {
            let role_assignment = RoleAssignment::try_from(entry_content)?;
            Ok(role_assignment.assigned)
        }
        _ => Ok(false),
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
