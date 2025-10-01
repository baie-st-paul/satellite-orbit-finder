use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2};

use crate::interface::SecToSim;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    // Earth sphere
    let texture = asset_server.load("earth_diffuse.png");
    let material = MeshMaterial3d(materials.add(StandardMaterial{
        base_color_texture: Some(texture),
        perceptual_roughness: 1.0,
        ..default()
    }));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(16, 16))), // adjust uv for geometric resolution of sphere
        material,
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Earth,
    ));

    spawn_atmosphere_layers(&mut commands, &mut meshes, &mut materials);

}

// 1 : 3000 real life vs simulation
// real life: 7.2921159 × 10−5 radians/second
// about 30 seconds = 1 rotation in simulation,
pub fn rotate_earth(mut query: Query<&mut Transform, With<Earth>>, time: Res<Time>, multiplier: Res<SecToSim>) {
    let angular_velocity = 7.2921159e-5;
    for mut transform in &mut query {
        transform.rotate_y(multiplier.0 * angular_velocity * time.delta_secs());
    }
}

#[derive(Component)]
pub struct Earth;

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
            ).mesh().ico(16).unwrap())),    //adjust ico(n) for number of division (smoother for larger n)
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


//earth mass = 5.97219e24 kg; G = 6.6743e-11; MU_M3_S2 = earth mass * G
pub const GM: f32 = 3.986019e14;
