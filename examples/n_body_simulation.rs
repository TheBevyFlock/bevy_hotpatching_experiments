use bevy::prelude::*;
use bevy_hotpatching_experiments::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            SimpleSubsecondPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_planets)
        .run()
}

#[derive(Component, Default)]
struct Planet {
    velocity: Vec3,
}

#[derive(Component)]
struct Attractor;

#[hot]
fn move_planets(
    mut planets: Query<(&mut Transform, &mut Planet), Without<Attractor>>,
    attractors: Query<&Transform, With<Attractor>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut planet_transform, mut planet) in &mut planets {
        let mut acceleration = Vec3::ZERO;

        for attractor in &attractors {
            let dir = attractor.translation - planet_transform.translation;
            let dist_sq = dir.length_squared().max(25.0); // Avoid divide by zero
            let force_dir = dir.normalize();
            // Try tweaking this value at runtime!
            // Try e.g. 1000_000 or 100 and see what happens :)
            let g = 10_000.0; // Gravitational constant
            acceleration += force_dir * g / dist_sq;
        }

        planet.velocity += acceleration * dt;
        planet_transform.translation += planet.velocity * dt;
    }
}

#[derive(Resource)]
struct GameAssets {
    planet_mesh: Handle<Mesh>,
    planet_material: Handle<ColorMaterial>,
    attractor_mesh: Handle<Mesh>,
    attractor_material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct Setup;

#[hot(rerun_on_hot_patch = true)]
fn setup(
    previous_setup: Query<Entity, With<Setup>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Clear all entities that were spawned on `Startup` so that
    // hot-patching does not spawn them again
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn a canvas to register our clicks
    commands
        .spawn((
            Setup,
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.0)))),
        ))
        .observe(react_to_click);

    // Spawn a camera to view the scene
    commands.spawn((Setup, Camera2d));

    // Define our assets.
    // Try changing these values at runtime and spawning new planets and attractors!
    commands.insert_resource(GameAssets {
        planet_mesh: meshes.add(Circle::new(3.0)),
        planet_material: materials.add(Color::WHITE),
        attractor_mesh: meshes.add(Circle::new(10.0)),
        attractor_material: materials.add(Color::BLACK),
    });

    // Spawn a text nodes to display instructions
    // Try changing the text at runtime!
    commands.spawn((
        Setup,
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            Text::new("Instructions:"),
            Text::new("Click to spawn a planet"),
            Text::new("Right click to spawn an attractor"),
            Text::new("Middle click to clear all bodies"),
        ],
    ));
}

fn react_to_click(
    trigger: On<Pointer<Click>>,
    planet_assets: Res<GameAssets>,
    mut commands: Commands,
    bodies: Query<Entity, Or<(With<Planet>, With<Attractor>)>>,
) {
    let Some(location) = trigger.event().event.hit.position else {
        return;
    };
    let transform = Transform::from_xyz(location.x, location.y, 0.0);

    match trigger.event().event.button {
        // Spawn a planet when the left mouse button is clicked
        PointerButton::Primary => {
            commands.spawn((
                transform,
                Planet::default(),
                Mesh2d(planet_assets.planet_mesh.clone()),
                MeshMaterial2d(planet_assets.planet_material.clone()),
            ));
        }
        // Spawn an attractor when the right mouse button is clicked
        PointerButton::Secondary => {
            commands.spawn((
                transform,
                Attractor,
                Mesh2d(planet_assets.attractor_mesh.clone()),
                MeshMaterial2d(planet_assets.attractor_material.clone()),
            ));
        }
        // Clear all bodies when the middle mouse button is clicked
        PointerButton::Middle => {
            for entity in &bodies {
                commands.entity(entity).despawn();
            }
        }
    };
}
