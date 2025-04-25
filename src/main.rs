mod camera;

use bevy::prelude::*;

use camera::CameraPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .run();
}
