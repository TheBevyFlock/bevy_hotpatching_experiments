//! Enabled component migration when hot patching happens.
//!
//! Implement [`Reflect`], [`HotPatchMigrate`], [`Default`] and [`Component`]
//! and reflect them for the component you want to migrate.
//! ```
//! # use bevy_hotpatching_experiments::prelude::*;
//! # use bevy::prelude::*;
//! #[derive(Debug, Reflect, Component, Default, HotPatchMigrate)]
//! #[reflect(Component, Default, HotPatchMigrate)]
//! struct Example {
//!     field: usize,
//! }
//! ```
//!
//! Additionally you will need to register these components and their
//! new, hot patched versions. This can be done in a startup system like this:
//! ```
//! # use bevy_hotpatching_experiments::prelude::*;
//! # use bevy::prelude::*;
//! // When creating the app:
//! // app.add_systems(Startup, register_components)
//!
//! # #[derive(Debug, Reflect, Component, Default, HotPatchMigrate)]
//! # #[reflect(Component, Default, HotPatchMigrate)]
//! # struct Example { }
//!
//! #[hot(rerun_on_hot_patch = true)]
//! fn register_components(registry: Res<AppTypeRegistry>) {
//!     let mut registry = registry.write();
//!     registry.register::<Example>();
//!     // ...register other components
//! }
//! ```

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::QueryBuilder,
    reflect::{AppTypeRegistry, ReflectComponent},
    resource::Resource,
    system::{Res, ResMut},
    world::World,
};
use bevy_log::warn;
use bevy_platform::sync::Arc;
use bevy_reflect::{FromType, Reflect, std_traits::ReflectDefault};
use bevy_utils::TypeIdMap;
use core::any::{Any, TypeId};

/// Enables migration for your components. Should be derived and
/// not implemented manually.
///
/// Requires that the type also implementes `Any`, `Reflect`,
/// `Component` and `Default`. Last two (and `HotPatchMigrate`)
/// should be reflected.
///
/// ```
/// # use bevy_hotpatching_experiments::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Debug, Reflect, Component, Default, HotPatchMigrate)]
/// #[reflect(Component, Default, HotPatchMigrate)]
/// struct Example {
///     field: usize,
/// }
/// ```
///
/// Supports renaming the struct and field addition/removal.
pub trait HotPatchMigrate: Any + Component + Reflect + Default {
    /// The default derive implementation is a hot patchable function
    /// which means that this will always return the current type id
    /// (if a struct is changed it will start returning a new type id).
    fn current_type_id() -> TypeId;
}

/// [`bevy_reflect::TypeData`] corresponding to the [`HotPatchMigrate`] trait. It contains the
/// [`HotPatchMigrate::current_type_id`] method. You don't need to use this
/// directly for hot patching or struct migration.
#[derive(Clone)]
pub struct ReflectHotPatchMigrate(pub Arc<dyn Fn() -> TypeId + Sync + Send + 'static>);

impl<T: HotPatchMigrate> FromType<T> for ReflectHotPatchMigrate {
    fn from_type() -> Self {
        Self(Arc::new(T::current_type_id))
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct ComponentMigrations(TypeIdMap<Arc<dyn Fn() -> TypeId + Sync + Send + 'static>>);

pub(crate) fn register_migratable_components(
    mut migrations: ResMut<ComponentMigrations>,
    registry: Res<AppTypeRegistry>,
) {
    for registration in registry.read().iter() {
        let Some(current_type_id) = registration.data::<ReflectHotPatchMigrate>() else {
            continue;
        };

        migrations
            .entry(registration.type_id())
            .or_insert_with(|| current_type_id.clone().0);
    }
}

pub(crate) fn migrate(world: &mut World) {
    let migrations = world.resource::<ComponentMigrations>();
    let changed: Vec<_> = migrations
        .iter()
        .filter(|(prev, current)| prev != &&current())
        .map(|(prev, current)| (*prev, current.clone()))
        .collect();

    for (prev, current) in &changed {
        migrate_component(world, *prev, current());
    }

    // Track hot patches to the new struct
    let mut migrations = world.resource_mut::<ComponentMigrations>();
    migrations.extend(changed.into_iter().map(|(_, current)| (current(), current)));
}

fn migrate_component(world: &mut World, prev: TypeId, to: TypeId) {
    world.resource_scope::<AppTypeRegistry, ()>(|world, registry| {
        let registry = registry.read();
        let Some(from_component_id) = world.components().get_id(prev) else {
            // If there is no ComponentId, it doesn't exist in bevy's storages so there is nothing to migrate
            return;
        };

        let name = world
            .components()
            .get_name(from_component_id)
            .unwrap_or_else(|| "Unknown".into());
        let Some(prev_reflect_component) = registry.get_type_data::<ReflectComponent>(prev) else {
            warn!("Component '{name}' needs to `#[reflect(Component)]`");
            return;
        };
        let Some(reflect_default) = registry.get_type_data::<ReflectDefault>(to) else {
            warn!("Component '{name}' needs to `#[reflect(Default)]`");
            return;
        };
        let Some(reflect_component) = registry.get_type_data::<ReflectComponent>(to) else {
            warn!("Component '{name}' needs to `#[reflect(Component)]`");
            return;
        };

        // Migrate each entity that contains a component matching `from` type id
        let mut builder = QueryBuilder::<Entity>::new(world);
        builder.with_id(from_component_id);
        let mut query = builder.build();

        let entities: Vec<_> = query.iter(world).collect();

        for entity in entities {
            let entity_mut = world.entity_mut(entity);
            let Some(prev_value) = prev_reflect_component.reflect(&entity_mut) else {
                let name = world
                    .components()
                    .get_name(from_component_id)
                    .unwrap_or_else(|| "Unknown".into());
                warn!("Tried to migrate entity {entity} but it didn't contain component '{name}'");
                continue;
            };

            let mut value = reflect_default.default();
            if let Err(err) = value.try_apply(prev_value) {
                let name = world
                    .components()
                    .get_name(from_component_id)
                    .unwrap_or_else(|| "Unknown".into());
                warn!("Tried to migrate component '{name}' on entity {entity} but operation wasn't supported: {err}. New component will contain default values.");
            }

            let mut entity_mut = world.entity_mut(entity);
            prev_reflect_component.remove(&mut entity_mut);
            reflect_component.insert(&mut entity_mut, value.as_partial_reflect(), &registry);
        }
    });
}
