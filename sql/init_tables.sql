DROP TABLE IF EXISTS tle_object;

CREATE TABLE tle_object (
    norad_cat_id            INTEGER PRIMARY KEY,
    ordinal                 INTEGER NOT NULL,
    comment                 VARCHAR(255) NOT NULL,
    originator              VARCHAR(255) NOT NULL,
    object_name             VARCHAR(255) NOT NULL,
    object_type             VARCHAR(255) NOT NULL,
    classification_type     VARCHAR(255) NOT NULL,
    intldes                 VARCHAR(255) NOT NULL,
    epoch                   TIMESTAMP NOT NULL,
    epoch_microseconds      INTEGER NOT NULL,
    mean_motion             DOUBLE PRECISION NOT NULL,
    eccentricity            DOUBLE PRECISION NOT NULL,
    inclination             DOUBLE PRECISION NOT NULL,
    ra_of_asc_node          DOUBLE PRECISION NOT NULL,
    arg_of_pericenter       DOUBLE PRECISION NOT NULL,
    mean_anomaly            DOUBLE PRECISION NOT NULL,
    ephemeris_type          INTEGER NOT NULL,
    element_set_no          INTEGER NOT NULL,
    rev_at_epoch            INTEGER NOT NULL,
    bstar                   DOUBLE PRECISION NOT NULL,
    mean_motion_dot         DOUBLE PRECISION NOT NULL,
    mean_motion_ddot        DOUBLE PRECISION NOT NULL,
    file                    INTEGER NOT NULL,
    tle_line0               VARCHAR(255) NOT NULL,
    tle_line1               VARCHAR(255) NOT NULL,
    tle_line2               VARCHAR(255) NOT NULL,
    object_id               VARCHAR(255) NOT NULL,
    object_number           INTEGER NOT NULL,
    semimajor_axis          DOUBLE PRECISION NOT NULL,
    period                  DOUBLE PRECISION NOT NULL,
    apogee                  DOUBLE PRECISION NOT NULL,
    perigee                 DOUBLE PRECISION NOT NULL,
    decayed                 BOOLEAN NOT NULL
);

DROP TABLE IF EXISTS sat_cat_object;

CREATE TABLE sat_cat_object (
    object_number        INTEGER PRIMARY KEY,
    intldes              VARCHAR(255) NOT NULL,
    norad_cat_id         INTEGER NOT NULL,
    object_type          VARCHAR(255) NOT NULL,
    satname              VARCHAR(255) NOT NULL,
    country              VARCHAR(255) NOT NULL,
    launch               DATE NOT NULL,
    site                 VARCHAR(255) NOT NULL,
    decay                DATE NOT NULL,
    period               DOUBLE PRECISION NOT NULL,
    inclination          DOUBLE PRECISION NOT NULL,
    apogee               DOUBLE PRECISION NOT NULL,
    perigee              DOUBLE PRECISION NOT NULL,
    comment              VARCHAR(255) NOT NULL,
    commentcode          VARCHAR(255) NOT NULL,
    rcsvalue             DOUBLE PRECISION NOT NULL,
    rcs_size             VARCHAR(255) NOT NULL,
    file                 INTEGER NOT NULL,
    launch_year          INTEGER NOT NULL,
    launch_num           INTEGER NOT NULL,
    launch_piece         VARCHAR(255) NOT NULL,
    current              BOOLEAN NOT NULL,
    object_name          VARCHAR(255) NOT NULL,
    object_id            VARCHAR(255) NOT NULL,

    FOREIGN KEY (norad_cat_id)
        REFERENCES tle_object(norad_cat_id)
        ON UPDATE CASCADE ON DELETE CASCADE
);
