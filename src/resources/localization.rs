//! Localization asset

use crate::{
    exts::fluent::{content::Request, BundleExt, Content},
    BundleAsset,
};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use fluent::FluentArgs;
use indexmap::IndexMap;
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Formatter},
};
use unic_langid::LanguageIdentifier;

/// Localization
///
/// Collection of [`BundleAsset`]s.
#[derive(Default)]
pub struct Localization(IndexMap<Handle<BundleAsset>, BundleAsset>);

impl Localization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handles(&self) -> impl Iterator<Item = &Handle<BundleAsset>> {
        self.0.keys()
    }

    pub fn insert(&mut self, handle: &Handle<BundleAsset>, asset: &BundleAsset) {
        self.0.insert(handle.clone(), asset.clone());
    }

    pub fn locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.0.values().map(|bundle| bundle.locale())
    }
}

impl<'a, T, U> Content<'a, T, U> for Localization
where
    T: Copy + Into<Request<'a, U>>,
    U: Borrow<FluentArgs<'a>>,
{
    #[instrument(fields(request = %request.into()), skip_all)]
    fn content(&self, request: T) -> Option<String> {
        self.0.values().find_map(|bundle| {
            let content = bundle.content(request);
            trace!(locale = %bundle.locale(), ?content);
            content
        })
    }
}

impl Debug for Localization {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Localization")
            .field(&self.locales().collect::<Vec<_>>())
            .finish()
    }
}
