use bevy::prelude::*;
use geirtris::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}
