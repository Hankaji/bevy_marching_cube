use bevy::prelude::*;
use debug::DebugSetting;

pub mod debug;
mod key_bindings;

pub struct SettingPlugin;

impl Plugin for SettingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .init_resource::<DebugSetting>();
    }
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}
