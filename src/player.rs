use bevy::prelude::*;

pub(crate) struct CarPlugin;
impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_car)
            .add_systems(Update, handle_car_physics);
    }
}

fn spawn_player_car(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let model = asset_server.load("d_gizmod.glb#Scene0");

    commands.spawn((SceneBundle {
        scene: model,
        transform: Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        ..default()
    },
                    Car {
                        speed: 5.0,
                        turn_speed: 3.0,
                    })
    );
}

#[derive(Component)]
pub struct Car {
    speed: f32,
    turn_speed: f32
}

fn handle_car_physics(
    time: Res<Time>,
    mut car_q: Query<(&Car, &mut Transform), With<Car>>
) {
    let (car, mut transform) = car_q.single();
}
