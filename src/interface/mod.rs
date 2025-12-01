use bevy::prelude::*;

mod planet;
mod camera;
mod debris;
mod commons;
mod sat;


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
        .add_systems(Startup, (planet::setup, camera::setup, debris::setup_tle))
        .add_systems(Main, planet::rotate_earth)
        .add_systems(Update, camera::orbit_camera)
        .add_systems(Update, debris::burn_debris_system)
        .add_systems(Update, debris::tle_drive_system)
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

