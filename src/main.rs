use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Number Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
 