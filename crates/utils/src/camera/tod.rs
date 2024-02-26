use bevy::prelude::*;

pub fn move_camera(
    mut players: Query<&mut Transform, With<Camera2d>>,
    input: Res<Input<KeyCode>>,
) {
    let movement_direction = if input.pressed(KeyCode::W) {
        (0, 1)
    } else if input.pressed(KeyCode::A) {
        (-1, 0)
    } else if input.pressed(KeyCode::S) {
        (0, -1)
    } else if input.pressed(KeyCode::D) {
        (1, 0)
    } else {
        return;
    };

    for mut transform in players.iter_mut() {
        transform.translation.x += movement_direction.0 as f32 * 3.0;
        transform.translation.y += movement_direction.1 as f32 * 3.0;
    }

}


pub fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 1.3;
    commands.spawn(camera);
}

