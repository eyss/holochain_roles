use hdk::prelude::*;

pub fn get_progenitor_address() -> ZomeApiResult<Address> {
    let progenitor_json = hdk::property("progenitor")?;
    let progenitor: Result<Address, _> = serde_json::from_str(&progenitor_json.to_string());

    match progenitor {
        Ok(progenitor_address) => Ok(progenitor_address),
        Err(_) => Err(ZomeApiError::from(String::from(
            "Could not get the progenitor address",
        ))),
    }
}
