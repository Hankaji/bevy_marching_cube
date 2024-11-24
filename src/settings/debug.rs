use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct DebugSetting {
    display_chunk_gizmos: bool,
}
