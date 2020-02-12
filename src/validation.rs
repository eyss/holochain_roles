use crate::ADMIN_ROLE_NAME;
use crate::RoleAssignment;
use crate::progenitor;
use hdk::prelude::*;

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
