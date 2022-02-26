use bevy::prelude::*;
use snake_plugin::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(SnakePlugin)
        .run();
}
