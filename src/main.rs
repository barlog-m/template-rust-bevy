mod camera;

use bevy::prelude::*;

use camera::CameraPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .run();
}
