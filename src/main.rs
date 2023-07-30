mod world;
mod camera;
mod player;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera::CameraPlugin;
use crate::player::CarPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new().add_plugins((
        DefaultPlugins,
        WorldPlugin,
        CameraPlugin,
        CarPlugin,
    ))
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin::default()))
        .run();
}
