use hdk::prelude::*;

use crate::{Role, ADMIN_ROLE_NAME, AGENT_TO_ROLE_LINK_TYPE, ANCHOR_TO_ROLE_LINK_TYPE};

/**
 * Creates the administrator role in the hApp
 * This function should be called 
 * Anyone can create the administrator role
 */
pub fn create_admin_role() -> ZomeApiResult<()> {
    let admin_role = Role::from(String::from(ADMIN_ROLE_NAME), vec![]);

    match hdk::get_entry(&admin_role.address()?)? {
        Some(_) => Ok(()),
        None => {
            create_role(&String::from(ADMIN_ROLE_NAME))?;

            Ok(())
        }
    }
}

/**
 * Creates the role with the given name
 * Only administrators can create roles
 */
pub fn create_role(role_name: &String) -> ZomeApiResult<Address> {
    let anchor_address = get_role_anchor()?;

    let initial_role_entry = Role::from(role_name.clone(), vec![]);

    let role_address = hdk::commit_entry(&initial_role_entry.entry())?;

    hdk::link_entries(&anchor_address, &role_address, ANCHOR_TO_ROLE_LINK_TYPE, "")?;

    Ok(role_address)
}

/**
 * Assigns the role with the given name to the given agent
 * Only administrators can assign roles
 */
pub fn assign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let mut current_role = get_role(&role_name)?;

    current_role.members.push(agent_address.clone());

    hdk::update_entry(current_role.entry(), &current_role.address()?)?;

    Ok(())
}

/**
 * Unassigns the role with the given name to the given agent
 * Only administrators can unassign roles
 */
pub fn unassign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let mut current_role = get_role(&role_name)?;

    current_role.members.remove_item(&agent_address);

    hdk::update_entry(current_role.entry(), &current_role.address()?)?;

    Ok(())
}

/**
 * Returns the current role entry for the role with the given name
 * This can be used to check all the current members that have been assigned to this role
 */
pub fn get_role(role_name: &String) -> ZomeApiResult<Role> {
    let role = Role::from(role_name.clone(), vec![]);

    let role_address = role.address()?;

    let role_entry: Role = hdk::utils::get_as_type(role_address.clone())?;
    Ok(role_entry)
}


/**
 * Returns all the roles that the given agent has been assigned to
 */
pub fn get_agent_roles(agent_address: &Address) -> ZomeApiResult<Vec<Role>> {
    hdk::utils::get_links_and_load_type(
        agent_address,
        LinkMatch::Exactly(AGENT_TO_ROLE_LINK_TYPE),
        LinkMatch::Any,
    )
}

/**
 * Returns all the roles present in the application
 */
pub fn get_all_roles() -> ZomeApiResult<Vec<Role>> {
    hdk::utils::get_links_and_load_type(
        &get_role_anchor()?,
        LinkMatch::Exactly(ANCHOR_TO_ROLE_LINK_TYPE),
        LinkMatch::Any,
    )
}

fn get_role_anchor() -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor("roles".into(), "all_roles".into())
}
