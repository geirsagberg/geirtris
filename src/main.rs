use bevy::prelude::*;
use geirtris::MainPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        monitor: MonitorSelection::Index(1),
                        width: 160.,
                        height: 320.,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(MainPlugin)
        .run();
}
