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
        roles::handlers::create_admin_role()?;

        Ok(())
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        roles::role_entry_def()
    }

    #[entry_def]
    fn anchors_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn create_role(role_name: String) -> ZomeApiResult<Address> {

        roles::handlers::create_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        roles::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        roles::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_role(role_name: String) -> ZomeApiResult<Role> {
        roles::handlers::get_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_all_roles() -> ZomeApiResult<Vec<Role>> {
        roles::handlers::get_all_roles()
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles() -> ZomeApiResult<Vec<Role>> {
        roles::handlers::get_all_roles()
    }
}
