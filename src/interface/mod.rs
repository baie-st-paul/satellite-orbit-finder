use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

pub fn init_interface() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }
        ))
        .insert_resource(CameraState::default())
        .add_systems(Startup, setup)
        .add_systems(Main, rotate_earth)
        .add_systems(Update, orbit_camera)
        .run();
}

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct OrbitCamera;

#[derive(Resource)]
struct CameraState {
    yaw: f32,
    pitch: f32,
    radius: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            radius: 4.0,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.5, 3.5, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera
    ));

    // Light
    commands.spawn((PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));

    // Earth sphere
    let texture = asset_server.load("earth_diffuse.png");
    let material = MeshMaterial3d(materials.add(StandardMaterial{
        base_color_texture: Some(texture),
        perceptual_roughness: 1.0,
        ..default()
    }));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        material,
        Transform::from_xyz(0.0, 0.0, 0.0),
        Earth,
    ));

}

fn rotate_earth(mut query: Query<&mut Transform, With<Earth>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(0.2 * time.delta_secs());
    }
}

fn orbit_camera(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<&mut Transform, With<OrbitCamera>>,
    mut state: ResMut<CameraState>,
) {
    // Zoom
    for event in mouse_wheel_events.read() {
        state.radius -= event.y * 0.5;
        state.radius = state.radius.clamp(2.0, 20.0);
    }

    // Mouse drag to rotate
    if mouse_button.pressed(MouseButton::Left) {
        for event in mouse_motion_events.read() {
            state.yaw += event.delta.x * 0.01;
            state.pitch += event.delta.y * 0.01;
            state.pitch = state.pitch.clamp(-1.5, 1.5); // prevent flipping
        }
    }

    // Convert spherical to Cartesian
    let x = state.radius * state.yaw.cos() * state.pitch.cos();
    let y = state.radius * state.pitch.sin();
    let z = state.radius * state.yaw.sin() * state.pitch.cos();

    let position = Vec3::new(x, y, z);
    let mut camera_transform = camera_query
        .single_mut()
        .expect("Camera not found");

    camera_transform.translation = position;
    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}


