use bevy::prelude::*;

pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_moving_box);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let model = asset_server.load("d_gizmod.glb#Scene0");

    commands.spawn((SceneBundle {
        scene: model,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },
    MovingBox)
    );
}

#[derive(Component)]
struct MovingBox;

const FREQ:f32 = 2.0 * std::f32::consts::PI * 0.125;

fn move_moving_box(
    time: Res<Time>,
    mut boxes: Query<&mut Transform, With<MovingBox>>
) {
    for mut bx in boxes.iter_mut()
    {
        let pos = f32::sin(time.elapsed_seconds() * FREQ);
        bx.translation.y = pos + 0.5;

        bx.rotation = Quat::from_rotation_y(time.elapsed_seconds() * FREQ)

    }
}
