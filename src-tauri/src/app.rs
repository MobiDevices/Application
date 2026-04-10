use crate::features::{navigation, window_title};
use crate::shared::external_links;
#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "linux")]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::utils::config::WebviewUrl;
use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};

#[cfg(target_os = "linux")]
fn is_wayland_session() -> bool {
    matches!(
        env::var("XDG_SESSION_TYPE").ok().as_deref(),
        Some(value) if value.eq_ignore_ascii_case("wayland")
    ) || env::var_os("WAYLAND_DISPLAY").is_some()
}

#[cfg(target_os = "linux")]
fn disable_broken_wayland_appmenu_proxy() {
    if !is_wayland_session() {
        return;
    }

    // `appmenu-gtk-module` / `unity-gtk-module` hook GTK window realization
    // to export menus over D-Bus. On Wayland this hook is known to trip
    // `gdk_wayland_window_set_dbus_properties_libgtk_only` assertions and can
    // destabilize GTK apps. Keep our native in-window menubar and disable only
    // the external global-menu proxy for this process.
    env::set_var("UBUNTU_MENUPROXY", "0");

    let Some(modules) = env::var_os("GTK_MODULES") else {
        return;
    };

    let filtered = modules
        .to_string_lossy()
        .split(':')
        .filter(|module| {
            !matches!(
                module.trim(),
                "appmenu-gtk-module"
                    | "appmenu-gtk3-module"
                    | "unity-gtk-module"
                    | "unity-gtk3-module"
            )
        })
        .map(str::trim)
        .filter(|module| !module.is_empty())
        .collect::<Vec<_>>()
        .join(":");

    if filtered.is_empty() {
        env::remove_var("GTK_MODULES");
    } else {
        env::set_var("GTK_MODULES", filtered);
    }
}

pub fn run() {
    #[cfg(target_os = "linux")]
    disable_broken_wayland_appmenu_proxy();

    let mut builder = tauri::Builder::default().plugin(tauri_plugin_shell::init());

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

            #[allow(unused_mut)]
            let mut main_window_builder = WebviewWindowBuilder::new(app, "main", url)
                .title(title)
                .on_document_title_changed(|window, title| {
                    window_title::apply_window_title(&window, &title);
                })
                .initialization_script(external_links::link_intercept_script())
                .inner_size(1200.0, 800.0)
                .min_inner_size(400.0, 600.0)
                .resizable(true)
                .fullscreen(false)
                .on_new_window(move |url, _features| {
                    external_links::log_external(&format!("on_new_window: {}", url.as_str()));
                    external_links::open_external(url.as_str());
                    NewWindowResponse::Deny
                })
                .on_navigation(move |url| {
                    if external_links::should_open_externally(url) {
                        external_links::log_external(&format!(
                            "on_navigation (external): {}",
                            url.as_str()
                        ));
                        external_links::open_external(url.as_str());
                        return false;
                    }

                    true
                });

            #[cfg(not(target_os = "linux"))]
            {
                main_window_builder = main_window_builder.center();
            }

            let _main_window = main_window_builder.build()?;

            #[cfg(target_os = "macos")]
            {
                navigation::install_macos_titlebar_nav(&_main_window)?;
            }

            #[cfg(target_os = "linux")]
            {
                // Under Wayland/GTK, attaching the menubar during the initial startup
                // path can race with the first surface configuration. Wait until the
                // window emits a normal runtime event, then apply native setup.
                let wayland_session = is_wayland_session();
                let menu_installed = Arc::new(AtomicBool::new(false));
                let window_centered = Arc::new(AtomicBool::new(false));
                let menu_window = _main_window.clone();
                let center_window = _main_window.clone();
                let menu_app_handle = app.handle().clone();
                let menu_installed_flag = Arc::clone(&menu_installed);
                let window_centered_flag = Arc::clone(&window_centered);

                _main_window.on_window_event(move |event| {
                    let should_try_install = matches!(
                        event,
                        tauri::WindowEvent::Resized(_) | tauri::WindowEvent::Focused(true)
                    );

                    if !should_try_install {
                        return;
                    }

                    if !wayland_session
                        && window_centered_flag
                            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                            .is_ok()
                        && center_window.center().is_err()
                    {
                        window_centered_flag.store(false, Ordering::Release);
                    }

                    if menu_installed_flag
                        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok()
                    {
                        let install_result = navigation::build_navigation_menu(&menu_app_handle)
                            .and_then(|menu| menu_window.set_menu(menu).map(|_| ()));

                        if install_result.is_err() {
                            menu_installed_flag.store(false, Ordering::Release);
                        }
                    }
                });
            }

            #[cfg(not(target_os = "linux"))]
            {
                let menu = navigation::build_navigation_menu(&app.handle())?;
                app.handle().set_menu(menu)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
