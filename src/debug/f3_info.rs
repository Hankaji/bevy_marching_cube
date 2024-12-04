use bevy::{
    color::palettes::{
        css::{BLACK, GREEN, RED, YELLOW},
        tailwind::GREEN_500,
    },
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::player::Player;

#[derive(Component)]
pub struct F3Info;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct PlayerCoordF3;

#[derive(Component)]
pub struct ChunkCoordF3;

fn dispay_info(mut commands: Commands) {
    let style = TextStyle {
        font_size: 20.0,
        ..default()
    };

    let text_bg_color = Color::from(BLACK.with_alpha(0.25));

    // Left column
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_grow: 1.,
                margin: UiRect::ZERO,
                ..default()
            },
            ..default()
        })
        .insert(F3Info)
        .with_children(|builder| {
            // -------------------- FPS --------------------
            builder.spawn((
                // Create a TextBundle that has a Text with a list of sections.
                TextBundle::from_sections([
                    TextSection::new("FPS: ", style.clone()),
                    TextSection::from_style(style.clone()),
                ])
                .with_background_color(text_bg_color),
                FpsText,
            ));

            // -------------------- Chunk coordinate --------------------
            builder.spawn((
                // Create a TextBundle that has a Text with a list of sections.
                TextBundle::from_sections([
                    TextSection::new("Player coordinate [", style.clone()),
                    TextSection::from_style(style.clone()),
                    TextSection::new("]", style.clone()),
                ])
                .with_background_color(text_bg_color),
                PlayerCoordF3,
            ));

            // -------------------- Chunk coordinate --------------------
            builder.spawn((
                // Create a TextBundle that has a Text with a list of sections.
                TextBundle::from_sections([
                    TextSection::new("Chunk coordinate [", style.clone()),
                    TextSection::new("0 0 0", style.clone()),
                    TextSection::new("]", style.clone()),
                ])
                .with_background_color(text_bg_color),
                ChunkCoordF3,
            ));
        });
}

pub(super) fn toggle_text_visibility(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    f3_info_q: Query<Entity, With<F3Info>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        if let Ok(f3_info_e) = f3_info_q.get_single() {
            commands.entity(f3_info_e).despawn_recursive();
        } else {
            dispay_info(commands);
        }
    }
}

pub(super) fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_q: Query<&mut Text, With<FpsText>>,
) {
    if let Ok(mut fps_text) = fps_text_q.get_single_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(v) = fps.smoothed() {
                fps_text.sections[1].value = format!("{v:.2}");
                fps_text.sections[1].style.color = match v {
                    ..30.0 => RED.into(),
                    30.0..45.0 => YELLOW.into(),
                    _ => GREEN_500.into(),
                }
            }
        }
    }
}

pub(super) fn update_curr_chunk(
    player_q: Query<&Transform, With<Player>>,
    mut chunk_coord_f3_q: Query<&mut Text, With<ChunkCoordF3>>,
) {
    if let Ok(mut chunk_coord_text) = chunk_coord_f3_q.get_single_mut() {
        let Ok(player_transform) = player_q.get_single() else {
            return;
        };

        let mut player_coord = player_transform.translation;
        player_coord /= 15.0;
        player_coord = player_coord.floor();

        chunk_coord_text.sections[1].value = format!(
            "{:.0} {:.0} {:.0}",
            player_coord.x, player_coord.y, player_coord.z
        );
    }
}

pub(super) fn update_player_position(
    player_q: Query<&Transform, With<Player>>,
    mut player_coord_f3_q: Query<&mut Text, With<PlayerCoordF3>>,
) {
    if let Ok(mut player_coord_text) = player_coord_f3_q.get_single_mut() {
        let Ok(player_transform) = player_q.get_single() else {
            return;
        };

        let player_coord = player_transform.translation;

        player_coord_text.sections[1].value = format!(
            "{:.2} {:.2} {:.2}",
            player_coord.x, player_coord.y, player_coord.z
        );
    }
}
