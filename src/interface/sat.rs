use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use crate::interface::commons::Sgp4Sat;

#[derive(Component)]
pub struct Satellite;

pub fn setup_sat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let identifier = commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(0.02).mesh().uv(16, 16)))),
        MeshMaterial3d(
            materials.add(
                StandardMaterial {
                    base_color: Color::srgb(0.2, 0.1, 0.2),
                    ..default() }
                    )
                ),
        Transform::from_xyz(1.05, 0.0, 0.0),
        Satellite
    )).id();

    let tle_line_1 = "1 99999U 24001A   25001.00000000  .00000000  00000-0  00000-0 0 0001";
    let tle_line_2 = "2 99999 000.01 000.00 0000010 000.00 120.00 01.00270000    01";
    
    let sat: Sgp4Sat = Sgp4Sat::from_tle_lines("Satellite", tle_line_1, tle_line_2, 300.0, identifier).expect("Invalid TLE");

    
}