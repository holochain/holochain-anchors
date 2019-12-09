use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::{
        entry::Entry,
    },
    holochain_persistence_api::{
        cas::content::{Address, AddressableContent},
    },
};

use serde_derive::{Serialize, Deserialize};

use hdk::prelude::*;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub mod defs;
pub use defs::*;

const ROOT_ANCHOR: &'static str = concat!("holochain_anchors", "::", "root_anchor"); 
const ROOT_ANCHOR_TYPE: &'static str = concat!("holochain_anchors", "::", "RootAnchor"); 
pub const ANCHOR_TYPE: &'static str = concat!("holochain_anchors", "::", "Anchor");
const ROOT_ANCHOR_LINK_TO: &'static str = "anchors";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

type RootAnchor = String;

lazy_static!{
    static ref ROOT_ANCHOR_ADDRESS: RwLock<Option<Address>> = RwLock::new(None);
}

fn root_anchor() -> ZomeApiResult<Address> {
    let root_address = {
        ROOT_ANCHOR_ADDRESS.read().map_err(|e| ZomeApiError::Internal(format!("Lockfailed {:?}", e)))?.as_ref().map(|a|a.clone())
    };

    match root_address {
        None => {
            let mut lock = ROOT_ANCHOR_ADDRESS.write().map_err(|e| ZomeApiError::Internal(format!("Lockfailed {:?}", e)))?;
            // Creating the root anchor
            let root_anchor_entry = Entry::App(
                ROOT_ANCHOR_TYPE.into(),
                ROOT_ANCHOR.into(),
            );
            // Only commit the entry if it doesn't exist from this agents perspective.
            let root_anchor_entry_address = root_anchor_entry.address();
            // get_entry is a slow network call.
            if let Ok(None) = hdk::get_entry(&root_anchor_entry_address) {
                let address;
                address = hdk::commit_entry(&root_anchor_entry)?;
                *lock = Some(address.clone());
                Ok(address)
            } else {
                Ok(root_anchor_entry_address)
            }
        },
        Some(address) => Ok(address),
    }

}

/// Add an anchor with a type.
/// If the anchor already exists then it will use the existing anchor.
pub fn create_anchor(anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
    // Create the anchor entry
    let anchor_entry = Entry::App(
        ANCHOR_TYPE.into(),
        Anchor {
            anchor_type,
            anchor_text
        }.into()
    );
    let anchor_address = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&(root_anchor()?), &anchor_address, ROOT_ANCHOR_LINK_TO, "")?;
    Ok(anchor_address)
}

/// Gives a list of all anchors.
pub fn get_anchors() -> ZomeApiResult<Vec<Address>> {
    let root_anchor_entry_address = root_anchor()?;
    Ok(hdk::get_links(&root_anchor_entry_address, LinkMatch::Exactly(ROOT_ANCHOR_LINK_TO), LinkMatch::Any)?.addresses().to_owned())
}
