use bevy::prelude::*;
use avian2d::prelude::*;

use basic_floor::BasicFloor;

pub mod basic_floor;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(BasicFloor)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
