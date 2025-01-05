use bevy::{
    asset::RenderAssetUsages,
    input::common_conditions::input_just_pressed,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

use super::camera::MyCamera;

const SHADER_ASSET_PATH: &str = "shaders/line_material.wgsl";

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

/// List of lines vertices
/// read as: Vec<(start_vertex, end_vertex)>
/// ----------
///
/// ----------
#[derive(Debug, Clone, Deref)]
struct LineList(Vec<(Vec3, Vec3)>);

// Necessary trait that tells the renderer how to transform LineList into a Mesh
impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> =
            line.0.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as
        // a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

fn gen_x_lines(
    n: usize,
    len: f32,
) -> Vec<(Vec3, Vec3)> {
    let mut v: Vec<(Vec3, Vec3)> = Vec::with_capacity(n * 2);

    // half of the lines will be rendered foward and the other half backwards.
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

    // half of the lines will be rendered foward and the other half backwards.
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
    mut materials: ResMut<Assets<LineMaterial>>,
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
                    Mesh3d(meshes.add(LineList(gen_x_lines(20, line_len.0)))),
                    // MeshMaterial3d(materials.add(StandardMaterial {
                    //     base_color: Color::linear_rgb(200., 0., 0.),
                    //     ..Default::default()
                    // })),
                    MeshMaterial3d(materials.add(LineMaterial {
                        color: LinearRgba::RED,
                    })),
                    Transform::default(),
                    GridX,
                ))
                .id();
            let y_id = commands
                .spawn((
                    Mesh3d(meshes.add(LineList(gen_y_lines(20, line_len.0)))),
                    MeshMaterial3d(materials.add(LineMaterial {
                        color: LinearRgba::GREEN,
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
            .add_plugins(MaterialPlugin::<LineMaterial>::default())
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
