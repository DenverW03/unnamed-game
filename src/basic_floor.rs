use bevy::{color::palettes::css::WHITE, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use avian2d::prelude::*;

pub struct BasicFloor;

impl Plugin for BasicFloor {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_floor);
    }
}

fn add_floor(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Query<&Window>) {
    if let Ok(window) = window.get_single() {
        let width: f32 = crate::RES_WIDTH as f32;

        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(width - 10.0, 20.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(width - 50.0, 20.0)).into(),
                transform: Transform::from_xyz(0.0, -150.0, 0.0),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },
        ));
    }
}
