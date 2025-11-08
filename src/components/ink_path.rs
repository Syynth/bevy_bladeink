use bevy::prelude::*;

/// Creates a new `InkPath` instance.
#[derive(Component)]
pub struct InkPath {
    #[expect(dead_code, reason = "todo")]
    pub(crate) path: String,
}

impl InkPath {
    /// Creates a new `InkPath` instance.
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}
