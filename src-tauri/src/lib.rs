use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIcon,
    tray::TrayIconBuilder,
    Manager, State,
};
use tauri_plugin_positioner::{Position, WindowExt};

struct TrayState(Mutex<Option<TrayIcon>>);

#[tauri::command]
fn update_tray_title(app: tauri::AppHandle, time: String, state: State<TrayState>) {
    let tray_guard = state.0.lock().unwrap();
    if let Some(tray) = tray_guard.as_ref() {
        let _ = tray.set_title(Some(time));
    }
    let _ = app;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .manage(TrayState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![update_tray_title])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let quit_item = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_item])?;

            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .title("25:00")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.move_window(Position::TrayCenter);
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            let state: State<TrayState> = app.state();
            *state.0.lock().unwrap() = Some(tray);

            // Esconde a janela automaticamente quando perde o foco
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}