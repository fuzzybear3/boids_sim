//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    gizmos, math::vec2, prelude::*, sprite::MaterialMesh2dBundle, transform,
    utils::petgraph::adj::Neighbors,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng; // Add the rand crate for generating random numbers
use std::f32::consts::PI;

mod components;
use components::*;

const NUM_BIRDS: usize = 2000;
const BOID_SIZE: f32 = 30.;
const SPAWN_RADIUS: f32 = 100.;
const NEIGHBOR_RADIUS: f32 = 100.;
const MAP_RADIUS: f32 = 1000.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                position: WindowPosition::Centered(bevy::window::MonitorSelection::Index(1)),
                // resolution: (500., 300.).into(),
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (boid_movement, boid_heading_calculator))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // open on second monitor

    // Camera
    commands.spawn((Camera2dBundle::default(), Name::new("Camera_2d")));

    let mut rng = rand::thread_rng();

    for i in 0..NUM_BIRDS {
        let x = rng.gen_range(-SPAWN_RADIUS..SPAWN_RADIUS);
        let y = rng.gen_range(-SPAWN_RADIUS..SPAWN_RADIUS);

        let transform = Transform::from_translation(Vec3::new(x, y, 0.));
        spawn_boid(&mut commands, &mut meshes, &mut materials, transform);
    }
}

fn spawn_boid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(BOID_SIZE, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform,
            ..default()
        },
        Boid::default(),
        Name::new("Boid"),
    ));
}

fn find_nearby_boids(boid_list: Vec<Vec3>, position: Vec3) -> Vec<Vec3> {
    let mut list_of_boids_neighbors: Vec<Vec3> = Vec::new();
    for neighbor_pos in boid_list.iter() {
        // for each boid, find the boids within a certain radius
        let distance = neighbor_pos.distance(position);
        // check for zero distance to avoid counting self.
        if distance < NEIGHBOR_RADIUS && distance > 0. {
            let relitive_position = *neighbor_pos - position;
            list_of_boids_neighbors.push(relitive_position);
        }
    }
    return list_of_boids_neighbors;
}

fn separation_calculator(list_of_boids_positions: &Vec<Vec3>) -> Vec3 {
    let mut separation_vector = Vec3::new(0., 0., 0.);

    for boid_position in list_of_boids_positions.iter() {
        let distance = boid_position.distance(*boid_position);
        if distance < NEIGHBOR_RADIUS {
            separation_vector -= *boid_position;
        }
    }
    if list_of_boids_positions.len() > 0 {
        return separation_vector;
    }
    separation_vector / list_of_boids_positions.len() as f32
}

fn alignment_calculator(list_of_boids_headings: &Vec<Vec3>) -> Vec3 {
    let mut alignment_vector = Vec3::new(0., 0., 0.);

    for boid_heading in list_of_boids_headings.iter() {
        alignment_vector += boid_heading.normalize();
    }

    alignment_vector
}

fn cohesion_calculator(list_of_boids_positions: &Vec<Vec3>) -> Vec3 {
    let mut cohesion_vector = Vec3::new(0., 0., 0.);

    for boid_position in list_of_boids_positions.iter() {
        let distance = boid_position.distance(*boid_position);
        if distance < NEIGHBOR_RADIUS {
            cohesion_vector += *boid_position;
        }
    }
    // if list_of_boids_positions.len() > 0 {
    //     return alignment_vector ;
    // }
    cohesion_vector / list_of_boids_positions.len() as f32
}

fn pointer(boid_position: Vec3, control_heading: Vec3, color: Color, gizmos: &mut Gizmos) {
    let heading_pointer = control_heading.xy().normalize() * 50. + boid_position.xy();
    gizmos.line_2d(
        boid_position.xy(),
        // Vec2::ZERO,
        heading_pointer,
        color,
    );
}

// HEADING CALCULATOR //
fn boid_heading_calculator(mut query: Query<(&mut Boid, &Transform)>, mut gizmos: Gizmos) {
    let mut list_of_all_boids_pos: Vec<Vec3> = Vec::new();
    for (boid, transform) in query.iter_mut() {
        list_of_all_boids_pos.push(transform.translation);
    }
    let mut list_of_boids_headings: Vec<Vec3> = Vec::new();
    for (boid, transform) in query.iter_mut() {
        list_of_boids_headings.push(boid.heading);
    }

    for (mut boid, transform) in query.iter_mut() {
        let list_of_boids_neighbors =
            find_nearby_boids(list_of_all_boids_pos.clone(), transform.translation);
        if list_of_boids_neighbors.len() > 0 {
            let separation_vector = separation_calculator(&list_of_boids_neighbors);

            pointer(boid.position, boid.heading, Color::RED, &mut gizmos);
            let alignment_vector = alignment_calculator(&list_of_boids_headings);
            pointer(boid.position, alignment_vector, Color::GREEN, &mut gizmos);
            let cohesion_vector = cohesion_calculator(&list_of_boids_neighbors);
            pointer(boid.position, cohesion_vector, Color::BEIGE, &mut gizmos);

            boid.heading = separation_vector + alignment_vector + cohesion_vector;
        }
        if transform.translation.distance(Vec3::ZERO) > MAP_RADIUS {
            boid.heading = -transform.translation.normalize();
        }
    }
}

fn boid_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Boid)>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut boid) in query.iter_mut() {
        let velocity = boid.speed * boid.heading.normalize();
        let transform_addition = Vec3::new(velocity.x, velocity.y, 0.) * time.delta_seconds();
        transform.translation += transform_addition;

        boid.position = transform.translation;
        gizmos.circle_2d(boid.position.xy(), NEIGHBOR_RADIUS, Color::BLUE);
    }

    // // rotate the boid to face the direction it's moving
    // for (mut transform, boid) in query.iter_mut() {
    //     transform.rotation = Quat::from_rotation_z(-boid.heading.y.atan2(boid.heading.x));
    // }
    //

    // draw bounding box
    gizmos.circle_2d(Vec2::ZERO, MAP_RADIUS, Color::BLACK);
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_find_nearby_boids() {
        let boid_list = vec![Vec3::new(0., 0., 0.), Vec3::new(1., 1., 0.)];
        let position = Vec3::new(0., 0., 0.);
        let result = find_nearby_boids(boid_list, position);
        assert_eq!(result, vec![Vec3::new(1., 1., 0.)]);
    }

    #[test]
    fn test_separation_calculator() {
        let list_of_boids_positions = vec![Vec3::new(0., 1., 0.), Vec3::new(1., 0., 0.)];
        let result = separation_calculator(&list_of_boids_positions);
        assert_eq!(result, Vec3::new(-1., -1., 0.));
    }

    #[test]
    fn test_cohesion_calculator() {
        let list_of_boids_positions = vec![Vec3::new(0., 1., 0.), Vec3::new(1., 0., 0.)];
        let result = cohesion_calculator(&list_of_boids_positions);
        assert_eq!(result, Vec3::new(1., 1., 0.));
    }
}
