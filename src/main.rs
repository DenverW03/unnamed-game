use bevy::prelude::*;
use avian2d::prelude::*;
use avian2d::math::Vector;

use basic_floor::BasicFloor;
use character_controller::CharacterControllerPlugin;
use player::Player;

pub mod basic_floor;
pub mod player;
pub mod character_controller;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_plugins((BasicFloor, Player, CharacterControllerPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
