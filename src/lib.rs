use bevy::prelude::*;

#[derive(Component)]
pub struct Movable;

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::Up) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::Down) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
        transform.looking_at(Vec3::ZERO, Vec3::Y);
    }
}