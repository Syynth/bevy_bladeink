use bevy::prelude::*;

use crate::assets::StoryJson;

/// Represents an ink project resource. Insert to kick off the systems that
/// manage the ink project.
#[derive(Resource, Debug, Clone)]
pub struct InkStory {
    asset_path: String,
    handle: Option<Handle<StoryJson>>,
    state: Option<String>,
}

impl InkStory {
    /// Creates a new `InkProject` resource with the given asset path.
    pub fn new(asset_path: impl AsRef<str>) -> Self {
        Self {
            asset_path: asset_path.as_ref().to_string(),
            handle: None,
            state: None,
        }
    }
}

impl InkStory {
    pub(crate) fn asset_path(&self) -> &str {
        &self.asset_path
    }

    pub(crate) fn handle(&self) -> Option<&Handle<StoryJson>> {
        self.handle.as_ref()
    }

    pub(crate) fn set_handle(&mut self, handle: Handle<StoryJson>) {
        self.handle = Some(handle);
    }

    pub(crate) fn state(&self) -> &Option<String> {
        &self.state
    }

    #[expect(dead_code, reason = "TODO")]
    pub(crate) fn set_state(&mut self, state: impl AsRef<str>) {
        self.state = Some(state.as_ref().to_string());
    }
}
