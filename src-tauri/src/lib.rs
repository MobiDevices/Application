use tauri::utils::config::WebviewUrl;
use tauri::webview::{NewWindowResponse, WebviewWindowBuilder};
use tauri_plugin_shell::ShellExt;

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            fn log_external(msg: &str) {
                // In production builds we keep logging off by default.
                // Enable explicitly with MOBIDEVICES_LOG_EXTERNAL=1 if ever needed.
                if !cfg!(debug_assertions) && std::env::var_os("MOBIDEVICES_LOG_EXTERNAL").is_none() {
                    return;
                }

                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                let path = if cfg!(target_os = "macos") {
                    std::path::PathBuf::from("/tmp/mobidevices-external-links.log")
                } else {
                    std::env::temp_dir().join("mobidevices-external-links.log")
                };
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    let _ = writeln!(file, "[{ts}] {msg}");
                }
            }

            fn open_external(app_handle: &tauri::AppHandle, url: &str) {
                log_external(&format!("open_external: {url}"));

                #[allow(deprecated)]
                match app_handle.shell().open(url, None) {
                    Ok(_) => {
                        log_external("shell().open: OK");
                        return;
                    }
                    Err(err) => {
                        log_external(&format!("shell().open: ERR: {err}"));
                    }
                }

                match open::that(url) {
                    Ok(_) => {
                        log_external("open::that: OK");
                        return;
                    }
                    Err(err) => {
                        log_external(&format!("open::that: ERR: {err}"));
                    }
                }

                // Last resort: macOS `open` (works even when library-based open fails)
                if cfg!(target_os = "macos") {
                    match Command::new("open").arg(url).spawn() {
                        Ok(_) => {
                            log_external("Command(open): OK");
                            return;
                        }
                        Err(err) => {
                            log_external(&format!("Command(open): ERR: {err}"));
                        }
                    }
                }

                log_external("open_external: FAILED (all methods)");
            }

            fn is_internal_host(host: Option<&str>) -> bool {
                matches!(
                    host,
                    Some("mobidevices.com")
                        | Some("www.mobidevices.com")
                        | Some("m.mobidevices.com")
                        | Some("amp.mobidevices.com")
                )
            }

            fn should_open_externally(url: &url::Url) -> bool {
                let scheme = url.scheme();
                if scheme == "mailto" || scheme == "tel" {
                    return true;
                }

                if scheme == "http" || scheme == "https" {
                    let host = url.host_str();
                    return !is_internal_host(host);
                }

                // Any other schemes should be handed off to the OS.
                true
            }

            const LINK_INTERCEPT_SCRIPT: &str = r#"
                (function () {
                    try {
                        // Only run on our site.
                        var host = String(window.location.hostname || '');
                        if (!host || !(/(^|\.)mobidevices\.com$/).test(host)) return;

                        function isInternal(u) {
                            var h = (u && u.hostname) ? String(u.hostname) : '';
                            return h === 'mobidevices.com';
                        }

                        function shouldExternal(u) {
                            if (!u) return false;
                            var scheme = String(u.protocol || '').replace(':','');
                            if (scheme === 'mailto' || scheme === 'tel') return true;
                            if (scheme === 'http' || scheme === 'https') return !isInternal(u);
                            return true;
                        }

                        document.addEventListener('click', function (e) {
                            // Only handle normal primary-button clicks without modifiers.
                            if (!e || e.defaultPrevented) return;
                            if (typeof e.button === 'number' && e.button !== 0) return;
                            if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;

                            var el = e.target;
                            if (!el || !el.closest) return;

                            var a = el.closest('a, area');
                            if (!a) return;

                            var href = a.getAttribute('href') || '';
                            if (!href || href === '#') return;

                            var u;
                            try {
                                u = new URL(href, window.location.href);
                            } catch (_) {
                                return;
                            }

                            if (!shouldExternal(u)) return;

                            // Force external open path through Tauri's new-window handler.
                            e.preventDefault();
                            if (e.stopImmediatePropagation) e.stopImmediatePropagation();
                            try {
                                window.open(u.href, '_blank');
                            } catch (_) {
                                // If window.open is blocked for any reason, fallback to direct navigation.
                                window.location.href = u.href;
                            }
                        }, true);
                    } catch (_) {}
                })();
            "#;

            // We create the main window manually so we can hook into navigation/new-window.
            let url = WebviewUrl::External("https://mobidevices.com".parse()?);

            let title = format!("MobiDevices v{}", app.package_info().version);

            log_external(&format!("setup: start ({title})"));

            let app_handle_new_window = app.handle().clone();
            let app_handle_navigation = app.handle().clone();

            WebviewWindowBuilder::new(app, "main", url)
                .title(title)
                .initialization_script(LINK_INTERCEPT_SCRIPT)
                .inner_size(1200.0, 800.0)
                .min_inner_size(400.0, 600.0)
                .resizable(true)
                .fullscreen(false)
                .center()
                .on_new_window(move |url, _features| {
                    log_external(&format!("on_new_window: {}", url.as_str()));
                    #[allow(deprecated)]
                    open_external(&app_handle_new_window, url.as_str());
                    NewWindowResponse::Deny
                })
                .on_navigation(move |url| {
                    if should_open_externally(url) {
                        log_external(&format!("on_navigation (external): {}", url.as_str()));
                        open_external(&app_handle_navigation, url.as_str());
                        return false;
                    }

                    true
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
