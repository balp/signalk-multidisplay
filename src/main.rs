use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_mod_reqwest::*;
use serde_json;
use signalk;

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

#[derive(Component)]
struct ShipData;

#[derive(Component)]
struct CourseOverGround {
    value: f64,
}
#[derive(Component)]
struct SpeedOverGround {
    value: f64,
}
#[derive(Component)]
struct Depth {
    value: f64,
}
#[derive(Component)]
struct WaterTemperature {
    value: f64,
}

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

fn add_signalk_data(mut commands: Commands) {
    commands
        .spawn(ShipData)
        .insert(SpeedOverGround { value: 0.0 })
        .insert(CourseOverGround { value: 0.0 })
        .insert(Depth { value: 0.0 })
        .insert(WaterTemperature { value: 0.0 });
}

fn setup_view(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn setup_top_left_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                "SOG",
                TextStyle {
                    font: asset_server.load("fonts/Hack-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::RED,
                },
            ),
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

fn setup_top_right_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                "COG",
                TextStyle {
                    font: asset_server.load("fonts/Hack-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::RED,
                },
            ),
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

fn setup_bottom_right_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                "WT",
                TextStyle {
                    font: asset_server.load("fonts/Hack-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::RED,
                },
            ),
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
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Hack-Bold.ttf"),
                font_size: 200.0,
                color: Color::WHITE,
            }),
            TextSection::new(
                "DPT",
                TextStyle {
                    font: asset_server.load("fonts/Hack-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::RED,
                },
            ),
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

fn text_update_top_left_text(
    mut query: Query<&mut Text, With<TopLeftText>>,
    speed_over_ground: Query<&SpeedOverGround, With<ShipData>>,
) {
    for sog in &speed_over_ground {
        let value = sog.value;
        for mut text in &mut query {
            text.sections[0].value = format!("{value:.1}");
        }
    }
}

fn text_update_top_right_text(
    mut query: Query<&mut Text, With<TopRightText>>,
    course_over_ground: Query<&CourseOverGround, With<ShipData>>,
) {
    for cog in &course_over_ground {
        let value = (cog.value * 180.0) / std::f64::consts::PI;
        for mut text in &mut query {
            text.sections[0].value = format!("{value:.1}");
        }
    }
}

fn text_update_bottom_left_text(
    mut query: Query<&mut Text, With<BottomLeftText>>,
    depth: Query<&Depth, With<ShipData>>,
) {
    for d in &depth {
        let value = d.value;
        for mut text in &mut query {
            text.sections[0].value = format!("{value:.1}");
        }
    }
}

fn text_update_bottom_right_text(
    mut query: Query<&mut Text, With<BottomRightText>>,
    water_temperature: Query<&WaterTemperature, With<ShipData>>,
) {
    for temp in &water_temperature {
        let value = temp.value - 273.15;
        for mut text in &mut query {
            text.sections[0].value = format!("{value:.1}");
        }
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
        .add_plugin(ReqwestPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MultiDisplay)
        .add_system(get_sk_data)
        .add_system(handle_sk_data)
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .run();
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

fn get_sk_data(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ReqTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Ok(url) = "http://demo.signalk.org/signalk/v1/api/".try_into() {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest(Some(req));
            commands.spawn(req);
        }
    }
}

fn handle_sk_data(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult)>,
    mut course_over_ground: Query<&mut CourseOverGround, With<ShipData>>,
    mut speed_over_ground: Query<&mut SpeedOverGround, With<ShipData>>,
    mut depth: Query<&mut Depth, With<ShipData>>,
    mut water_temperature: Query<&mut WaterTemperature, With<ShipData>>,
) {
    for (e, res) in results.iter() {
        let signalk_json_data = res.as_str().unwrap();
        bevy::log::info!("{signalk_json_data}");
        if let Ok(json_data) = serde_json::from_str(signalk_json_data) {
            let sk_data: signalk::V1FullFormat = json_data;
            if let Some(self_vessel) = sk_data.get_self() {
                if let Some(ref env) = self_vessel.environment {
                    if let Some(ref env_depth) = env.depth {
                        if let Some(ref depth_num_value) = env_depth.below_transducer {
                            if let Some(depth_value) = depth_num_value.value {
                                bevy::log::info!("depth: {}", depth_value);
                                for (mut depth_v) in &mut depth {
                                    depth_v.value = depth_value;
                                }
                            }
                        }
                    }
                    if let Some(ref env_water) = env.water {
                        if let Some(ref water_temp_num_value) = env_water.temperature {
                            if let Some(temp_value) = water_temp_num_value.value {
                                bevy::log::info!("Water Temp: {}", temp_value);
                                for (mut temp_v) in &mut water_temperature {
                                    temp_v.value = temp_value;
                                }
                            }
                        }
                    }
                }
                if let Some(ref nav) = self_vessel.navigation {
                    if let Some(ref cog_mag_val) = nav.course_over_ground_magnetic {
                        if let Some(cog_mag) = cog_mag_val.value {
                            bevy::log::info!("cog_mag: {}", cog_mag);
                            for (mut cog_v) in &mut course_over_ground {
                                cog_v.value = cog_mag;
                            }
                        }
                    }
                    if let Some(ref cog_true_val) = nav.course_over_ground_true {
                        if let Some(cog_true) = cog_true_val.value {
                            bevy::log::info!("cog_true: {}", cog_true);
                            for (mut cog_v) in &mut course_over_ground {
                                cog_v.value = cog_true;
                            }
                        }
                    }
                    if let Some(ref cog_val) = nav.speed_over_ground {
                        if let Some(sog) = cog_val.value {
                            bevy::log::info!("sog: {}", sog);
                            for (mut sog_v) in &mut speed_over_ground {
                                sog_v.value = sog;
                            }
                        }
                    }
                    if let Some(ref pos) = nav.position {
                        bevy::log::info!(
                            "Position: lat {} long {}",
                            pos.value.latitude,
                            pos.value.longitude
                        );
                    }
                }
            }
        }

        // Done with this entity

        commands.entity(e).despawn_recursive();
    }
}
