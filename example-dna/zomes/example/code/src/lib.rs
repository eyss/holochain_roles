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

use hc_roles_mixin;
use hdk_proc_macros::zome;

use hc_roles_mixin::Role;

#[zome]
mod my_zome {

    #[init]
    fn init() {
        hc_roles_mixin::handlers::create_admin_role()?;

        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        hc_roles_mixin::role_entry_def()
    }

    #[entry_def]
    fn anchors_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn create_role(role_name: String) -> ZomeApiResult<Address> {
        hc_roles_mixin::handlers::create_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        hc_roles_mixin::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        hc_roles_mixin::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_role(role_name: String) -> ZomeApiResult<Role> {
        hc_roles_mixin::handlers::get_role(&role_name)
    }

    #[zome_fn("hc_public")]
    fn get_all_roles() -> ZomeApiResult<Vec<Role>> {
        hc_roles_mixin::handlers::get_all_roles()
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles() -> ZomeApiResult<Vec<Role>> {
        hc_roles_mixin::handlers::get_all_roles()
    }
}
