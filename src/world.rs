use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(crate) struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));

    }
}


fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_conf: ResMut<GizmoConfig>
) {
    gizmo_conf.depth_bias = -1.0;
    let mesh = Mesh::from(shape::Plane::from_size(15.0));

    let floor = (
            PbrBundle {
                mesh: meshes.add(mesh.clone()),
                material: materials.add(Color::DARK_GREEN.into()),
                ..default()
            },
            RigidBody::Fixed,
            Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
        );

    commands.spawn(floor);

    commands.spawn(Collider::cuboid(1.5, 1.5, 1.5));
}

fn spawn_light(
    mut commands: Commands
) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(light);
}