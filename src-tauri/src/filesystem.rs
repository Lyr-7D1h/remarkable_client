use std::path::{Path, PathBuf};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct File {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    folders: Vec<Folder>,
    files: Vec<File>,
}

pub struct Filesystem {
    path: PathBuf,
    root: Folder,
}

impl Filesystem {
    pub fn load(path: &Path) -> Filesystem {
        // read_dir(path)
        //     .unwrap()
        //     .into_iter()
        //     .filter_map(|entry| {
        //         let entry = entry.unwrap();
        //         if let Some(file_stem) = entry.path().file_stem() {
        //             if visited.contains(&file_stem.to_owned()) {
        //                 return None;
        //             }
        //         }
        //         if let Some(ext) = entry.path().extension() {
        //             if ext == "metadata" {
        //                 if let Ok(_metadata) =
        //                     serde_json::from_str::<Metadata>(&read_to_string(entry.path()).unwrap())
        //                 {
        //                     let path = entry.path();
        //                     let file_stem = path
        //                         .file_stem()
        //                         .expect(&format!("Could not find filename for {:?}", entry.path()));
        //                     visited.insert(file_stem.to_owned());
        //                 }
        //             }
        //         }

        //         Some(entry)
        //     })
        //     .filter_map(|entry| {
        //         let file_type = entry.file_type().unwrap();
        //         if file_type.is_dir() {
        //             Some(
        //                 button(text(format!(
        //                     "{}",
        //                     entry.file_name().to_string_lossy().to_string()
        //                 )))
        //                 .width(300)
        //                 .on_press(Message::Open(entry.path())),
        //             )
        //         } else {
        //             None
        //         }
        //     })
        //     .into()
        Filesystem {
            path: path.to_owned(),
            root: Folder {
                folders: vec![],
                files: vec![],
            },
        }
    }
}
