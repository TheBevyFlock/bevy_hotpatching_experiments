use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_hotpatching_experiments::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        // try adding and removing systems from here! make whole new ones!
        .with_hot_patch(|app: &mut App| {
            // StartupRerunHotPatch is like Startup, but will rerun on hot-reload.
            // You need the #[hot(hot_patch_signature = true)] macro to auto-despawn entities spawned in it!
            app.add_systems(StartupRerunHotPatch, spawn_ui);
            // All other systems do not require `#[hot]`.
            // Try writing, adding, and removing new ones here at runtime!
            app.add_systems(Update, print_hello);
            app.add_systems(
                Update,
                change_text.run_if(input_just_pressed(KeyCode::Space)),
            );
        })
        .run()
}

#[hot(hot_patch_signature = true)]
fn spawn_ui(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        // Currently bevy forgets to do `track_caller` on `commands.spawn` so to
        // auto-despawn entities spawned inside a StartupRerunHotPatch schedule
        // we need to call spawn on `world` instead.
        world.spawn((
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
                Text::new("Press space to change the text below:"),
                (
                    Text::new("(no button pressed yet, or this system was reset)"),
                    InfoLabel
                ),
            ],
        ));
        world.spawn(Camera2d);
    });
}

fn print_hello() {
    info_once!("Hello, world!");
}

#[derive(Component)]
struct InfoLabel;

fn change_text(mut query: Query<&mut Text, With<InfoLabel>>, time: Res<Time>) {
    for mut text in &mut query {
        text.0 = format!("You pressed the space key at t = {} s", time.elapsed_secs());
    }
}
