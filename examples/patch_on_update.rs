use bevy::prelude::*;
use bevy_hotpatching_experiments::prelude::*;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, configure_ui)
        .run()
}

#[derive(Component)]
#[require(Node)]
struct Ui;

fn setup(mut commands: Commands) {
    commands.spawn(Ui);
    commands.spawn(Camera2d);
}

#[hot]
fn configure_ui(ui: Single<Entity, With<Ui>>, mut commands: Commands) {
    commands.entity(*ui).despawn_related::<Children>().insert((
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
}
