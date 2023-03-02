use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MetadataType {
    DocumentType,
    CollectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    deleted: bool,
    lastModified: String,
    lastOpened: Option<String>,
    lastOpenedPage: Option<u32>,
    metadatamodified: bool,
    modified: bool,
    parent: String,
    pinned: bool,
    synced: bool,
    #[serde(rename(deserialize = "type"))]
    _type: MetadataType,
    version: u16,
    visibleName: String,
}

pub struct File {}

pub struct Folder {
    folders: Vec<Folder>,
    files: Vec<File>,
}
