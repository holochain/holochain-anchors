use hdk::{
    entry_definition::ValidatingEntryType,
    entry,
    to,
    link,
};
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
};

use crate::{
    ROOT_ANCHOR_TYPE,
    ROOT_ANCHOR_LINK_TO,
    ANCHOR_TYPE,
    Anchor,
    RootAnchor,
};

/// This defines the root anchor that is used to list all anchors.
/// It must be called from your zome.
pub fn root_anchor_definition() -> ValidatingEntryType {
    entry!(
        name: ROOT_ANCHOR_TYPE,
        description: "All other anchors are linked from the root anchor so we can list all the anchors.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<RootAnchor>| {
            Ok(())
        },
        links: [
            to!(
                ANCHOR_TYPE,
                link_type: ROOT_ANCHOR_LINK_TO,

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

/// This defines the anchor type that is used to create arbitrary anchors.
/// It must be called from your zome.
pub fn anchor_definition() -> ValidatingEntryType {
    entry!(
        name: ANCHOR_TYPE,
        description: "Anchors are used as the base for links so linked entries can be found with a text search.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Anchor>| {
            Ok(())
        }
    )
}
