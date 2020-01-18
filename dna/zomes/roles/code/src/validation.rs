use hdk::prelude::*;

use crate::assignment::Assignment;
use crate::role::Role;
use crate::utils;
use crate::ASSIGNMENT_TYPE;

pub fn agent_has_role(role_name: String, chain_entries: &Vec<Entry>) -> ZomeApiResult<bool> {
    let agent_address = utils::get_chain_agent_id(chain_entries)?;

    let role = Role::from(role_name);
    let role_address = role.address()?;

    let assignment: Option<Assignment> =
        utils::find_entry(chain_entries, ASSIGNMENT_TYPE, |assignment: Assignment| {
            Ok(
                assignment.role_address == role_address
                    && assignment.agent_address == agent_address,
            )
        });

    match assignment {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}
