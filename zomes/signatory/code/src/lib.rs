#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    signature::Signature,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

use hdk::holochain_wasm_utils::api_serialization::{
        keystore::{
            KeyType,
            KeystoreListResult
        },
};

pub fn handle_sign_message(sign_by_id:String,message:String) -> ZomeApiResult<Signature> {
    hdk::keystore_sign(sign_by_id.to_string(), message.to_string()).map(Signature::from)
}

pub fn handle_create_key(seed_id:String, key_id:String) -> ZomeApiResult<String> {
    let rev_key = hdk::keystore_derive_key(seed_id.to_string(), key_id.to_string(), KeyType::Signing)?;
    hdk::debug(format!("Revocation Key 1 : {:}",rev_key).to_string())?;
    Ok(rev_key)
}

pub fn handle_get_all_keys() -> ZomeApiResult<KeystoreListResult> {
    hdk::keystore_list()
}

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: [
        sign: {
            inputs: | sign_by_id:String, message:String |,
            outputs: |result: ZomeApiResult<Signature>|,
            handler: handle_sign_message
        }
        derive_key: {
            inputs: | seed_id:String, key_id:String |,
            outputs: |result: ZomeApiResult<String>|,
            handler: handle_create_key
        }
        get_all_keys: {
            inputs: | |,
            outputs: |result: ZomeApiResult<KeystoreListResult>|,
            handler: handle_get_all_keys
        }
    ]

    traits: {
        hc_public [sign,derive_key,get_all_keys]
    }
}
