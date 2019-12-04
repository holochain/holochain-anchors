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

pub mod defs;
pub use defs::*;

const ROOT_ANCHOR: &'static str = concat!("holochain_anchors", "::", "root_anchor"); 
const ROOT_ANCHOR_TYPE: &'static str = concat!("holochain_anchors", "::", "RootAnchor"); 
pub const ANCHOR_TYPE: &'static str = "Anchor";
const ROOT_ANCHOR_LINK_TO: &'static str = "anchors";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct Anchor {
    anchor_type: String,
    anchor_text: String,
}

type RootAnchor = String;

// Will only create a new root anchor if the Agent is not synched or its first hardTimeout
// will accumulate headers for each agent that adds.
// Ideally it would be cool to be able to linkn to the root_anchor_entry.address() and it not be an entry.
fn root_anchor() -> ZomeApiResult<Address> {
    // Creating the root anchor
    let root_anchor_entry = Entry::App(
        ROOT_ANCHOR_TYPE.into(),
        ROOT_ANCHOR.into(),
    );
    // Only commit the entry if it doesn't exist from this agents perspective.
    let root_anchor_entry_address = root_anchor_entry.address();
    if hdk::get_entry(&root_anchor_entry_address)?.is_none() {
        Ok(hdk::commit_entry(&root_anchor_entry)?)
    } else {
        Ok(root_anchor_entry_address)
    }
}

// Add an anchor with a type
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
    // TODO put root_anchor address into lazy static
    hdk::link_entries(&root_anchor()?, &anchor_address, ROOT_ANCHOR_LINK_TO, "")?;
    Ok(anchor_address)
}

pub fn get_anchor(anchor_address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&anchor_address)
}

pub fn get_anchors() -> ZomeApiResult<Vec<Address>> {
    // TODO lazy static
    let root_anchor_entry_address = root_anchor()?;
    Ok(hdk::get_links(&root_anchor_entry_address, LinkMatch::Exactly(ROOT_ANCHOR_LINK_TO), LinkMatch::Any)?.addresses().to_owned())
}