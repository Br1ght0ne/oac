#![warn(missing_docs)]

//! A parser for the documents conforming to OAC [spec].
//!
//! TODO: finish docs

/// Types for convenient error handling.
pub mod error;
use error::{Error, Result};

/// Implementation of the
/// [OpenAutoComplete spec](https://github.com/openautocomplete/openautocomplete/blob/master/SPECIFICATION.md).
///
/// The root for a JSON file conforming to the spec is [Document].
pub mod spec;
pub use spec::{Document, VERSION};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

/// Crude implementation of `$ref` syntax used in JSON Schema.
#[derive(Debug, Deserialize, Serialize)]
pub struct Reference<'a, T> {
    /// A reference path, e.g. `#/components/commands/foo`.
    #[serde(rename = "$ref")]
    pub r#ref: String,
    #[serde(skip)]
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Reference<'a, T>
where
    T: DeserializeOwned,
{
    fn parts(&self) -> Vec<&str> {
        self.r#ref.split('/').skip(1).collect()
    }

    /// Resolves the reference relative to the `document`.
    pub fn resolve(&self, document: &Document) -> Result<T> {
        use serde_json::Value::*;

        let mut json = serde_json::to_value(&document).expect("failed to serialize document");
        for part in self.parts() {
            json = json[part].clone();
        }
        match json {
            Null => Err(Error::MissingReferenceTarget {
                reference: self.r#ref.clone(),
            }),
            value => Ok(serde_json::from_value::<T>(value).unwrap()),
        }
    }
}
