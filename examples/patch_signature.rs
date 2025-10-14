use bevy::prelude::*;
use bevy_hotpatching_experiments::prelude::*;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Update, greet)
        .run()
}

// You can change the signature of this system at runtime.
// Try running the app and then uncommenting the lines below!
#[hot(hot_patch_signature = true)]
fn greet(/* time: Res<Time> */) {
    info_once!(
        "Hello from a hotpatched system! Try changing this string while the app is running!"
    );
    // info!("Time: {}", time.elapsed_secs());
}
