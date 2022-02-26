use bevy::prelude::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

/// Create 2d camera, initial wall and snake.
fn setup(mut commands: Commands) {
    // add 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // add snake
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
