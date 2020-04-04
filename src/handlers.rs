use crate::{
    progenitor, RoleAssignment, AGENT_TO_ASSIGNMENT_LINK_TYPE, ROLE_TO_ASSIGNMENT_LINK_TYPE,
};
use hdk::prelude::*;

/**
 * Assigns the role with the given name to the given agent
 * Only administrators can assign roles
 */
pub fn assign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    update_assignment_entry(&role_name, &agent_address, true)
}

/**
 * Unassigns the role with the given name to the given agent
 * Only administrators can unassign roles
 */
pub fn unassign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    update_assignment_entry(&role_name, &agent_address, false)
}

fn update_assignment_entry(
    role_name: &String,
    agent_address: &Address,
    assigned: bool,
) -> ZomeApiResult<()> {
    let role_address = get_role_anchor_address(&role_name)?;

    let links_result = hdk::get_links(
        &agent_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Exactly(String::from(role_name.clone()).as_str()),
    )?;

    let maybe_previous_address: Option<Address> =
        links_result.addresses().get(0).map(|a| a.clone());

    let new_assignment_address: Address = {
        if let Some(previous_address) = maybe_previous_address {
            let mut previous_assignment: RoleAssignment =
                hdk::utils::get_as_type(previous_address.clone())?;

            // If assigned has not changed, we don't have to do anything, return
            if previous_assignment.assigned == assigned {
                return Ok(());
            } else {
                previous_assignment.previous_assignment_address = Some(previous_address.clone());
                previous_assignment.assigned = assigned;
                
                // TODO: comment this when update_entry works
                hdk::remove_link(
                    &agent_address,
                    &previous_address,
                    AGENT_TO_ASSIGNMENT_LINK_TYPE,
                    role_name.as_str(),
                )?;
                hdk::remove_link(
                    &role_address,
                    &previous_address,
                    ROLE_TO_ASSIGNMENT_LINK_TYPE,
                    String::from(agent_address.clone()).as_str(),
                )?;
                 
                // TODO: uncomment this when update_entry works hdk::update_entry(previous_assignment.entry(), &previous_address)?
                hdk::commit_entry(&previous_assignment.entry())?
            }
        } else if !assigned {
            return Ok(());
        } else {
            let initial_assignment =
                RoleAssignment::initial(role_name.clone(), agent_address.clone());
            hdk::commit_entry(&initial_assignment.entry())?
        }
    };

    hdk::link_entries(
        &agent_address,
        &new_assignment_address,
        AGENT_TO_ASSIGNMENT_LINK_TYPE,
        role_name.as_str(),
    )?;
    hdk::link_entries(
        &role_address,
        &new_assignment_address,
        ROLE_TO_ASSIGNMENT_LINK_TYPE,
        String::from(agent_address.clone()).as_str(),
    )?;

    Ok(())
}

/**
 * Returns all the roles that the given agent has been assigned to
 */
pub fn get_agent_roles(agent_address: &Address) -> ZomeApiResult<Vec<String>> {
    let assignments: Vec<RoleAssignment> = hdk::utils::get_links_and_load_type(
        agent_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Any,
    )?;

    let mut role_names: Vec<String> = assignments
        .iter()
        .filter_map(|a| match a.assigned {
            true => Some(a.role_name.clone()),
            false => None,
        })
        .collect();

    if progenitor::get_progenitor_address()? == agent_address.clone() {
        role_names.push(String::from(crate::ADMIN_ROLE_NAME));
    }

    Ok(role_names)
}

/**
 * Returns all the roles that the given agent has been assigned to
 */
pub fn get_agents_with_role(role_name: &String) -> ZomeApiResult<Vec<Address>> {
    let role_address = get_role_anchor_address(&role_name)?;

    let assignment: Vec<RoleAssignment> = hdk::utils::get_links_and_load_type(
        &role_address,
        LinkMatch::Exactly(AGENT_TO_ASSIGNMENT_LINK_TYPE),
        LinkMatch::Any,
    )?;

    Ok(assignment
        .iter()
        .filter_map(|assignment| match assignment.assigned {
            true => Some(assignment.agent_address.clone()),
            false => None,
        })
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
    holochain_anchors::anchor("roles".into(), "all_roles".into())
}

/**
 * Returns the role anchor address for the role with the given name
 */
fn get_role_anchor_address(role_name: &String) -> ZomeApiResult<Address> {
    let role_anchor = holochain_anchors::anchor("role".into(), role_name.into())?;

    let root_anchor = get_role_root_anchor()?;

    hdk::link_entries(&root_anchor, &role_anchor, crate::ANCHOR_LINK_TYPE, "")?;

    Ok(role_anchor)
}
