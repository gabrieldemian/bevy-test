use bevy::{
    input::mouse::MouseMotion, pbr::CascadeShadowConfigBuilder, prelude::*,
};
use std::f32::consts::PI;

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

    let rt = std::f32::consts::FRAC_PI_8 / 5.0;
    let mv = 0.2;

    // rotate left
    if keyboard.just_pressed(KeyCode::KeyZ) {
        transform.rotate_y(rt);
    }

    // rotate right
    if keyboard.just_pressed(KeyCode::KeyX) {
        transform.rotate_y(-rt);
    }

    // rotate up
    if keyboard.just_pressed(KeyCode::KeyQ) {
        transform.rotate_x(rt);
    }

    // rotate down
    if keyboard.just_pressed(KeyCode::KeyE) {
        transform.rotate_x(-rt);
    }

    // move up
    if keyboard.just_pressed(KeyCode::Space) {
        transform.translation.y += mv + time.delta_secs();
    }

    // move down
    if keyboard.just_pressed(KeyCode::KeyC) {
        transform.translation.y -= mv + time.delta_secs();
    }

    // move forwards
    if keyboard.just_pressed(KeyCode::KeyW) {
        transform.translation.z -= mv + time.delta_secs();
    }

    // move left
    if keyboard.just_pressed(KeyCode::KeyS) {
        transform.translation.z += mv + time.delta_secs();
    }

    // move right
    if keyboard.just_pressed(KeyCode::KeyA) {
        transform.translation.x -= mv + time.delta_secs();
    }

    // move backwards
    if keyboard.just_pressed(KeyCode::KeyD) {
        transform.translation.x += mv + time.delta_secs();
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        // make camera look at the center
        Transform::from_xyz(0.0, 0.0, 12.0),
        // Transform::from_xyz(3.0, 7., 7.0)
        //     .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        MyCamera,
    ));

    // light
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}

fn mouse_motion(
    mut camera: Query<&mut Transform, With<MyCamera>>,
    mut evr_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut t = camera.single_mut();

    for ev in evr_motion.read() {
        info!("X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

        let mut end = t.rotation;
        end.y -= ev.delta.x;
        end.x -= ev.delta.y;

        t.rotation = t.rotation.lerp(end, time.delta_secs() * 0.1);
    }
}

impl Plugin for CameraPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            (
                mouse_motion,
                move_camera.run_if(in_state(AppState::InGame)),
            ),
        );
    }
}
