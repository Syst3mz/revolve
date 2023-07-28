mod world;
mod camera;
mod player;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, WorldPlugin, CameraPlugin, PlayerPlugin))
        .run();
}
