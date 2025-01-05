use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseMotion},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::{
    borrow::Borrow,
    f32::consts::{FRAC_PI_2, PI},
};

use crate::BodyKinematics;

use super::app_state::AppState;

#[derive(Component)]
pub struct MyCamera;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than
            // vertical. We use a component for them so that we can
            // make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect`
            // the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

pub struct CameraPlugin;

fn move_camera(
    mut q: Query<(&mut Transform, &BodyKinematics), With<MyCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, body) = q.single_mut();

    // move up
    if keyboard.pressed(KeyCode::Space) {
        let dir = transform.up();
        transform.translation += dir * body.speed * time.delta_secs();
    }

    // move down
    if keyboard.pressed(KeyCode::ShiftLeft) {
        let dir = transform.down();
        transform.translation += dir * body.speed * time.delta_secs();
    }

    // move forwards
    if keyboard.pressed(KeyCode::KeyW) {
        let dir = transform.forward();
        transform.translation += dir * body.speed * time.delta_secs();
    }

    // move backwards
    if keyboard.pressed(KeyCode::KeyS) {
        let dir = transform.back();
        transform.translation += dir * body.speed * time.delta_secs();
    }

    // move left
    if keyboard.pressed(KeyCode::KeyA) {
        let dir = transform.left();
        transform.translation += dir * body.speed * time.delta_secs();
    }

    // move right
    if keyboard.pressed(KeyCode::KeyD) {
        let dir = transform.right();
        transform.translation += dir * body.speed * time.delta_secs();
    }
}

fn mouse_motion(
    mut camera: Query<(&mut Transform, &CameraSensitivity), With<MyCamera>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    };
    let (mut transform, camera_sensitivity) = camera.single_mut();
    let delta_yaw = -delta.x * camera_sensitivity.x;
    let delta_pitch = -delta.y * camera_sensitivity.y;
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;
    // prevent gimbal lock
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    // let delta_x = Quat::from_rotation_y(-delta.x * camera_sensitivity.x);
    // transform.rotation *= delta_x;
    // let delta_y = Quat::from_rotation_x(-delta.y * camera_sensitivity.y);
    // transform.rotation *= delta_y;
}

fn startup(
    mut commands: Commands,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    // lock and center mouse
    let w_size = window.size();
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.set_cursor_position(Some(w_size / 2.));

    // let trans = Transform::from_xyz(0.0, 0.0, 3.0);
    let trans = Transform::from_xyz(3.0, 7., 3.0)
        .looking_at(Vec3::new(0., 0., 0.), Vec3::Y);

    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        trans,
        CameraSensitivity::default(),
        BodyKinematics {
            speed: 10.0,
            start_pos: trans.translation,
        },
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
            rotation: Quat::from_rotation_x(-PI / 2.),
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

impl Plugin for CameraPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            (
                mouse_motion
                    .run_if(on_event::<MouseMotion>)
                    .run_if(in_state(AppState::InGame)),
                move_camera.run_if(in_state(AppState::InGame)),
            ),
        );
    }
}
