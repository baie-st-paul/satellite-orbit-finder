use bevy::prelude::*;
mod planet;
mod camera;
mod debris;
use chrono::{DateTime, Utc};
use glam::DVec3;
use sgp4::{Elements, Constants, MinutesSinceEpoch};
use crate::interface::debris::Debris;

pub fn init_interface() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }
        ))
        .insert_resource(camera::CameraState::default())
        .insert_resource(SecToSim::default())
        .add_systems(Startup, (planet::setup, camera::setup, setup_tle))
        .add_systems(Main, planet::rotate_earth)
        .add_systems(Update, camera::orbit_camera)
       // .add_systems(Update, (debris::update_forces, debris::update_motion.after(debris::update_forces)))
        .add_systems(Update, debris::burn_debris_system)
        .add_systems(Update, tle_drive_system)
        .run();
}

// Rate of time in simulation
#[derive(Resource)]
pub struct SecToSim(f32);

impl Default for SecToSim {
    fn default() -> Self {
        SecToSim(100.0)//SecToSim(3000.0)        // change this to make time faster in simulation. 1.0 = real time
    }
}

#[derive(Resource)]
struct Sgp4Sat {
    constants: Constants,      // compiled from TLE
    epoch_utc: f64,  // TLE epoch (for reference/logs)
    start_utc: DateTime<Utc>,  // sim reference start time
    time_scale: f64,           // sim seconds per real second
}

// Simple helper to build from 2 TLE lines
impl Sgp4Sat {
    fn from_tle_lines(name: &str, l1: &str, l2: &str, time_scale: f64) -> Result<Self> {
        let el = Elements::from_tle(Some(name.to_owned()), l1.as_bytes(), l2.as_bytes())?; // parse TLE
        let constants = Constants::from_elements(&el)?;  // build propagator
        Ok(Self {
            constants,
            epoch_utc: Elements::epoch(&el),
            start_utc: Utc::now(),
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

fn tle_drive_system(
    time: Res<Time>,
    mut sat: ResMut<Sgp4Sat>,
    mut q_debris: Query<&mut Transform, With<Debris>>,
) {
    // how many real seconds passed
    let dt_real = time.delta_secs_f64();
    // Minutes since epoch for this frame (relative to sim start)
    let mins = MinutesSinceEpoch(sat.minutes_since_epoch(time.delta_secs_f64()));
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

fn setup_tle(
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
    let sat = Sgp4Sat::from_tle_lines("*sat Name*", l1, l2, 60.0).expect("Invalid TLE");
    commands.insert_resource(sat);
}

