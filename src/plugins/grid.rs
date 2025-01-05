use bevy::{
    asset::RenderAssetUsages, input::common_conditions::input_just_pressed,
    prelude::*, render::mesh::PrimitiveTopology,
};

use super::camera::MyCamera;

pub struct GridPlugin;

/// If the grid is being rendered or not.
#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GridState {
    /// X entity, Y entity
    On(Entity, Entity),
    #[default]
    Off,
}

#[derive(Component)]
pub struct GridX;

#[derive(Component)]
pub struct GridY;

#[derive(Resource)]
struct LineLen(f32);

impl Default for LineLen {
    fn default() -> Self {
        Self(30.)
    }
}

/// List of lines
/// where every pair is a start and end point
/// read as: Vec<(start_point, end_point)>
/// ----------
///
/// ----------
#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

// Necessary trait that tells the renderer how to transform LineList into a Mesh
impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> =
            line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
/// ----------
/// - -  -  -
/// ----------
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
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

fn gen_x_lines(
    n: usize,
    len: f32,
) -> Vec<(Vec3, Vec3)> {
    let mut v: Vec<(Vec3, Vec3)> = Vec::with_capacity(n * 2);
    for i in 0..n / 2 {
        v.push((
            Vec3::new(-len, 0., 1. * i as f32),
            Vec3::new(len, 0., 1. * i as f32),
        ));
        v.push((
            Vec3::new(-len, 0., -(1. * i as f32)),
            Vec3::new(len, 0., -(1. * i as f32)),
        ));
    }
    v
}

fn gen_y_lines(
    n: usize,
    len: f32,
) -> Vec<(Vec3, Vec3)> {
    let mut v: Vec<(Vec3, Vec3)> = Vec::with_capacity(n * 2);
    for i in 0..n / 2 {
        v.push((
            Vec3::new(0., -len, 1. * i as f32),
            Vec3::new(0., len, 1. * i as f32),
        ));
        v.push((
            Vec3::new(0., -len, -(1. * i as f32)),
            Vec3::new(0., len, -(1. * i as f32)),
        ));
    }
    v
}

fn is_grid_on(state: Res<State<GridState>>) -> bool {
    *state.get() != GridState::Off
}

/// Listen for the trigger of the grid,
/// in this case it's Tab.
fn listen_trigger(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state_mut: ResMut<NextState<GridState>>,
    state: Res<State<GridState>>,
    line_len: Res<LineLen>,
) {
    match state.get() {
        GridState::On(x, y) => {
            commands.entity(*x).despawn();
            commands.entity(*y).despawn();
            state_mut.set(GridState::Off);
        }
        _ => {
            let x_id = commands
                .spawn((
                    Mesh3d(meshes.add(LineList {
                        lines: gen_x_lines(20, line_len.0),
                    })),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::linear_rgb(200., 0., 0.),
                        ..Default::default()
                    })),
                    Transform::default(),
                    GridX,
                ))
                .id();
            let y_id = commands
                .spawn((
                    Mesh3d(meshes.add(LineList {
                        lines: gen_y_lines(20, line_len.0),
                    })),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::linear_rgb(0., 200.0, 0.),
                        ..Default::default()
                    })),
                    Transform::default(),
                    GridY,
                ))
                .id();
            state_mut.set(GridState::On(x_id, y_id));
        }
    }
}

/// Generate the grid dynamically relative to the position of the camera.
fn render_grid_dynamically(
    mut x: Single<&mut Transform, With<GridX>>,
    mut y: Single<&mut Transform, (With<GridY>, Without<GridX>)>,
    camera: Single<
        &Transform,
        (With<MyCamera>, Without<GridX>, Without<GridY>),
    >,
) {
    let camera_z = camera.translation.z;
    let camera_y = camera.translation.y;
    let camera_x = camera.translation.x;

    x.translation.z = camera_z;
    x.translation.x = camera_x;
    y.translation.z = camera_z;
    y.translation.y = camera_y;
}

fn startup() {}

impl Plugin for GridPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<GridState>()
            .add_systems(Startup, startup)
            .init_resource::<LineLen>()
            .add_systems(
                Update,
                (
                    render_grid_dynamically.run_if(is_grid_on),
                    listen_trigger.run_if(input_just_pressed(KeyCode::Tab)),
                ),
            );
    }
}
