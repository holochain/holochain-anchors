# Holochain Anchors
This create allows a Holochain project to easily use the anchors pattern for creating links.
This is still a work in progress and feedback would be appreciated.

## Install
Add the following to your zomes cargo toml.
```
holochain_anchors = "0.2.3"
```

## Usage
Add the anchor entry def to your zome.
```rust
 #[entry_def]
fn anchor_def() -> ValidatingEntryType {
    holochain_anchors::anchor_definition()
}
```
Link from the `ANCHOR_TYPE`
```rust
links: [
    from!(
        holochain_anchors::ANCHOR_TYPE,
        link_type: "my_link_type",
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::LinkValidationData| {
            Ok(())
        }
    )
]
```
Create an anchor and link an entry to it.
If the anchor already exists then it will use the existing anchor.
```rust
let my_entry = Entry::App(
    "my_entry".into(),
        MyEntry{
        content: "some_content".into()
    }.into()
);
let address = hdk::commit_entry(&my_entry)?;
let anchor_address = holochain_anchors::anchor("my_anchor_type".into(), "my_anchor".into())?;
hdk::link_entries(&anchor_address, &address, "my_link_type", "my_anchor")?;
```
Get all the links on that anchor.
```rust
let anchor_address = holochain_anchors::anchor("my_anchor_type".into(), "my_anchor".into())?;
hdk::utils::get_links_and_load_type(&anchor_address, LinkMatch::Exactly("my_link_type"), LinkMatch::Any)
```
