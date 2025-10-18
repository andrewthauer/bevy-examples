use bevy::prelude::*;
use bevy_example_app::BevyExampleApp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyExampleApp {
            name: "Player Move".into(),
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("images/kitten.png")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.0)),
    ));
}

fn move_player(
    keycode: Res<ButtonInput<KeyCode>>,
    // mut player: Single<&mut Player>
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;
    let mut scale = 0.0;

    if keycode.pressed(KeyCode::ArrowLeft) {
        x_direction = -1.0;
    }
    if keycode.pressed(KeyCode::ArrowRight) {
        x_direction = 1.0;
    }
    if keycode.pressed(KeyCode::ArrowUp) {
        y_direction = 1.0;
    }
    if keycode.pressed(KeyCode::ArrowDown) {
        y_direction = -1.0;
    }
    if keycode.just_pressed(KeyCode::KeyJ) {
        scale -= 0.05;
    }
    if keycode.just_pressed(KeyCode::KeyK) {
        scale += 0.05;
    }

    let new_player_position_x = player_transform.translation.x + (x_direction * 10.0);
    let new_player_position_y = player_transform.translation.y + (y_direction * 10.0);
    let new_scale = player_transform.scale.x + scale;

    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
    player_transform.scale = Vec3::new(new_scale, new_scale, 0.0);
}
