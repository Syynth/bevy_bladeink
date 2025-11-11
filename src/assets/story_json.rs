use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use thiserror::Error;

/// `StoryJson` is a struct that represents a JSON file containing a story.
#[derive(Asset, TypePath)]
pub struct StoryJson {
    pub(crate) text: String,
}

impl StoryJson {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

/// Possible errors that can be produced by [`InkStoryJsonLoaderError`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum InkStoryJsonLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// An [UTF8 Error](std::str::Utf8Error)
    #[error("Could not convert bytes to string: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

/// `InkStoryJsonLoader` is a struct that loads a JSON file containing a story.
pub struct InkStoryJsonLoader;

impl AssetLoader for InkStoryJsonLoader {
    type Asset = StoryJson;
    type Settings = ();
    type Error = InkStoryJsonLoaderError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut text = String::new();
        reader.read_to_string(&mut text).await?;
        // not sure why, but the npx inkjs CLI sometimes inserts a single NBSP
        // byte at the beginning of the file. i could investigate, but this is
        // an easy workaround for now.
        if !text.starts_with("{") {
            text.remove(0);
        }
        Ok(StoryJson::new(text))
    }

    fn extensions(&self) -> &[&str] {
        &[".ink.json"]
    }
}
