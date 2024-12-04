use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct DebugSetting {
    pub display_chunk_gizmos: bool,
}
