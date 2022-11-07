use bevy::{prelude::*, render::camera, math::vec3};
use bevy_rapier2d::prelude::*;

fn spawn_entity(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: vec3(50., 50., 1.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5));
}

fn resize_collider(mut colliders: Query<&mut Collider>) {
    let mut collider = colliders.single_mut();
    let mut cuboid = collider.as_cuboid_mut().unwrap();
    cuboid.sed_half_extents(cuboid.half_extents() * 1.01);
    println!("{}", cuboid.half_extents());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_startup_system(spawn_entity)
        .add_system(resize_collider)
        .run()
}
