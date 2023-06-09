use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_mod_reqwest::*;
use clap::Parser;
use serde_json;
use serde_json::Error;
use signalk;
use signalk::{V1FullFormat, V1Vessel};

pub struct MultiDisplay;

#[derive(Resource)]
struct Configuration {
    server_uri: String,
}

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

impl CourseOverGround {
    fn set_from_vessel(&mut self, vessel: &V1Vessel) {
        if let Some(ref nav) = vessel.navigation {
            if let Some(ref cog_val) = nav.course_over_ground_magnetic {
                if let Some(cog) = cog_val.value {
                    self.value = cog;
                }
            }
            if let Some(ref cog_val) = nav.course_over_ground_true {
                if let Some(cog) = cog_val.value {
                    self.value = cog;
                }
            }
        }
    }
}

#[derive(Component)]
struct SpeedOverGround {
    value: f64,
}

impl SpeedOverGround {
    fn set_from_vessel(&mut self, vessel: &V1Vessel) {
        if let Some(ref nav) = vessel.navigation {
            if let Some(ref sog_val) = nav.speed_over_ground {
                if let Some(sog) = sog_val.value {
                    self.value = sog;
                }
            }
        }
    }
}

#[derive(Component)]
struct Depth {
    value: f64,
}

impl Depth {
    fn set_from_vessel(&mut self, vessel: &V1Vessel) {
        if let Some(ref env) = vessel.environment {
            if let Some(ref env_depth) = env.depth {
                if let Some(ref depth_num_value) = env_depth.below_transducer {
                    if let Some(d) = depth_num_value.value {
                        self.value = d;
                    }
                }
            }
        }
    }
}

#[derive(Component)]
struct WaterTemperature {
    value: f64,
}

impl WaterTemperature {
    fn set_from_vessel(&mut self, vessel: &V1Vessel) {
        if let Some(ref env) = vessel.environment {
            if let Some(ref env_water) = env.water {
                if let Some(ref water_temp_num_value) = env_water.temperature {
                    if let Some(wt) = water_temp_num_value.value {
                        self.value = wt;
                    }
                }
            }
        }
    }
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
            .add_system(bevy::window::close_on_esc)
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://demo.signalk.org/signalk/v1/api/")]
    server: String,

    #[arg(short, long, default_value_t = 5.0)]
    delay: f32,
}
fn main() {
    let args = Args::parse();

    let configuration = Configuration {
        server_uri: args.server.clone(),
    };
    App::new()
        .insert_resource(configuration)
        .add_plugins(DefaultPlugins)
        .add_plugin(ReqwestPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MultiDisplay)
        .add_system(get_sk_data)
        .add_system(handle_sk_data)
        .insert_resource(ReqTimer(Timer::new(
            std::time::Duration::from_secs_f32(args.delay),
            TimerMode::Repeating,
        )))
        .run();
}

#[derive(Resource)]
struct ReqTimer(pub Timer);

fn get_sk_data(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ReqTimer>,
    configuration: Res<Configuration>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        bevy::log::debug!("Signalk uri: {}", configuration.server_uri);
        if let Ok(url) = reqwest::Url::parse(&*configuration.server_uri) {
            let req = reqwest::Request::new(reqwest::Method::GET, url);
            let req = ReqwestRequest(Some(req));
            commands.spawn(req);
        }
    }
}

fn get_depth_value(vessel: &V1Vessel) -> Option<f64> {
    if let Some(ref env) = vessel.environment {
        if let Some(ref env_depth) = env.depth {
            if let Some(ref depth_num_value) = env_depth.below_transducer {
                return depth_num_value.value;
            }
        }
    }
    None
}

fn get_water_temp_value(vessel: &V1Vessel) -> Option<f64> {
    if let Some(ref env) = vessel.environment {
        if let Some(ref env_water) = env.water {
            if let Some(ref water_temp_num_value) = env_water.temperature {
                return water_temp_num_value.value;
            }
        }
    }
    None
}

fn get_course_over_ground(vessel: &V1Vessel) -> Option<f64> {
    if let Some(ref nav) = vessel.navigation {
        if let Some(ref cog_mag_val) = nav.course_over_ground_magnetic {
            return cog_mag_val.value;
        }
        if let Some(ref cog_true_val) = nav.course_over_ground_true {
            return cog_true_val.value;
        }
    }
    None
}

fn get_speed_over_ground(vessel: &V1Vessel) -> Option<f64> {
    if let Some(ref nav) = vessel.navigation {
        if let Some(ref sog_val) = nav.speed_over_ground {
            return sog_val.value;
        }
    }
    None
}

fn res_to_vessel(res: &ReqwestBytesResult) -> Option<V1Vessel> {
    if let Some(signalk_json_data) = res.as_str() {
        let opt_sk_data: Result<V1FullFormat, Error> = serde_json::from_str(signalk_json_data);
        if let Ok(sk_data) = opt_sk_data {
            return (sk_data.get_self()).cloned();
        }
    }
    None
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
        if let Some(self_vessel) = res_to_vessel(res) {
            for ref mut cog_v in &mut course_over_ground {
                cog_v.set_from_vessel(&self_vessel);
            }
            for ref mut sog_v in &mut speed_over_ground {
                sog_v.set_from_vessel(&self_vessel);
            }
            for mut depth_v in &mut depth {
                depth_v.set_from_vessel(&self_vessel);
            }
            for ref mut temp_v in &mut water_temperature {
                temp_v.set_from_vessel(&self_vessel);
            }
        }
        // Done with this entity
        commands.entity(e).despawn_recursive();
    }
}
