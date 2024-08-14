// SPDX-FileCopyrightText: 201/fzf.vim9-2021 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

//! Search index for a documentation source.
//!
//! The search index is read from the `search-index.js` file generated by rustdoc.  It contains a
//! list of items grouped by their crate.
//!
//! For details on the format of the search index, see the `html/render/mod.rs` (previously
//! `html/render.rs`) file in `librustdoc`.  Note that the format of the search index changed in
//! April 2020 (Rust 1.44.0) with commit b4fb3069ce82f61f84a9487d17fb96389d55126a.  We only support
//! the new format as the old format is much harder to parse.
//!
//! For details on the generation of the search index, see the `html/render/cache.rs` file in
//! `librustdoc`.

mod v1_44;
mod v1_52;
mod v1_69;

use std::collections;
use std::fmt;
use std::fs;
use std::io;
use std::path;

use crate::doc;

#[derive(Debug)]
pub struct Index {
    path: path::PathBuf,
    data: Data,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IndexItem {
    pub name: doc::Fqn,
    pub ty: doc::ItemType,
    pub description: String,
}

impl fmt::Display for IndexItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.description.is_empty() {
            write!(f, "{} ({})", &self.name, self.ty.name())
        } else {
            write!(
                f,
                "{} ({}): {}",
                &self.name,
                self.ty.name(),
                &self.description
            )
        }
    }
}

#[derive(Debug, Default, PartialEq, serde::Deserialize)]
#[serde(transparent)]
struct Data {
    crates: collections::HashMap<String, CrateData>,
}

#[derive(Debug, Default, PartialEq)]
struct CrateData {
    items: Vec<ItemData>,
    paths: Vec<(usize, String)>,
}

impl<'de> serde::Deserialize<'de> for CrateData {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CrateDataVersions::deserialize(deserializer).map(From::from)
    }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
enum CrateDataVersions {
    V1_44(v1_44::CrateData),
    V1_52(v1_52::CrateData),
    V1_69(v1_69::CrateData),
}

impl From<CrateDataVersions> for CrateData {
    fn from(versions: CrateDataVersions) -> Self {
        match versions {
            CrateDataVersions::V1_44(data) => data.into(),
            CrateDataVersions::V1_52(data) => data.into(),
            CrateDataVersions::V1_69(data) => data.into(),
        }
    }
}

#[derive(Debug, PartialEq, serde_tuple::Deserialize_tuple)]
struct ItemData {
    ty: ItemType,
    name: String,
    path: String,
    desc: String,
    parent: Option<usize>,
    _ignored: serde_json::Value,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ItemType(doc::ItemType);

impl From<doc::ItemType> for ItemType {
    fn from(ty: doc::ItemType) -> Self {
        Self(ty)
    }
}

impl From<ItemType> for doc::ItemType {
    fn from(ty: ItemType) -> Self {
        ty.0
    }
}

impl<'de> serde::Deserialize<'de> for ItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use core::convert::TryInto;
        use serde::de::Error;

        match u8::deserialize(deserializer)?.try_into() {
            Ok(item) => Ok(Self(item)),
            _ => Err(D::Error::custom("Unexpected item type")),
        }
    }
}

impl Index {
    pub fn load(path: impl AsRef<path::Path>) -> anyhow::Result<Option<Self>> {
        use std::io::BufRead;

        anyhow::ensure!(
            path.as_ref().is_file(),
            "Search index '{}' must be a file",
            path.as_ref().display()
        );

        let mut json: Option<String> = None;
        let mut finished = false;

        for line in io::BufReader::new(fs::File::open(path.as_ref())?).lines() {
            let line = line?;
            if let Some(json) = &mut json {
                if line == "}');" {
                    json.push('}');
                    finished = true;
                    break;
                } else {
                    json.push_str(line.trim_end_matches('\\'));
                }
            } else if line == "var searchIndex = JSON.parse('{\\" {
                json = Some(String::from("{"));
            }
        }

        if let Some(json) = json {
            if finished {
                use anyhow::Context;
                let json = json.replace("\\'", "'");
                let data: Data = serde_json::from_str(&json)
                    .context(format!("Could not parse search index of {}", &json))?;

                Ok(Some(Index {
                    data,
                    path: path.as_ref().to_owned(),
                }))
            } else {
                log::info!(
                    "Did not find JSON end line in search index '{}'",
                    path.as_ref().display()
                );
                Ok(None)
            }
        } else {
            log::info!(
                "Did not find JSON start line in search index '{}'",
                path.as_ref().display()
            );
            Ok(None)
        }
    }

    pub fn find(&self, name: &doc::Name) -> Vec<IndexItem> {
        log::info!(
            "Looking up '{}' in search index '{}'",
            name,
            self.path.display()
        );
        let mut matches: Vec<IndexItem> = Vec::new();
        for (krate, data) in &self.data.crates {
            let mut path = krate;
            for item in &data.items {
                path = if item.path.is_empty() {
                    path
                } else {
                    &item.path
                };

                let ty = doc::ItemType::from(item.ty);
                if ty == doc::ItemType::AssocType {
                    continue;
                }

                let full_path = match item.parent {
                    Some(idx) => {
                        let parent = &data.paths[idx].1;
                        format!("{}::{}", path, parent)
                    }
                    None => path.to_owned(),
                };
                let full_name: doc::Fqn = format!("{}::{}", &full_path, &item.name).into();
                if full_name.ends_with(name) {
                    log::info!("Found index match '{}'", full_name);
                    matches.push(IndexItem {
                        name: full_name,
                        ty,
                        description: item.desc.clone(),
                    });
                }
            }
        }
        matches.sort_unstable();
        matches.dedup();
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::{CrateData, Data, Index, IndexItem, ItemData};
    use crate::doc::ItemType;
    use crate::test_utils::{with_rustdoc, Format};

    #[test]
    fn test_empty() {
        let expected: Data = Default::default();
        let actual: Data = serde_json::from_str("{}").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_empty_crate() {
        let mut expected: Data = Default::default();
        expected
            .crates
            .insert("test".to_owned(), Default::default());
        let actual: Data = serde_json::from_str("{\"test\": {\"i\": [], \"p\": []}}").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_one_item() {
        let mut expected: Data = Default::default();
        let mut krate: CrateData = Default::default();
        krate.items.push(ItemData {
            ty: ItemType::Module.into(),
            name: "name".to_owned(),
            path: "path".to_owned(),
            desc: "desc".to_owned(),
            parent: None,
            _ignored: Default::default(),
        });
        expected.crates.insert("test".to_owned(), krate);
        let actual: Data = serde_json::from_str(
            "{\"test\": {\"i\": [[0, \"name\", \"path\", \"desc\", null, null]], \"p\": []}}",
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index() {
        with_rustdoc(">=1.44.0, <1.50.0", Format::all(), |_, _, path| {
            let index = Index::load(path.join("search-index.js")).unwrap().unwrap();

            let empty: Vec<IndexItem> = Vec::new();

            let node_data_ref = vec![IndexItem {
                name: "kuchiki::NodeDataRef".to_owned().into(),
                ty: ItemType::Struct,
                description: "Holds a strong reference to a node, but dereferences to…".to_owned(),
            }];
            assert_eq!(node_data_ref, index.find(&"NodeDataRef".to_owned().into()));
            assert_eq!(
                node_data_ref,
                index.find(&"kuchiki::NodeDataRef".to_owned().into())
            );
            assert_eq!(empty, index.find(&"DataRef".to_owned().into()));
            assert_eq!(empty, index.find(&"NodeDataReff".to_owned().into()));
        });

        with_rustdoc(">=1.50.0", Format::all(), |_, _, path| {
            let index = Index::load(path.join("search-index.js")).unwrap().unwrap();

            let empty: Vec<IndexItem> = Vec::new();

            let node_data_ref = vec![IndexItem {
                name: "kuchiki::NodeDataRef".to_owned().into(),
                ty: ItemType::Struct,
                description: "Holds a strong reference to a node, but dereferences to …".to_owned(),
            }];
            assert_eq!(node_data_ref, index.find(&"NodeDataRef".to_owned().into()));
            assert_eq!(
                node_data_ref,
                index.find(&"kuchiki::NodeDataRef".to_owned().into())
            );
            assert_eq!(empty, index.find(&"DataRef".to_owned().into()));
            assert_eq!(empty, index.find(&"NodeDataReff".to_owned().into()));
        });

        with_rustdoc(">=1.44.0", Format::all(), |_, _, path| {
            let index = Index::load(path.join("search-index.js")).unwrap().unwrap();

            let empty: Vec<IndexItem> = Vec::new();

            let as_node = vec![IndexItem {
                name: "kuchiki::NodeDataRef::as_node".to_owned().into(),
                ty: ItemType::Method,
                description: "Access the corresponding node.".to_owned(),
            }];
            assert_eq!(as_node, index.find(&"as_node".to_owned().into()));
            assert_eq!(
                as_node,
                index.find(&"NodeDataRef::as_node".to_owned().into())
            );
            assert_eq!(
                as_node,
                index.find(&"kuchiki::NodeDataRef::as_node".to_owned().into())
            );
            assert_eq!(empty, index.find(&"DataRef::as_node".to_owned().into()));
        });
    }
}
