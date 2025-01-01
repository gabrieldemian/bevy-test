use bevy::{
    asset::RenderAssetUsages,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_resource::{
            PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

pub struct GridPlugin;

#[derive(Component)]
pub struct Grid;

#[derive(Resource, Debug)]
pub struct GridResource(Entity);

#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> =
            line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}

// #[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
// struct LineMaterial {
//     #[uniform(0)]
//     color: LinearRgba,
// }
//
// impl Material for LineMaterial {
//     fn fragment_shader() -> ShaderRef {
//         SHADER_ASSET_PATH.into()
//     }
//
//     fn specialize(
//         _pipeline: &MaterialPipeline<Self>,
//         descriptor: &mut RenderPipelineDescriptor,
//         _layout: &MeshVertexBufferLayoutRef,
//         _key: MaterialPipelineKey<Self>,
//     ) -> Result<(), SpecializedMeshPipelineError> {
//         // This is the important part to tell bevy to render this material as
// a         // line between vertices
//         descriptor.primitive.polygon_mode = PolygonMode::Line;
//         Ok(())
//     }
// }

fn render_grid(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_res: Option<Res<GridResource>>,
) {
    let line = Line3d {
        direction: Dir3::new_unchecked(Vec3::new(1.0, 0.0, 0.0)),
    };

    // commands.spawn((
    //     Mesh3d(meshes.add(LineList {
    //         lines: vec![(Vec3::ZERO, Vec3::new(100.0, 0.0, 0.0))],
    //     })),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::linear_rgb(155.0, 0.0, 0.0),
    //         ..Default::default()
    //     })),
    //     Transform::default(),
    //     Grid,
    // ));

    if keyboard.just_pressed(KeyCode::Tab) {
        if let Some(grid_res) = grid_res {
            commands.entity(grid_res.0).despawn();
            commands.remove_resource::<GridResource>();
        } else {
            let id = commands
                .spawn((
                    Mesh3d(meshes.add(LineList {
                        lines: vec![
                            (
                                Vec3::new(-100.0, 0.0, 0.0),
                                Vec3::new(100.0, 0.0, 0.0),
                            ),
                            (
                                Vec3::new(-100.0, 0.0, 1.0),
                                Vec3::new(100.0, 0.0, 1.0),
                            ),
                            (
                                Vec3::new(-100.0, 0.0, 2.0),
                                Vec3::new(100.0, 0.0, 2.0),
                            ),
                        ],
                    })),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::linear_rgb(200.0, 0.0, 0.0),
                        ..Default::default()
                    })),
                    Transform::default(),
                    Grid,
                ))
                .id();
            commands.insert_resource(GridResource(id));
        }
    }

    // commands.spawn((
    //     Mesh3d(meshes.add(LineList {
    //         lines: vec![(Vec3::ZERO, Vec3::new(0.0, 0.0, 100.0))],
    //     })),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::linear_rgb(0.0, 0.0, 100.0),
    //         ..Default::default()
    //     })),
    //     Transform::default(),
    //     Grid,
    // ));

    // if is_on {
    //     let mut res = res.unwrap();
    //     res.is_on = false;
    // } else {
    //     let entity = commands.spawn((Grid,)).id();
    //     commands.insert_resource(GridResource {
    //         entity,
    //         is_on: true,
    //     });
    // }
}

fn startup(mut _commands: Commands) {}

impl Plugin for GridPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(Startup, startup)
            .add_systems(Update, render_grid);
    }
}
