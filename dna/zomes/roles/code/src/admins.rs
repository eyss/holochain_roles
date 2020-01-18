use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::Address,
};

use crate::{utils, validation, ADMINISTRATOR_ROLE};

pub fn is_agent_admin(chain_entries: &Vec<Entry>) -> ZomeApiResult<bool> {
    let agent_address = utils::get_chain_agent_id(&chain_entries)?;
    match is_agent_initial_admin(&agent_address)? {
        true => Ok(true),
        false => validation::agent_has_role(String::from(ADMINISTRATOR_ROLE), chain_entries),
    }
}

pub fn get_initial_admins() -> ZomeApiResult<Vec<Address>> {
    let initial_admins_json = hdk::property("initial_admins")?;
    let initial_admins: Result<Vec<Address>, _> =
        serde_json::from_str(&initial_admins_json.to_string());

    match initial_admins {
        Ok(admins) => Ok(admins),
        Err(_) => Err(ZomeApiError::from(String::from(
            "could not get initial admins list",
        ))),
    }
}

pub fn is_agent_initial_admin(agent_address: &Address) -> ZomeApiResult<bool> {
    let initial_admins = get_initial_admins()?;

    Ok(initial_admins.contains(agent_address))
}
