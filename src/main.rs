//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng; // Add the rand crate for generating random numbers

const BOID_SIZE: f32 = 30.;
const NUM_BIRDS: usize = 10;
const SPAWN_RADIUS: f32 = 250.;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(WindowPlugin:: { primary_window: Some(Window {})}))
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

    // Boid
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
    //         transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
    //         ..default()
    //     },
    //     Name::new("Boid"),
    // ));
}

fn spawn_boid(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
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
        Name::new("Boid"),
    ));
}
