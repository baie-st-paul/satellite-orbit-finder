use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use glam::DVec3;
use sgp4::{Elements, Constants, MinutesSinceEpoch};

#[derive(Component)]
pub struct Debris;

pub fn burn_debris_system(
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

/// Generate a line mesh from orbit points
fn orbit_line_mesh(points: &[Vec3]) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::RENDER_WORLD);

    // convert points into vertex buffer
    let positions: Vec<[f32; 3]> = points.iter().map(|p| p.to_array()).collect();

    // optional color per vertex (white)
    let colors: Vec<[f32; 4]> = points.iter()
        .map(|_| [1.0, 1.0, 1.0, 1.0])
        .collect();

    // indices for the line strip
    let indices: Vec<u32> = (0..points.len() as u32).collect();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    
    return mesh
}

fn generate_orbit_points(elements: &Elements, constants: &Constants, samples: usize) -> Vec<Vec3> {
    let mut points = Vec::new();

    // Compute orbital period in minutes
    let mean_motion_rev_per_day = elements.mean_motion; // rev/day
    let period_minutes = 1440.0 / mean_motion_rev_per_day; // minutes per orbit

    for i in 0..samples {
        let minutes = (i as f64) * (period_minutes / samples as f64);
        if let Ok(pred) = constants.propagate(MinutesSinceEpoch(minutes)) {
            let r = DVec3::from_array(pred.position); // km
            let scale = 1.0 / 6371.0; // normalize
            points.push(Vec3::new(
                (r.x * scale) as f32,
                (r.y * scale) as f32,
                (r.z * scale) as f32,
            ));
        }
    }
    points
}

#[derive(Resource, Clone)]
pub struct Sgp4Sat {
    element: Elements,// compiled from TLE
    constants: Constants,
    time_scale: f64,           // sim seconds per real second
}

// Simple helper to build from 2 TLE lines
impl Sgp4Sat {
    fn from_tle_lines(name: &str, l1: &str, l2: &str, time_scale: f64) -> Result<Self> {
        let element = Elements::from_tle(Some(name.to_owned()), l1.as_bytes(), l2.as_bytes())?; // parse TLE
        let constants = Constants::from_elements(&element).unwrap();
        Ok(Self {
            element,
            constants,
            time_scale,
        })
    }

    // minutes since the TLE epoch for the current sim time
    fn minutes_since_epoch(&self, real_elapsed_secs: f64) -> f64 {
        let sim_elapsed = real_elapsed_secs * self.time_scale;    // accelerate time
        sim_elapsed / 60.0
    }
}

const EARTH_RADIUS_KM: f32 = 6371.0;

pub fn tle_drive_system(
    time: Res<Time>,
    sat: ResMut<Sgp4Sat>,
    mut q_debris: Query<&mut Transform, With<Debris>>,
) {
    // Minutes since epoch for this frame (relative to sim start)
    let mins = MinutesSinceEpoch(sat.minutes_since_epoch(time.elapsed_secs_f64()));
    // Propagate with SGP4
    if let Ok(pred) = sat.constants.propagate(mins) {
        // pred.position is in TEME (km)
        let r_km: DVec3 = DVec3::from_array(pred.position); // glam::DVec3-like but f64 tuple in km
        // km → scene units (1 unit = 6371 km)
        let to_units = 1.0 / (EARTH_RADIUS_KM as f64);
        let x = (r_km.x * to_units) as f32;
        let y = (r_km.y * to_units) as f32;
        let z = (r_km.z * to_units) as f32;

        // Move all Debris with this TLE (or scope to a specific entity if you prefer)
        for mut tf in &mut q_debris {
            tf.translation = Vec3::new(x, y, z);
        }
    }
}

pub fn setup_tle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Example: ISS from CelesTrak “stations” group (paste current lines)
    // Fetch TLEs from CelesTrak or Space-Track. (Examples linked below.)
    let l1 = "1 00033U 60003  C 60280.33271408  .00001932 +00000-0 +00000-0 0  9994";
    let l2 = "2 00033 051.2879 136.0668 0242757 196.7800 163.6100 15.38343354027003";

    // Create a debris entity (render only; position will be overwritten by SGP4 each frame)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere::new(0.02).mesh().uv(16, 16)))),
        MeshMaterial3d(
            materials.add(
                StandardMaterial {
                     base_color: Color::srgb(1.0, 0.2, 0.2),
                      ..default() }
                    )
                ),
        Transform::from_xyz(1.05, 0.0, 0.0),
        Debris
    ));

    // Register the SGP4 resource (e.g., 60× time)
    let sat = Sgp4Sat::from_tle_lines("*sat Name*", l1, l2, 300.0).expect("Invalid TLE");
    let sat_copy = sat.clone();
    commands.insert_resource(sat);
    
    let orbit_points = generate_orbit_points(&sat_copy.element, &sat_copy.constants, 256);
    let orbit_mesh = orbit_line_mesh(&orbit_points);

    commands.spawn( (
        Mesh3d(meshes.add(orbit_mesh)),
        MeshMaterial3d( materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.7, 1.0),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::IDENTITY,
    ));
}