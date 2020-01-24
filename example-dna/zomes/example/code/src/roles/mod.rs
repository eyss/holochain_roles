use hdk::prelude::*;
use holochain_anchors;

pub mod progenitor;
pub mod validation;

pub const ROLE_TYPE: &'static str = "role";
pub const AGENT_TO_ROLE_LINK_TYPE: &'static str = "agent->role";
pub const ANCHOR_TO_ROLE_LINK_TYPE: &'static str = "anchor->role";
pub const ADMIN_ROLE_NAME: &'static str = "Admin";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Role {
    pub role_name: String,
    pub members: Vec<Address>,
}

impl Role {
    pub fn from(role_name: String, members: Vec<Address>) -> Role {
        Role { role_name, members }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("role".into(), self.into())
    }

    pub fn address(&self) -> ZomeApiResult<Address> {
        let initial_role_entry = Role::from(self.role_name.clone(), vec![]);

        hdk::entry_address(&initial_role_entry.entry())
    }
}

pub fn role_entry_def() -> ValidatingEntryType {
    entry!(
        name: ROLE_TYPE,
        description: "role for ",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Role>| {
            match _validation_data {
                hdk::EntryValidationData::Create { entry, validation_data } => {
                    if entry.members.len() == 0 {
                        return Err(String::from("The first role entry cannot have any members"));
                    }

                    let agent_address = &validation_data.sources()[0];

                    match validation::is_agent_admin(&agent_address)? {
                        true => Ok(()),
                        false => Err(String::from("Only admins can create roles"))
                    }
                },
                hdk::EntryValidationData::Modify { validation_data, .. } => {
                    let agent_address = &validation_data.sources()[0];

                    match validation::is_agent_admin(&agent_address)? {
                        true => Ok(()),
                        false => Err(String::from("Only admins can create roles"))
                    }
                },
                _ => Err(String::from("Cannot delete roles"))
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: AGENT_TO_ROLE_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: ANCHOR_TO_ROLE_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn create_role(role_name: &String) -> ZomeApiResult<Address> {
    let anchor_address = get_role_anchor()?;

    let initial_role_entry = Role::from(role_name.clone(), vec![]);

    let role_address = hdk::commit_entry(&initial_role_entry.entry())?;

    hdk::link_entries(&anchor_address, &role_address, AGENT_TO_ROLE_LINK_TYPE, "")?;

    Ok(role_address)
}

pub fn assign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let mut current_role = get_role(&role_name)?;

    current_role.members.push(agent_address.clone());

    hdk::update_entry(current_role.entry(), &current_role.address()?)?;

    Ok(())
}

pub fn unassign_role(role_name: &String, agent_address: &Address) -> ZomeApiResult<()> {
    let mut current_role = get_role(&role_name)?;

    current_role.members.remove_item(&agent_address);

    hdk::update_entry(current_role.entry(), &current_role.address()?)?;

    Ok(())
}

pub fn get_role(role_name: &String) -> ZomeApiResult<Role> {
    let role = Role::from(role_name.clone(), vec![]);

    let role_address = role.address()?;

    let role_entry: Role = hdk::utils::get_as_type(role_address.clone())?;
    Ok(role_entry)
}

pub fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Vec<Role>> {
    hdk::utils::get_links_and_load_type(
        &agent_address,
        LinkMatch::Exactly(AGENT_TO_ROLE_LINK_TYPE),
        LinkMatch::Any,
    )
}

pub fn get_all_roles() -> ZomeApiResult<Vec<Role>> {
    hdk::utils::get_links_and_load_type(
        &get_role_anchor()?,
        LinkMatch::Exactly(ANCHOR_TO_ROLE_LINK_TYPE),
        LinkMatch::Any,
    )
}

pub fn get_role_anchor() -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor("roles".into(), "all_roles".into())
}
