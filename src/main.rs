use std::f32::consts::PI;

use bevy::prelude::*;
use bevyconf::BevyConfigPlugin;
use debug::DebugPlugin;
use fly_cam::FlyCamPlugin;
use map_generator::MapGeneratorPlugin;
use player::PlayerPlugin;
use settings::SettingPlugin;

mod bevyconf;
mod debug;
mod fly_cam;
mod map_generator;
mod player;
mod settings;

fn main() {
    App::new()
        .add_plugins(DebugPlugin)
        .add_plugins(BevyConfigPlugin)
        .add_plugins(SettingPlugin)
        .add_plugins((PlayerPlugin, FlyCamPlugin, MapGeneratorPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Directional sunlight
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.0),
            ..default()
        },
        ..default()
    });
}
