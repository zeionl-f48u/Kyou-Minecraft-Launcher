use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use window_vibrancy::apply_mica;

const SHOW_WINDOW_MENU_ID: &str = "show_window";
const QUIT_APP_MENU_ID: &str = "quit_app";

struct AppState {
    is_quitting: AtomicBool,
    close_to_tray_enabled: AtomicBool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameVersion {
    pub id: String,
    pub name: String,
    pub version: String,
    pub loader: Option<String>,
    pub loader_version: Option<String>,
    pub path: String,
    pub is_modded: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub uuid: String,
    pub token: String,
    pub account_type: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchConfig {
    pub version_id: String,
    pub account_id: String,
    pub max_memory: u32,
    pub min_memory: u32,
    pub java_path: Option<String>,
    pub java_args: Vec<String>,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn set_close_to_tray_enabled(app: tauri::AppHandle, enabled: bool) {
    app.state::<AppState>()
        .close_to_tray_enabled
        .store(enabled, Ordering::Relaxed);
}

#[tauri::command]
fn get_minecraft_dir() -> Result<String, String> {
    dirs::minecraft_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find Minecraft directory".to_string())
}

#[tauri::command]
fn get_game_versions() -> Result<Vec<GameVersion>, String> {
    let minecraft_dir = dirs::minecraft_dir().ok_or("Could not find Minecraft directory")?;
    let versions_dir = minecraft_dir.join("versions");

    if !versions_dir.exists() {
        return Ok(vec![]);
    }

    let mut versions = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let version_json_path = path.join(format!("{}.json", path.file_name().unwrap_or_default().to_string_lossy()));
                if version_json_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&version_json_path) {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                            let id = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                            let name = json.get("id")
                                .or_else(|| json.get("name"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(&id)
                                .to_string();
                            let version = json.get("version")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();

                            versions.push(GameVersion {
                                id: id.clone(),
                                name,
                                version,
                                loader: None,
                                loader_version: None,
                                path: path.to_string_lossy().to_string(),
                                is_modded: false,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(versions)
}

#[tauri::command]
fn get_launcher_profiles() -> Result<serde_json::Value, String> {
    let minecraft_dir = dirs::minecraft_dir().ok_or("Could not find Minecraft directory")?;
    let launcher_profiles = minecraft_dir.join("launcher_profiles.json");

    if !launcher_profiles.exists() {
        return Ok(serde_json::json!({}));
    }

    let content = std::fs::read_to_string(&launcher_profiles)
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[tauri::command]
fn launch_game(config: LaunchConfig) -> Result<(), String> {
    println!("Launching game with config: {:?}", config);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            is_quitting: AtomicBool::new(false),
            close_to_tray_enabled: AtomicBool::new(false),
        })
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_close_to_tray_enabled,
            get_minecraft_dir,
            get_game_versions,
            get_launcher_profiles,
            launch_game
        ])
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            let _ = apply_mica(&window, None);

            let show_window =
                MenuItem::with_id(app, SHOW_WINDOW_MENU_ID, "显示主窗口", true, None::<&str>)?;
            let quit_app = MenuItem::with_id(app, QUIT_APP_MENU_ID, "退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_window, &quit_app])?;

            let mut tray_builder = TrayIconBuilder::with_id("main-tray")
                .menu(&tray_menu)
                .tooltip("Kyou Minecraft Launcher")
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    SHOW_WINDOW_MENU_ID => show_main_window(app),
                    QUIT_APP_MENU_ID => {
                        app.state::<AppState>()
                            .is_quitting
                            .store(true, Ordering::Relaxed);
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                });

            if let Some(icon) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(icon);
            }

            tray_builder.build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<AppState>();
                if state.is_quitting.load(Ordering::Relaxed) {
                    return;
                }

                if !state.close_to_tray_enabled.load(Ordering::Relaxed) {
                    return;
                }

                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
