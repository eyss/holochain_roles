# hc_roles_mixin

Generic holochain mixin to include administrator and dynamic roles in any holochain application, using the progenitor pattern.

This mixin is built to target `hc v0.0.42-alpha3`. It also depends on the [holochain_anchors](https://github.com/holochain/holochain_anchors) to be present and configured.

> Known issue: due to [updated entries not being propagated](https://github.com/holochain/holochain-rust/issues/2008), this mixin won't actually work in a real environtment, since it depends on an update on the role entry to be propagated througout the network.

## Design

Here is the design for this mixin: https://hackmd.io/6xfwfSVYSGeZe3vQ_-1cWw?view.

## Installation

Add the following to your zomes cargo toml.

```
holochain_anchors = "0.2.1"
hc_roles_mixin = "0.1.1"
```

## Usage

### Setup

Add the anchor entry definition to your zome.

```rust
 #[entry_def]
fn anchor_def() -> ValidatingEntryType {
    holochain_anchors::anchor_definition()
}
```

Add the roles entry definition to your zome.

```rust
 #[entry_def]
fn roles_def() -> ValidatingEntryType {
    hc_roles_mixin::role_entry_def()
}
```

In your `init` function, create the `Admin` role:

```rust
#[init]
fn init() {
    hc_roles_mixin::handlers::create_admin_role()?;
    Ok(())
}
```

### Create a role

To create a role, simply call the `create_role` function:

```rust
#[zome_fn("hc_public")]
fn some_public_function() {
    let my_role_name = String::from("editor");

    hc_roles_mixin::handlers::create_role(&my_role_name)?;
    ...
}
```

### Assign a role

To assign a role, simply call the `assign_role` function:

```rust
#[zome_fn("hc_public")]
fn some_other_public_function(agent_address: Address) {
    let my_role_name = String::from("editor");

    hc_roles_mixin::handlers::assign_role(&my_role_name, &agent_address)?;
    ...
}
```

### Check if user has a certain role

To check if a user has a certain role, you can use the validation `has_agent_role` function:

```rust
validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
    match _validation_data {
        hdk::EntryValidationData::Create { entry, validation_data } => {
            let agent_address = &validation_data.sources()[0];
            let is_agent_permitted_to_create_this_entry = hc_roles_mixin::validaton::has_agent_role(&agent_address, String::from("editor"))?;

            if !is_agent_permitted_to_create_this_entry {
                return Err(String::from("Only editors can create a new entry"));
            }
            ...
            

```

### Unassign a role

To unassign a role, simply call the `unassign_role` function:

```rust
#[zome_fn("hc_public")]
fn some_other_public_function(agent_address: Address) {
    let my_role_name = String::from("editor");

    hc_roles_mixin::handlers::unassign_role(&my_role_name, &agent_address)?;
    ...
}
```