use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderSettings {
    pub render_distance: (u32, u32),
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            render_distance: (2, 2),
        }
    }
}
