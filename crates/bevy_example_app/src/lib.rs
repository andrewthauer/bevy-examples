use bevy::{prelude::*, window::PrimaryWindow};

pub struct BevyExampleApp {
    pub name: String,
}

const DEFAULT_APP_NAME: &str = "App";

impl Default for BevyExampleApp {
    fn default() -> Self {
        BevyExampleApp {
            name: DEFAULT_APP_NAME.into(),
        }
    }
}

impl Plugin for BevyExampleApp {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppInfo {
            name: self.name.clone(),
            ..default()
        });
        app.add_systems(Startup, (set_window_info, _get_window_dimensions));
    }
}

#[derive(Resource)]
pub struct AppInfo {
    pub name: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            name: DEFAULT_APP_NAME.into(),
        }
    }
}

fn set_window_info(app_info: Res<AppInfo>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.title = format!("Bevy Example - {name}", name = app_info.name.clone());
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
