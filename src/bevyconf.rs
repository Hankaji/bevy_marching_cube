use bevy::prelude::*;
use bevy_mod_billboard::plugin::BillboardPlugin;

const RES: (f32, f32) = (0.9 * 1920.0, 0.9 * 1080.0);

pub struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Project T Revamped".to_string(),
                resizable: false,
                resolution: RES.into(),
                // mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..Default::default()
        }))
        .add_plugins(BillboardPlugin);
    }
}
