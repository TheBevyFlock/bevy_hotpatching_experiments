use bevy::prelude::*;
use bevy_hotpatching_experiments::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, register_components)
        .add_systems(Update, print_components)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Player {
        name: "Killgore".to_string(),
        health: 100.0,
        ..default()
    });

    commands.spawn(Camera2d);
    commands.spawn(Text::default());
}

// This is #[hot] because new versions of hot patched components need
// to be registered so their reflected type data are available

#[hot(rerun_on_hot_patch = true)]
fn register_components(registry: Res<AppTypeRegistry>) {
    let mut registry = registry.write();
    registry.register::<Player>();
}

// Try changing the component below at runtime:
// - Rename them
// - Add a field
// - Remove a field

#[derive(Debug, Reflect, Component, Default, HotPatchMigrate)]
#[reflect(Component, Default, HotPatchMigrate)]
struct Player {
    name: String,
    health: f32,
    mana: f32,
}

#[hot(hot_patch_signature = true)]
fn print_components(player: Single<&Player>, mut text: Single<&mut Text>) {
    text.0 = format!("Player: {:#?}", player.into_inner());
}
