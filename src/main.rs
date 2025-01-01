use bevy_test::{BodyKinematics, plugins::*};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// A marker component for our shapes so we can query them separately from the
/// ground plane
#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    ass: Res<AssetServer>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(debug_material),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color: RED.into(),
        //     ..Default::default()
        // })),
        Transform::default(),
        BodyKinematics {
            speed: 10.0,
            start_pos: Transform::default().translation,
        },
        Shape,
        // Transform::from_rotation(Quat::from_rotation_x(
        //     -std::f32::consts::FRAC_PI_2,
        // )),
    ));

    // let ground_gltf = ass.load("../assets/ground.gltf#Scene0");

    commands.spawn((Text::new("WASD to move"), Node {
        position_type: PositionType::Absolute,
        top: Val::Px(12.0),
        left: Val::Px(12.0),
        ..default()
    }));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_test::plugins::CameraPlugin,
            AppStatePlugin,
            GridPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255,
        102, 255, 102, 255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255,
        236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)]
            .copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
