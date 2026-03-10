use crate::features::{navigation, window_title};
use crate::shared::external_links;
use tauri::utils::config::WebviewUrl;
use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};

pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![window_title::set_window_title]);

    // On Linux, window-state restore can prevent the first window from appearing
    // on some GTK/WebKit combinations. Keep startup path conservative there.
    #[cfg(all(not(mobile), not(target_os = "linux")))]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
    }

    #[cfg(desktop)]
    {
        builder = builder.on_menu_event(|app, event| {
            navigation::handle_menu_event(app, event.id().as_ref());
        });
    }

    builder
        .setup(|app| {
            // We create the main window manually so we can hook into navigation/new-window.
            let url = WebviewUrl::External("https://mobidevices.com".parse()?);
            let title = "MobiDevices";

            let app_handle_new_window = app.handle().clone();
            let app_handle_navigation = app.handle().clone();

            let _main_window = WebviewWindowBuilder::new(app, "main", url)
                .title(title)
                .initialization_script(external_links::link_intercept_script())
                .initialization_script(window_title::title_sync_script())
                .inner_size(1200.0, 800.0)
                .min_inner_size(400.0, 600.0)
                .resizable(true)
                .fullscreen(false)
                .center()
                .on_new_window(move |url, _features| {
                    external_links::log_external(&format!("on_new_window: {}", url.as_str()));
                    #[allow(deprecated)]
                    external_links::open_external(&app_handle_new_window, url.as_str());
                    NewWindowResponse::Deny
                })
                .on_navigation(move |url| {
                    if external_links::should_open_externally(url) {
                        external_links::log_external(&format!(
                            "on_navigation (external): {}",
                            url.as_str()
                        ));
                        external_links::open_external(&app_handle_navigation, url.as_str());
                        return false;
                    }

                    true
                })
                .build()?;

            #[cfg(target_os = "macos")]
            {
                navigation::install_macos_titlebar_nav(&_main_window)?;
            }

            #[cfg(not(target_os = "linux"))]
            {
                let menu = navigation::build_navigation_menu(&app.handle())?;
                app.handle().set_menu(menu)?;
            }

            #[cfg(target_os = "linux")]
            {
                if let Ok(menu) = navigation::build_navigation_menu(&app.handle()) {
                    let _ = app.handle().set_menu(menu);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
