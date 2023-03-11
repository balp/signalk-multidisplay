use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct MultiDisplay;

// Design
// 000 cog 0.0 sog
// 000 dpt 0.0 aws

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct BottomRightText;

#[derive(Component)]
struct BottomLeftText;

#[derive(Component)]
struct TopRightText;

#[derive(Component)]
struct TopLeftText;

impl Plugin for MultiDisplay {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_signalk_data)
            .add_startup_system(setup_view)
            .add_startup_system(setup_top_right_text)
            .add_startup_system(setup_top_left_text)
            .add_startup_system(setup_bottom_right_text)
            .add_startup_system(setup_bottom_left_text)
            .add_startup_system(setup_fps_view)
            .add_system(text_update_fps)
            .add_system(text_update_top_right_text)
            .add_system(text_update_top_left_text)
            .add_system(text_update_bottom_right_text)
            .add_system(text_update_bottom_left_text);
    }
}

fn add_signalk_data(mut commands: Commands) {}

fn setup_view(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_top_right_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([TextSection::from_style(
            TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new("SOG", TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 60.0,
                color: Color::RED,
            })
        ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(60.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        TopRightText,
    ));
}

fn setup_top_left_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([TextSection::from_style(
            TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new("COG", TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 60.0,
                color: Color::RED,
            })
        ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(60.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        TopLeftText,
    ));
}

fn setup_bottom_right_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([TextSection::from_style(
            TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new("AWS", TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 60.0,
                color: Color::RED,
            })
        ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        BottomRightText,
    ));
}
fn setup_bottom_left_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([TextSection::from_style(
            TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new("DPT", TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 60.0,
                color: Color::RED,
            })
        ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        BottomLeftText,
    ));
}

fn setup_fps_view(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/Hack-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Regular.ttf"),
                font_size: 16.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

fn text_update_top_right_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<TopRightText>>,
) {
    let seconds = time.elapsed_seconds();
    for mut text in &mut query {
        text.sections[0].value = format!("{seconds:.2}");
    }
}

fn text_update_top_left_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<TopLeftText>>,
) {
    let seconds = time.elapsed_seconds();
    for mut text in &mut query {
        text.sections[0].value = format!("{seconds:.2}");
    }
}

fn text_update_bottom_right_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<BottomRightText>>,
) {
    let seconds = time.elapsed_seconds();
    for mut text in &mut query {
        text.sections[0].value = format!("{seconds:.2}");
    }
}
fn text_update_bottom_left_text(
    time: Res<Time>,
    mut query: Query<&mut Text, With<BottomLeftText>>,
) {
    let seconds = time.elapsed_seconds();
    for mut text in &mut query {
        text.sections[0].value = format!("{seconds:.2}");
    }
}

fn text_update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MultiDisplay)
        .run();
}
