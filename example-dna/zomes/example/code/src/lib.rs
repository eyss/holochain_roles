#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate holochain_json_derive;
extern crate serde_json;

use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;
use holochain_roles;

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        holochain_roles::role_entry_def()
    }

    #[entry_def]
    fn anchors_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_role_agents(role_name: String) -> ZomeApiResult<Role> {
        holochain_roles::handlers::get_role_agents(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Vec<String>> {
        hc_roles_mixin::handlers::get_agent_roles(&agent_address)
    }
}
