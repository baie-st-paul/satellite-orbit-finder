use bevy::prelude::*;

use crate::interface::SecToSim;
pub fn setup(mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>) {
    // a derbis in burn zone
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(8).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.2, 0.0), // just inside burn zone
        Debris {
            mass: 1000.0,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        },
    ));

    // a derbis with unstable orbit
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(8).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.5, 0.0),
        Debris {
            mass: 1000.0,
            velocity: Vec3::new(0.0, 0.0, 4315.158),
            acceleration: Vec3::ZERO,
        },
    ));

    // a derbis in orbit
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(8).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.7, 0.0),
        Debris {
            mass: 1000.0,
            velocity: Vec3::new(0.0, 0.0,  3574.2028),
            acceleration: Vec3::ZERO,
        },
    ));
}

#[derive(Component)]
pub struct Debris {
    mass: f32,
    velocity: Vec3,
    acceleration: Vec3,
}

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


// Calculate forces to update acceleration of debris based on position
pub fn update_forces(mut query: Query <(&mut Transform, &mut Debris)>) {
 //gravity
 for (transform, mut debris) in &mut query {
    let r = 6371000.0 * (transform.translation.length_squared());   //in meters

    let ga = super::planet::GM / (r * r); //acceleration from gravity
    debris.acceleration = -transform.translation.normalize_or_zero() * ga; //apply acceleration
 }

 //other? resistance?
}

// Update motion of debris based on forces
pub fn update_motion(mut query: Query <(&mut Transform, &mut Debris)>, time: Res<Time>, multiplier: Res<SecToSim>) {
    let dt = time.delta_secs() * multiplier.0;
    for (mut transform, mut debris) in &mut query {
        let a = debris.acceleration;
        debris.velocity += a * dt;
        transform.translation += debris.velocity * dt / 6371000.0; //scale back to simulation unit
    }
}