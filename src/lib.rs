mod game;
mod macros;
mod menu;

use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;

pub struct MainPlugin;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Running,
    GameOver,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_state::<GameState>()
            .add_plugin(MenuPlugin)
            .add_plugin(GamePlugin);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    Pause,
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
