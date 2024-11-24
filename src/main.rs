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
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 24.0, 8.0),
        ..default()
    });
}
