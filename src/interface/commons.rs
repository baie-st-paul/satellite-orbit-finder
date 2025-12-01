use bevy::ecs::entity::Entity;
use sgp4::{Elements, Constants};
use bevy::prelude::*;


#[derive(Clone)]
pub struct Sgp4Sat {
    pub element: Elements,// compiled from TLE
    pub constants: Constants,
    time_scale: f64,
    pub entity_id: Entity         // sim seconds per real second
}

// Simple helper to build from 2 TLE lines
impl Sgp4Sat {
    pub fn from_tle_lines(name: &str, l1: &str, l2: &str, time_scale: f64, entity_id: Entity) -> Result<Self> {
        let element = Elements::from_tle(Some(name.to_owned()), l1.as_bytes(), l2.as_bytes())?; // parse TLE
        return Sgp4Sat::from_element(element, time_scale, entity_id);
    }

    pub fn from_element(element: Elements, time_scale: f64, entity_id: Entity) -> Result<Self> {
        let constants = Constants::from_elements(&element).unwrap();
        Ok(Self {
            element,
            constants,
            time_scale,
            entity_id
        })
    }

    // minutes since the TLE epoch for the current sim time
    pub fn minutes_since_epoch(&self, real_elapsed_secs: f64) -> f64 {
        let sim_elapsed = real_elapsed_secs * self.time_scale;    // accelerate time
        sim_elapsed / 60.0
    }
}