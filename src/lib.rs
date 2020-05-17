use hdk::{
    self,
    error::ZomeApiResult,
    holochain_core_types::entry::Entry,
    holochain_persistence_api::cas::content::{Address, AddressableContent},
};

use serde_derive::{Deserialize, Serialize};

use hdk::prelude::*;

pub mod defs;
pub use defs::*;

const ROOT_ANCHOR: &'static str = concat!("holochain_anchors", "::", "root_anchor");
pub const ANCHOR_TYPE: &'static str = concat!("holochain_anchors", "::", "anchor");
const ANCHOR_LINK_TYPE: &'static str = concat!("holochain_anchors", "::", "anchor_link");

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Anchor {
    pub anchor_type: String,
    pub anchor_text: Option<String>,
}

/// Add an anchor with a type.
/// If the anchor already exists then it will use the existing anchor.
/// If not it will commit it and check to see if it'd Anchor Type and Root anchor need to be committed too.
pub fn anchor(anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
    let anchor_entry = Anchor::new(anchor_type.clone(), Some(anchor_text.clone())).entry();
    let anchor_address = anchor_entry.address();
    if let Ok(None) = hdk::get_entry(&anchor_entry.address()) {
        hdk::commit_entry(&anchor_entry)?;
        let anchor_type_address = check_parent(anchor_type)?;
        hdk::link_entries(
            &anchor_type_address,
            &anchor_address,
            ANCHOR_LINK_TYPE,
            &anchor_text,
        )?;
    }
    Ok(anchor_address)
}

pub fn get_anchor(address: Address) -> ZomeApiResult<Anchor> {
    hdk::utils::get_as_type(address)
}

/// Gives a list of all anchors.
pub fn get_anchors() -> ZomeApiResult<Vec<Address>> {
    let root_anchor_entry_address = root_anchor()?;
    Ok(hdk::get_links(
        &root_anchor_entry_address,
        LinkMatch::Exactly(ANCHOR_LINK_TYPE),
        LinkMatch::Any,
    )?
    .addresses()
    .to_owned())
}

/**
 * Get all anchors of the specified type
 */
pub fn get_anchors_of_type(anchor_type: String) -> ZomeApiResult<Vec<Address>> {
    let anchor_entry = Anchor::new(anchor_type.clone(), None).entry();
    let parent_anchor_address = anchor_entry.address();

    let links = hdk::get_links(
        &parent_anchor_address,
        LinkMatch::Exactly(ANCHOR_LINK_TYPE),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

fn check_parent(anchor_type: String) -> ZomeApiResult<Address> {
    let anchor_type_entry = Anchor::new(anchor_type.clone(), None).entry();
    let anchor_type_address = anchor_type_entry.address();
    if let Ok(None) = hdk::get_entry(&anchor_type_address) {
        hdk::commit_entry(&anchor_type_entry)?;
    }
    check_root(anchor_type_address.clone(), anchor_type)?;
    Ok(anchor_type_address)
}

fn check_root(anchor_type_address: Address, anchor_type: String) -> ZomeApiResult<()> {
    let root_anchor_address = root_anchor()?;
    hdk::link_entries(
        &root_anchor_address,
        &anchor_type_address,
        ANCHOR_LINK_TYPE,
        &anchor_type,
    )?;
    Ok(())
}

fn root_anchor() -> ZomeApiResult<Address> {
    let root_anchor_entry = Anchor::new(ROOT_ANCHOR.into(), None).entry();
    let root_anchor_address = root_anchor_entry.address();
    if let Ok(None) = hdk::get_entry(&root_anchor_address) {
        hdk::commit_entry(&root_anchor_entry)?;
    }
    Ok(root_anchor_address)
}

impl Anchor {
    fn new(anchor_type: String, anchor_text: Option<String>) -> Self {
        Anchor {
            anchor_type,
            anchor_text,
        }
    }
    fn entry(self) -> Entry {
        Entry::App(ANCHOR_TYPE.into(), self.into())
    }
}

/*
philtr -> tweets

let anchor = Anchor {
    "%root",
    ""
};
let anchor = Anchor {
    "handle",
    ""
};
hdk::get_links(&anchor_address, LinkMatch::Exactly("anchor"), LinkMatch::Any);
let anchor = Anchor {
    "handle",
    "philtr"
};
let philtr_address = anchor.address();
hdk::get_links(&philtr_address, LinkMatch::Exactly("favourites"), LinkMatch::Exactly("motorbikes"))
*/
