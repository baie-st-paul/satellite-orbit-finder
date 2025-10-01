use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseMotion;

pub fn setup(mut commands: Commands) {
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
}

#[derive(Component)]
pub struct OrbitCamera;

#[derive(Resource)]
pub struct CameraState {
    pub(crate) yaw: f32,
    pub(crate) pitch: f32,
    pub(crate) radius: f32,
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

pub fn orbit_camera(
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
