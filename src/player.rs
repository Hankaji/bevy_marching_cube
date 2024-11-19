use bevy::prelude::*;

pub struct PlayerPLugin;

impl Plugin for PlayerPLugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player,
    ));
}

#[derive(Component)]
pub struct Player;

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    for mut player_transform in player_q.iter_mut() {
        let mut dir = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            dir += *player_transform.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            dir += *player_transform.back();
        }
        if keys.pressed(KeyCode::KeyA) {
            dir += *player_transform.left();
        }
        if keys.pressed(KeyCode::KeyD) {
            dir += *player_transform.right();
        }

        const VELOCITY: f32 = 6.0;

        let movement = dir.normalize_or_zero() * VELOCITY * time.delta_seconds();
        player_transform.translation += movement;
    }
}
