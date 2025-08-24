mod camera;
mod common;
mod state;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow, WindowResized};

use camera::CameraPlugin;
use common::CommonPlugin;
use state::WindowSize;

fn main() {
    let platform_api = if cfg!(target_os = "windows") {
        Backends::DX12
    } else if cfg!(target_os = "macos") {
        Backends::METAL
    } else if cfg!(target_os = "linux") {
        Backends::VULKAN
    } else {
        panic!("Unsupported platform!");
    };

    let pkg_name = env!("CARGO_PKG_NAME");

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: Some(platform_api),
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: pkg_name.to_string(),
                        name: Some(pkg_name.to_string()),
                        position: WindowPosition::Centered(
                            MonitorSelection::Primary,
                        ),
                        resolution: Vec2::new(1024.0, 1024.0).into(),
                        present_mode: PresentMode::Fifo,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            CommonPlugin,
            CameraPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup, grab_cursor, window_size))
        .add_systems(Update, on_window_resize)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
}

fn window_size(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.single() {
        commands
            .insert_resource(WindowSize::new(window.width(), window.height()));
    }
}

fn grab_cursor(mut primary_window: Single<&mut Window, With<PrimaryWindow>>) {
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn on_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    for e in resize_reader.read() {
        window_size.width = e.width;
        window_size.height = e.height;
    }
}
