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

pub mod roles;

use roles::Role;

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
        roles::role_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_role(role_name: String) -> ZomeApiResult<Address> {
        roles::create_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        roles::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        roles::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_role(role_name: String) -> ZomeApiResult<Role> {
        roles::get_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_all_roles() -> ZomeApiResult<Vec<Role>> {
        roles::get_all_roles()
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles() -> ZomeApiResult<Vec<Role>> {
        roles::get_all_roles()
    }
}
