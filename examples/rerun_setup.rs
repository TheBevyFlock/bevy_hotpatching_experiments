use bevy::prelude::*;
use bevy_hotpatching_experiments::prelude::*;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Startup, setup)
        .run()
}

#[derive(Component)]
struct Setup;

#[hot(rerun_on_hot_patch = true)]
fn setup(previous_setup: Query<Entity, With<Setup>>, mut commands: Commands) {
    // Clear all entities that were spawned on `Startup` so that
    // hot-patching does not spawn them again
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        Setup,
        Node {
            // You can change the `Node` however you want at runtime
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        children![
            Text::new("Hello, world!"),
            Text::new("Try adding new texts below"),
        ],
    ));
    commands.spawn((Setup, Camera2d));

    commands.insert_resource(UiDebugOptions {
        // Set this to `true` to see the UI debug overlay. Try changing it at runtime!
        enabled: false,
        ..default()
    });
}
