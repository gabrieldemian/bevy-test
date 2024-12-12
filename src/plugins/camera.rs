use bevy::prelude::*;

use super::app_state::AppState;

#[derive(Component)]
pub struct MyCamera;

pub struct CameraPlugin;

fn move_camera(
    mut q: Query<&mut Transform, With<MyCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = q.single_mut();

    // rotation
    if keyboard.just_pressed(KeyCode::KeyZ) {
        transform.rotate_y(-std::f32::consts::FRAC_PI_2 / 25.0);
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        transform.rotate_y(std::f32::consts::FRAC_PI_2 / 25.0);
    }
    if keyboard.just_pressed(KeyCode::KeyQ) {
        transform.rotate_x(std::f32::consts::FRAC_PI_2 / 25.0);
    }
    if keyboard.just_pressed(KeyCode::KeyE) {
        transform.rotate_x(-std::f32::consts::FRAC_PI_2 / 25.0);
    }

    // up Y
    if keyboard.just_pressed(KeyCode::Space) {
        transform.translation.y += 1.0 + time.delta_secs();
    }

    // down Y
    if keyboard.just_pressed(KeyCode::KeyC) {
        transform.translation.y -= 1.0 + time.delta_secs();
    }

    // translation
    if keyboard.just_pressed(KeyCode::KeyW) {
        transform.translation.z -= 1.0 + time.delta_secs();
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        transform.translation.z += 1.0 + time.delta_secs();
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        transform.translation.x -= 1.0 + time.delta_secs();
    }

    if keyboard.just_pressed(KeyCode::KeyD) {
        transform.translation.x += 1.0 + time.delta_secs();
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        // make camera look at the center
        Transform::from_xyz(0.0, 0.0, 5.0),
        MyCamera,
    ));
}

impl Plugin for CameraPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Update, move_camera);
        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            move_camera.run_if(in_state(AppState::InGame)),
        );
    }
}
