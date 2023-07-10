use bevy::prelude::shape::Plane;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use std::f32::consts::PI;

use crate::GameState;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<VertexNoiseMaterial>::default())
            .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update.in_set(OnUpdate(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VertexNoiseMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut plane = Mesh::from(Plane {
        size: 100.0,
        subdivisions: 1000,
    });
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        plane.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        plane.insert_attribute(
            Mesh::ATTRIBUTE_COLOR,
            positions
                .iter()
                .map(|[r, g, b]| [(1. - *r) / 2., (1. - *g) / 2., (1. - *b) / 2., 1.])
                .collect::<Vec<[f32; 4]>>(),
        );
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(plane),
        material: materials.add(VertexNoiseMaterial { time: Vec4::ZERO }),
        ..Default::default()
    });
}

fn update(time: Res<Time>, mut materials: ResMut<Assets<VertexNoiseMaterial>>) {
    for material in materials.iter_mut() {
        material.1.time.x += time.delta_seconds();
    }
}

impl Material for VertexNoiseMaterial {
    fn vertex_shader() -> ShaderRef {
        "sketch_1/shaders/vertex_noise_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct VertexNoiseMaterial {
    #[uniform(0)]
    time: Vec4,
}
