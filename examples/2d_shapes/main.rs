use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // title: "I am a window!".into(),
                resolution: WindowResolution::new(1024, 768).with_scale_factor_override(1.0),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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

fn _get_window_dimensions(window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.single() {
        info!(
            "The windows resolution is {:0.0} by {:0.0}",
            window.resolution.width(),
            window.resolution.height()
        )
    }
}
