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

fn add_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::new(10.0, 20.0)).into(),
            material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(10.0, 20.0), Vector::NEG_Y * 1500.0)
            .with_movement(1250.0, 0.92, 400.0, (30.0 as Scalar).to_radians()),
    ));
}
