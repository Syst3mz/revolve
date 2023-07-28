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
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
                    Car)
    );
}

#[derive(Component)]
pub struct Car;

const FREQ:f32 = 2.0 * std::f32::consts::PI * 0.125;

fn handle_car_physics(
    time: Res<Time>,
    mut boxes: Query<&mut Transform, With<Car>>
) {
    for mut bx in boxes.iter_mut()
    {
        let pos = f32::sin(time.elapsed_seconds() * FREQ);
        bx.translation.y = pos + 0.5;
        bx.translation.x = pos;

        bx.rotation = Quat::from_rotation_y(time.elapsed_seconds() * FREQ)

    }
}
