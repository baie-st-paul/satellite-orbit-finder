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
        .insert_resource(SecToSim::default())
        .add_systems(Startup, setup)
        .add_systems(Main, rotate_earth)
        .add_systems(Update, orbit_camera)
        .run();
}

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct AtmosphereLayer;

fn spawn_atmosphere_layers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let km_to_sim = 1.0 / 6371.0;

    let layers = [
        (1.0, 12.0, Color::srgba(0.2, 0.7, 1.0, 0.05)), // Troposphere
        (12.1, 50.0, Color::srgba(0.0, 0.5, 1.0, 0.04)), // Stratosphere
        (50.1, 85.0, Color::srgba(1.0, 0.3, 0.0, 0.03)), // Mesosphere
        (85.1, 600.0, Color::srgba(0.8, 0.1, 0.6, 0.02)), // Thermosphere
        (600.1, 10000.0, Color::srgba(0.9, 0.9, 1.0, 0.01)), // Exosphere
    ];

    for (alt_min, alt_max, color) in layers {
        let inner = (6371.0 + alt_min) * km_to_sim;
        let outer = (6371.0 + alt_max) * km_to_sim;
        let radius = (inner + outer) / 2.0;

        commands.spawn((
            Mesh3d( meshes.add(Sphere::new(
                radius
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
            AtmosphereLayer
        ));
    }
}

#[derive(Component)]
struct Debris;

fn burn_debris_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Debris>>,
) {
    let burn_radius = (6371.0 + 85.0) / 6371.0; // normalized burn zone radius

    for (entity, transform) in query.iter() {
        let distance = transform.translation.length();

        if distance <= burn_radius {
            println!(" Debris burned in atmosphere!");
            commands.entity(entity).despawn();
        }
    }
}



#[derive(Component)]
struct OrbitCamera;

// Rate of time in simulation
#[derive(Resource)]
struct SecToSim(f32);

impl Default for SecToSim {
    fn default() -> Self {
        SecToSim(3000.0)
    }
}

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

    spawn_atmosphere_layers(&mut commands, &mut meshes, &mut materials);
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(8).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.2, 0.0), // just inside burn zone
        Debris,
    ));
}

// 1 : 3000 real life vs simulation
// real life: 7.2921159 × 10−5 radians/second
// about 30 seconds = 1 rotation in simulation,
fn rotate_earth(mut query: Query<&mut Transform, With<Earth>>, time: Res<Time>, multiplier: Res<SecToSim>) {
    let angular_velocity = 7.2921159e-5;
    for mut transform in &mut query {
        transform.rotate_y(multiplier.0 * angular_velocity * time.delta_secs());
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



