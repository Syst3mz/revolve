use bevy::prelude::*;
use crate::player::Car;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_car);
    }
}

fn spawn_camera(
    mut commands: Commands
) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);
}

fn follow_car(
    car_query: Query<&Transform, With<Car>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Car>)>
) {
    let car_transform = car_query.single();
    let mut camera_transform = camera_query.single_mut();
    let above_car = car_transform.translation + Vec3::new(0.0, 3.0, 0.0);

    let camera_offset = Vec3::new(0.0, 0.0, -10.0);


    camera_transform.translation = above_car + camera_offset;
    camera_transform.look_at(above_car, Vec3::Y)


}
