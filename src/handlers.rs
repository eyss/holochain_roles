use crate::ROLE_TO_ASSIGNMENT_LINK_TYPE;
use crate::AGENT_TO_ASSIGNMENT_LINK_TYPE;
use hdk::prelude::*;

use crate::RoleAssignment;

/**
 * Assigns the role with the given name to the given agent
 * Only administrators can assign roles
 */
pub fn assign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let role_address = get_role_anchor_address(&role_name)?;

    let assignment = RoleAssignment::from(role_name.clone(), agent_address.clone());

    let assignment_address = hdk::commit_entry(&assignment.entry())?;

    hdk::link_entries(
        &agent_address,
        &assignment_address,
        AGENT_TO_ASSIGNMENT_LINK_TYPE,
        "",
    )?;

    hdk::link_entries(
        &role_address,
        &assignment_address,
        ROLE_TO_ASSIGNMENT_LINK_TYPE,
        "",
    )?;

    Ok(())
}

/**
 * Unassigns the role with the given name to the given agent
 * Only administrators can unassign roles
 */
pub fn unassign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let role_anchor = get_role_anchor_address(&role_name)?;

    let assignment = RoleAssignment::from(role_name.clone(), agent_address.clone());
    let assignment_address = assignment.address()?;

    hdk::remove_link(
        &agent_address,
        &assignment_address,
        AGENT_TO_ASSIGNMENT_LINK_TYPE,
        "",
    )?;
    hdk::remove_link(
        &role_anchor,
        &assignment_address,
        ROLE_TO_ASSIGNMENT_LINK_TYPE,
        "",
    )?;
    hdk::remove_entry(&assignment_address)?;

    Ok(())
}

/**
 * Returns all the roles that the given agent has been assigned to
 */
pub fn get_agent_roles(agent_address: &Address) -> ZomeApiResult<Vec<RoleAssignment>> {
    hdk::utils::get_links_and_load_type(
        agent_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Any,
    )
}

/**
 * Returns all the roles that the given agent has been assigned to
 */
pub fn get_role_agents(role_name: &String) -> ZomeApiResult<Vec<Address>> {
    let role_address = get_role_anchor_address(&role_name)?;

    let assignment: Vec<RoleAssignment> = hdk::utils::get_links_and_load_type(
        &role_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Any,
    )?;

    Ok(assignment
        .iter()
        .map(|assignment| assignment.agent_address.clone())
        .collect())
}

/**
* Returns all the roles present in the application
pub fn get_all_roles() -> ZomeApiResult<Vec<String>> {
    let roles = hdk::utils::get_links_and_load_type(
        &get_role_root_anchor()?,
        LinkMatch::Exactly(holochain_anchors::ANCHOR_TYPE),
        LinkMatch::Any,
       )?;

       roles.iter().map(|role| role.)
   }
   */

fn get_role_root_anchor() -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor("roles".into(), "all_roles".into())
}


/**
 * Returns the role anchor address for the role with the given name
 */
fn get_role_anchor_address(role_name: &String) -> ZomeApiResult<Address> {
    let role_anchor = holochain_anchors::create_anchor("role".into(), role_name.into())?;

    let root_anchor = get_role_root_anchor()?;

    hdk::link_entries(
        &root_anchor,
        &role_anchor,
        holochain_anchors::ANCHOR_TYPE,
        "",
    )?;

    Ok(role_anchor)
}
