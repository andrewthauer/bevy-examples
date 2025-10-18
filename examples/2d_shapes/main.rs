use bevy::prelude::*;
use bevy_example_app::BevyExampleApp;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyExampleApp { ..default() })
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Create square
    let square_mesh = meshes.add(Rectangle::new(100.0, 100.0));
    let square_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));

    // Spawn the square
    commands.spawn((Mesh2d(square_mesh), MeshMaterial2d(square_material)));
}
