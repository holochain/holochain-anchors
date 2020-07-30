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

/// Gives a list of all addresses from root anchor
pub fn list_anchor_type_addresses() -> ZomeApiResult<Vec<Address>> {
    let root_anchor_address = root_anchor()?;
    Ok(hdk::get_links(
        &root_anchor_address,
        LinkMatch::Exactly(ANCHOR_LINK_TYPE),
        LinkMatch::Any,
    )?
    .addresses()
    .to_owned())
}

/// Gives a list of all link tags from root anchor (same as the anchor_text value)
pub fn list_anchor_type_tags() -> ZomeApiResult<Vec<String>> {
    let root_anchor_address = root_anchor()?;
    Ok(hdk::get_links(&root_anchor_address, LinkMatch::Exactly(ANCHOR_LINK_TYPE), LinkMatch::Any)?.links()
    .iter()
    .map(|link| link.tag.clone())
    .collect())
}

/// Gives a list of all addresses from an anchor type
pub fn list_anchor_addresses(anchor_type: String) -> ZomeApiResult<Vec<Address>> {
    let anchor_type_entry = Anchor::new(anchor_type.clone(), None).entry();
    let anchor_type_address = anchor_type_entry.address();
    Ok(hdk::get_links(
        &anchor_type_address,
        LinkMatch::Exactly(ANCHOR_LINK_TYPE),
        LinkMatch::Any,
    )?
    .addresses()
    .to_owned())
}

/// Gives a list of all link tags from an anchor type (same as the anchor_text value)
pub fn list_anchor_tags(anchor_type: String) -> ZomeApiResult<Vec<String>> {
    let anchor_type_entry = Anchor::new(anchor_type.clone(), None).entry();
    let anchor_type_address = anchor_type_entry.address();
    Ok(hdk::get_links(&anchor_type_address, LinkMatch::Exactly(ANCHOR_LINK_TYPE), LinkMatch::Any)?.links()
    .iter()
    .map(|link| link.tag.clone())
    .collect())
}

pub fn link_anchors(anchor_from_type: String, anchor_from_text: String, anchor_type: String, anchor_text: String) -> ZomeApiResult<Address> {
    hdk::link_entries(&anchor(anchor_from_type, anchor_from_text.clone())?, &anchor(anchor_type, anchor_text)?, ANCHOR_LINK_TYPE, &anchor_from_text)
}

pub fn bi_directional_link_anchors(anchor_1_type: String, anchor_1_text: String, anchor_2_type: String, anchor_2_text: String) -> ZomeApiResult<Address> {
    hdk::link_entries(&anchor(anchor_1_type.clone(), anchor_1_text.clone())?, &anchor(anchor_2_type.clone(), anchor_2_text.clone())?, ANCHOR_LINK_TYPE, &anchor_1_text.clone())?;
    hdk::link_entries(&anchor(anchor_2_type.clone(), anchor_2_text.clone())?, &anchor(anchor_1_type.clone(), anchor_1_text.clone())?, ANCHOR_LINK_TYPE, &anchor_2_text.clone())
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
