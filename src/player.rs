use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use avian2d::prelude::*;
use avian2d::math::{Scalar, Vector};

use crate::character_controller::CharacterControllerBundle;

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player);
    }
}

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player/initialdesign.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(10.0, 12.0), Vector::NEG_Y * 1500.0)
            .with_movement(1250.0, 0.92, 400.0, (30.0 as Scalar).to_radians()),
        crate::PIXEL_PERFECT_LAYERS,
    ));
}
