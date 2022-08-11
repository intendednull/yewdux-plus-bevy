use async_std::channel::Receiver;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::state::State;

pub const CANVAS_ID: &str = "engine-canvas";

pub fn start(receiver: Receiver<State>) {
    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some(format!("#{}", CANVAS_ID)),
            height: 800.,
            width: 800.,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(receiver)
        .insert_resource(State::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_state)
        .add_system(update_blocks.after(update_state))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert_bundle((RigidBody::Fixed, Collider::cuboid(5., 0.01, 5.)));
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_state(receiver: Res<Receiver<State>>, mut state: ResMut<State>) {
    if let Ok(val) = receiver.try_recv() {
        info!("State received: {:?}", val);
        *state = val;
    }
}

#[derive(Component)]
struct Block;

fn update_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    blocks: Query<&Block>,
    state: Res<State>,
) {
    if !state.is_changed() {
        return;
    }

    let count = blocks.iter().count();
    info!("Block count is: {}", count);

    for _ in 0..state.count - count {
        // cube
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 3.5, 0.0),
                ..default()
            })
            .insert_bundle((RigidBody::Dynamic, Collider::cuboid(0.5, 0.5, 0.5), Block));
    }
}
