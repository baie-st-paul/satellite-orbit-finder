use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::service::schema::tle_object)]
#[diesel(check_for_backend(diesel::postgres))]
pub struct TleObject {
    pub norad_cat_id: i32,
    pub ordinal: i32,
    pub comment: String,
    pub originator: String,
    pub object_name: String,
    pub object_type: String,
    pub classification_type: String,
    pub intldes: String,
    pub epoch: NaiveDateTime,
    pub epoch_microseconds: i32,
    pub mean_motion: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub ra_of_asc_node: f64,
    pub arg_of_pericenter: f64,
    pub mean_anomaly: f64,
    pub ephemeris_type: i32,
    pub element_set_no: i32,
    pub rev_at_epoch: i32,
    pub bstar: f64,
    pub mean_motion_dot: f64,
    pub mean_motion_ddot: f64,
    pub file: i32,
    pub tle_line0: String,
    pub tle_line1: String,
    pub tle_line2: String,
    pub object_id: String,
    pub object_number: i32,
    pub semimajor_axis: f64,
    pub period: f64,
    pub apogee: f64,
    pub perigee: f64,
    pub decayed: bool,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::service::schema::sat_cat_object)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SatCatObject {
    pub object_number: i32,
    pub intldes: String,
    pub norad_cat_id: i32,
    pub object_type: String,
    pub satname: String,
    pub country: String,
    pub launch: NaiveDate,
    pub site: String,
    pub decay: NaiveDate,
    pub period: f64,
    pub inclination: f64,
    pub apogee: f64,
    pub perigee: f64,
    pub comment: String,
    pub commentcode: String,
    pub rcsvalue: f64,
    pub rcs_size: String,
    pub file: i32,
    pub launch_year: i32,
    pub launch_num: i32,
    pub launch_piece: String,
    pub current: bool,
    pub object_name: String,
    pub object_id: String,
}