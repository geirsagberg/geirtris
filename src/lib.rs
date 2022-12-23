mod game;
mod macros;
mod menu;

use bevy::prelude::*;
use game::GamePlugin;
use iyes_loopless::prelude::AppLooplessStateExt;
use menu::MenuPlugin;

pub struct MainPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    Running,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::MainMenu)
            .add_startup_system(setup_camera)
            .add_plugin(MenuPlugin)
            .add_plugin(GamePlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}