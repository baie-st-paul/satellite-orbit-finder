use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_camera)
        .add_system(zoom_camera)
        .run();
}

#[derive(Component)]
struct Planet;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Earth Sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 64,
            })),
            material: materials.add(Color::rgb(0.2, 0.4, 1.0).into()), // You can load texture here
            ..default()
        },
        Planet,
    ));
}

fn rotate_camera(
    mut motion_evr: EventReader<MouseMotion>,
    mouse_button: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        let delta = motion_evr.iter().map(|m| m.delta).sum::<Vec2>();
        let mut transform = query.single_mut();
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-delta.x * 0.005));
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(-delta.y * 0.005));
    }
}

fn zoom_camera(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut transform = query.single_mut();
    for ev in scroll_evr.iter() {
        transform.translation += transform.forward() * ev.y * 0.1;
    }
}
