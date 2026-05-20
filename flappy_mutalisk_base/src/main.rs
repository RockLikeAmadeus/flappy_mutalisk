use bevy::{app::AppExit, prelude::*};
use my_library::{self, RandomPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Mutalisk".into(),
                resolution: (1024, 768).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RandomPlugin)
        .run();
}

