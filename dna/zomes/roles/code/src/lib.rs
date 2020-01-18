#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

pub mod role;
pub mod assignment;

pub const ROLE_TYPE: &str = "role";
pub const ASSIGNMENT_TYPE: &str = "role_assignment";
pub const ROLE_ASSIGNMENT_LINK_TYPE: &str = "role->role_assignment";
pub const AGENT_ASSIGNMENT_LINK_TYPE: &str = "agent->role_assignment";

pub mod assignment;
pub mod role;

#[zome]
pub mod roles_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn role_entry_def() {
        role::role_entry_definition()
    }

    #[entry_def]
    fn assignment_entry_def() {
        assignment::assignment_entry_definition()
    }

}
