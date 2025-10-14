#![allow(unused_mut, unused_variables)]
use bevy::{
    ecs::{
        message::MessageCursor,
        schedule::ScheduleConfigs,
        system::{ScheduleSystem, SystemParam},
    },
    prelude::*,
};
use bevy_hotpatching_experiments::prelude::*;

#[test]
fn add_to_app() {
    App::new().add_systems(
        Update,
        (
            (
                empty_system,
                system_with_commands,
                system_with_commands_mut,
                system_with_zst_query,
                system_with_readonly_query,
                system_with_mut_query,
                system_with_mixed_query,
                system_with_single_query,
                system_with_resource,
                system_with_resource_and_query,
                system_with_mut_resource,
                system_with_mut_resource_and_query,
                system_with_mut_resource_and_mut_query,
                system_with_mut_resource_and_single_query,
                system_with_mut_resource_and_mut_single_query,
                system_with_mut_resource_and_mut_single_query_rerun_true,
                system_with_mut_resource_and_mut_single_query_rerun_false,
            ),
            (
                system_with_return_value,
                system_with_aliased_return,
                system_with_generic::<Transform>,
                system_with_generic_and_exclusive::<Transform>,
                system_with_generic_static_and_exclusive::<Transform>,
                system_with_generic_non_static_and_exclusive::<Transform>,
                system_with_generic_and_exclusive_mut::<Transform>,
                save_to_previous::<Transform>,
                apply_config::<DevConfig>,
                exclusive_mut,
                exclusive,
                force_loading_screen.pipe(ignore_progress),
                wait_in_screen(1.0),
            ),
        ),
    );
}

#[hot(rerun_on_hot_patch = true)]
fn empty_system() {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_commands(commands: Commands) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_commands_mut(mut commands: Commands) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_zst_query(query: Query<()>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_readonly_query(query: Query<&Transform>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_query(mut query: Query<&mut Transform>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mixed_query(query: Query<&Transform>, mut mut_query: Query<&mut Node>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_single_query(query: Single<Entity, With<Transform>>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_resource(resource: Res<Time>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_resource_and_query(resource: Res<Time>, query: Query<&Transform>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource(mut resource: ResMut<Time>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource_and_query(mut resource: ResMut<Time>, query: Query<&Transform>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource_and_mut_query(
    mut resource: ResMut<Time>,
    mut query: Query<&mut Transform>,
) {
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource_and_single_query(
    mut resource: ResMut<Time>,
    query: Single<Entity, With<Transform>>,
) {
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource_and_mut_single_query(
    mut resource: ResMut<Time>,
    mut query: Single<&mut Transform, With<Transform>>,
) {
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_mut_resource_and_mut_single_query_rerun_true(
    mut resource: ResMut<Time>,
    mut query: Single<&mut Transform, With<Transform>>,
) {
}

#[hot(rerun_on_hot_patch = false)]
fn system_with_mut_resource_and_mut_single_query_rerun_false(
    mut resource: ResMut<Time>,
    mut query: Single<&mut Transform, With<Transform>>,
) {
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_return_value() -> Result<(), BevyError> {
    Ok(())
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_aliased_return() -> Result {
    Ok(())
}

#[hot(rerun_on_hot_patch = true)]
fn system_with_generic<T: Component>(query: Query<&T>) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_generic_and_exclusive<T: Component>(world: &World) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_generic_static_and_exclusive<T: 'static>(world: &mut World) {}

//#[hot(rerun_on_hot_patch = true)]
fn system_with_generic_non_static_and_exclusive<T>(world: &mut World) {}

#[hot(rerun_on_hot_patch = true)]
fn system_with_generic_and_exclusive_mut<T: Component>(world: &mut World) {}

#[derive(Component)]
struct Previous<T: Component + Clone>(T);

#[hot(rerun_on_hot_patch = true)]
fn save_to_previous<C: Component + Clone>(
    mut previous_query: Query<(&mut Previous<C>, &C), Changed<C>>,
) {
}

//#[hot(rerun_on_hot_patch = true)]
fn apply_config<C: Config>(world: &mut World, mut cursor: Local<MessageCursor<AssetEvent<C>>>) {}

#[hot(rerun_on_hot_patch = true)]
fn exclusive_mut(world: &mut World) {}

#[hot(rerun_on_hot_patch = true)]
fn exclusive(world: &World) {}

//#[hot(rerun_on_hot_patch = true)]
fn force_loading_screen(config: ConfigRef<DevConfig>, screen: CurrentRef<Screen>) -> Progress {
    todo!()
}

//#[hot(rerun_on_hot_patch = true)]
fn wait_in_screen(duration: f32) -> ScheduleConfigs<ScheduleSystem> {
    (move |screen_time: Res<Time>| ()).into_configs()
}

fn ignore_progress(_: In<Progress>) {}

pub trait Config: Asset {}
pub trait State: Resource + Sized {}
#[derive(SystemParam)]
pub struct CurrentRef<'w, S: State>(pub Option<Res<'w, S>>);

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Resource)]
pub enum Screen {}
impl State for Screen {}

#[derive(SystemParam)]
pub struct ConfigRef<'w, C: Config> {
    pub assets: Res<'w, Assets<C>>,
}
#[derive(Asset, Reflect)]
struct DevConfig {
    pub progress: f32,
}
impl Config for DevConfig {}

pub struct Progress {
    pub done: u32,
    pub total: u32,
}
