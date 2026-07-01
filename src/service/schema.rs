// @generated automatically by Diesel CLI.

diesel::table! {
    sat_cat_object (object_number) {
        object_number -> Int4,
        #[max_length = 255]
        intldes -> Varchar,
        norad_cat_id -> Int4,
        #[max_length = 255]
        object_type -> Varchar,
        #[max_length = 255]
        satname -> Varchar,
        #[max_length = 255]
        country -> Varchar,
        launch -> Date,
        #[max_length = 255]
        site -> Varchar,
        decay -> Date,
        period -> Float8,
        inclination -> Float8,
        apogee -> Float8,
        perigee -> Float8,
        #[max_length = 255]
        comment -> Varchar,
        #[max_length = 255]
        commentcode -> Varchar,
        rcsvalue -> Float8,
        #[max_length = 255]
        rcs_size -> Varchar,
        file -> Int4,
        launch_year -> Int4,
        launch_num -> Int4,
        #[max_length = 255]
        launch_piece -> Varchar,
        current -> Bool,
        #[max_length = 255]
        object_name -> Varchar,
        #[max_length = 255]
        object_id -> Varchar,
    }
}

diesel::table! {
    tle_object (norad_cat_id) {
        norad_cat_id -> Int4,
        ordinal -> Int4,
        #[max_length = 255]
        comment -> Varchar,
        #[max_length = 255]
        originator -> Varchar,
        #[max_length = 255]
        object_name -> Varchar,
        #[max_length = 255]
        object_type -> Varchar,
        #[max_length = 255]
        classification_type -> Varchar,
        #[max_length = 255]
        intldes -> Varchar,
        epoch -> Timestamp,
        epoch_microseconds -> Int4,
        mean_motion -> Float8,
        eccentricity -> Float8,
        inclination -> Float8,
        ra_of_asc_node -> Float8,
        arg_of_pericenter -> Float8,
        mean_anomaly -> Float8,
        ephemeris_type -> Int4,
        element_set_no -> Int4,
        rev_at_epoch -> Int4,
        bstar -> Float8,
        mean_motion_dot -> Float8,
        mean_motion_ddot -> Float8,
        file -> Int4,
        #[max_length = 255]
        tle_line0 -> Varchar,
        #[max_length = 255]
        tle_line1 -> Varchar,
        #[max_length = 255]
        tle_line2 -> Varchar,
        #[max_length = 255]
        object_id -> Varchar,
        object_number -> Int4,
        semimajor_axis -> Float8,
        period -> Float8,
        apogee -> Float8,
        perigee -> Float8,
        decayed -> Bool,
    }
}

diesel::joinable!(sat_cat_object -> tle_object (norad_cat_id));

diesel::allow_tables_to_appear_in_same_query!(sat_cat_object, tle_object,);
