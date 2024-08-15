use bevy::{prelude::*, render::{camera::RenderTarget, view::RenderLayers}, window::WindowResized};
use avian2d::prelude::*;
use avian2d::math::Vector;

use basic_floor::BasicFloor;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use character_controller::CharacterControllerPlugin;
use player::Player;

pub mod basic_floor;
pub mod player;
pub mod character_controller;

// In-game resolution
const RES_WIDTH: u32 = 640;
const RES_HEIGHT: u32 = 360;

const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), PhysicsPlugins::default()))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .insert_resource(Msaa::Off)
        .add_plugins((BasicFloor, Player, CharacterControllerPlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, fit_canvas)
        .run();
}

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    //commands.spawn(Camera2dBundle::default());

    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // Canvas representing the low res screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // This camera renders whatever is on the 'PIXEL_PERFECT_LAYERS' to the canvas
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));

    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
