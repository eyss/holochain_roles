use hdk::prelude::*;
use crate::*;

pub fn has_agent_role(agent_address: &Address, role_name: &String) -> ZomeApiResult<bool> {
    let role = Role::from(role_name.clone(), vec![]);

    let role_address = role.address()?;

    let current_role: Role = hdk::utils::get_as_type(role_address.clone())?;

    Ok(current_role.members.contains(&agent_address))
}

pub fn is_agent_admin(agent_address: &Address) -> ZomeApiResult<bool> {
    let progenitor_address = progenitor::get_progenitor_address()?;

    if progenitor_address == agent_address.clone() {
        hdk::debug(format!("HII agent {} is progenitor", agent_address.clone()))?;
        return Ok(true);
    }
    let result = has_agent_role(&agent_address, &String::from(ADMIN_ROLE_NAME))?;

    hdk::debug(format!(
        "HII agent {} is admin? Result : {}",
        agent_address.clone(),
        result.clone()
    ))?;
    Ok(result)
}
