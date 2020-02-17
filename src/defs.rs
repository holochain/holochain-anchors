use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::{entry, entry_definition::ValidatingEntryType, from, link};

use crate::{Anchor, ANCHOR_LINK_TYPE, ANCHOR_TYPE};

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
        },
        links: [
            from!(
                ANCHOR_TYPE,
                link_type: ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                },
            to!(
                ANCHOR_ANCHOR_TYPE,
                link_type: ANCHOR_ANCHOR_LINK_TYPE,
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
