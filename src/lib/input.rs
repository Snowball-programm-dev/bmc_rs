use bevy::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Component)]
pub struct Movable;

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        let rotate = Vec4::from_array(transform.rotation.to_array());
        if input.pressed(KeyCode::Up) {
            direction.z -= 1.0;
            //rotate.x += 0.01;
        }
        if input.pressed(KeyCode::Down) {
            direction.z += 1.0;
            //rotate.x -= 0.01;            
        }
        if input.pressed(KeyCode::Left) {
            //rotate.w -= 0.01;
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
            //rotate.w += 0.01;
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
        transform.rotation = Quat::from_xyzw(rotate.x, rotate.y, rotate.z, rotate.w);
        
    }
}

pub fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.primary_mut();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_grab_mode(CursorGrabMode::None);
    }
}