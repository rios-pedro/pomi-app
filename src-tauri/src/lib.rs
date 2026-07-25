use std::sync::Mutex;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIcon,
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, State,
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_positioner::{Position, WindowExt};

struct TrayState(Mutex<Option<TrayIcon>>);

struct TimerState(Mutex<TimerData>);

struct TimerData {
    seconds_left: u64,
    total_seconds: u64,
    running: bool,
}

impl Default for TimerData {
    fn default() -> Self {
        TimerData { seconds_left: 25 * 60, total_seconds: 25 * 60, running: false }
    }
}

#[derive(Clone, serde::Serialize)]
struct TickPayload {
    seconds_left: u64,
    total_seconds: u64,
    running: bool,
}

fn format_time(total_seconds: u64) -> String {
    format!("{:02}:{:02}", total_seconds / 60, total_seconds % 60)
}

fn emit_tick(app: &AppHandle, data: &TimerData) {
    let _ = app.emit(
        "timer-tick",
        TickPayload {
            seconds_left: data.seconds_left,
            total_seconds: data.total_seconds,
            running: data.running,
        },
    );

    let tray_state: State<TrayState> = app.state();
    let tray_guard = tray_state.0.lock().unwrap();
    if let Some(tray) = tray_guard.as_ref() {
        let _ = tray.set_title(Some(format_time(data.seconds_left)));
    }
}

#[tauri::command]
fn start_timer(app: AppHandle, state: State<TimerState>) {
    let mut data = state.0.lock().unwrap();
    if data.seconds_left > 0 {
        data.running = true;
    }
    emit_tick(&app, &data);
}

#[tauri::command]
fn pause_timer(app: AppHandle, state: State<TimerState>) {
    let mut data = state.0.lock().unwrap();
    data.running = false;
    emit_tick(&app, &data);
}

#[tauri::command]
fn set_minutes(app: AppHandle, minutes: u64, state: State<TimerState>) {
    let mut data = state.0.lock().unwrap();
    data.running = false;
    data.total_seconds = minutes * 60;
    data.seconds_left = minutes * 60;
    emit_tick(&app, &data);
}

#[tauri::command]
fn reset_timer(app: AppHandle, state: State<TimerState>) {
    let mut data = state.0.lock().unwrap();
    data.running = false;
    data.seconds_left = data.total_seconds;
    emit_tick(&app, &data);
}

#[tauri::command]
fn get_timer_state(state: State<TimerState>) -> TickPayload {
    let data = state.0.lock().unwrap();
    TickPayload {
        seconds_left: data.seconds_left,
        total_seconds: data.total_seconds,
        running: data.running,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .manage(TrayState(Mutex::new(None)))
        .manage(TimerState(Mutex::new(TimerData::default())))
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            set_minutes,
            reset_timer,
            get_timer_state
        ])
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

            let tray_state: State<TrayState> = app.state();
            *tray_state.0.lock().unwrap() = Some(tray);

            // Esconde ao perder o foco
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = window_clone.hide();
                    }
                });
            }

            let app_handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(1));

                let timer_state: State<TimerState> = app_handle.state();
                let mut data = timer_state.0.lock().unwrap();

                if !data.running {
                    continue;
                }

                if data.seconds_left > 0 {
                    data.seconds_left -= 1;
                }

                if data.seconds_left == 0 {
                    data.running = false;
                    let _ = app_handle
                        .notification()
                        .builder()
                        .title("Pomi 🍊")
                        .body("Finished. Take a break or start another session.")
                        .sound("Ping")
                        .show();
                }

                emit_tick(&app_handle, &data);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}