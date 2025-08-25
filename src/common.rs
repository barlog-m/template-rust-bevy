use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const OCEANIC_WHITE: LinearRgba = LinearRgba::rgb(0.52, 0.56, 0.62);
/*
const OCEANIC_BLACK: LinearRgba = LinearRgba::rgb(0.01, 0.02, 0.03);
const OCEANIC_GRAY: LinearRgba = LinearRgba::rgb(0.13, 0.17, 0.20);
const OCEANIC_RED: LinearRgba = LinearRgba::rgb(0.85, 0.11, 0.13);
const OCEANIC_ORANGE: LinearRgba = LinearRgba::rgb(0.96, 0.29, 0.09);
const OCEANIC_GREEN: LinearRgba = LinearRgba::rgb(0.32, 0.57, 0.30);
const OCEANIC_YELLOW: LinearRgba = LinearRgba::rgb(0.96, 0.57, 0.13);
const OCEANIC_BLUE: LinearRgba = LinearRgba::rgb(0.13, 0.32, 0.60);
const OCEANIC_BLUE_LIGHT: LinearRgba = LinearRgba::rgb(0.51, 0.69, 0.93);
const OCEANIC_MAGENTA: LinearRgba = LinearRgba::rgb(0.56, 0.30, 0.56);
const OCEANIC_CYAN: LinearRgba = LinearRgba::rgb(0.12, 0.45, 0.45);
*/

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FrameTimeDiagnosticsPlugin::new(32),))
            .add_systems(Startup, setup)
            .add_systems(Update, update_fps_text)
            .add_systems(Update, exit);
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new("-1 fps -1 ms/frame"),
        TextFont {
            font: default(),
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::from(OCEANIC_WHITE)),
        FpsText,
    ));
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_value) = fps.smoothed() {
                if let Some(time) =
                    diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                {
                    if let Some(time_value) = time.smoothed() {
                        text.clear();
                        text.push_str(&format!(
                            "{fps_value:.2} fps {time_value:.2} ms/frame"
                        ));
                    }
                }
            }
        }
    }
}

fn exit(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit_events: EventWriter<AppExit>,
) {
    if keys.any_just_pressed([KeyCode::Escape]) {
        exit_events.write(AppExit::Success);
    }
}
