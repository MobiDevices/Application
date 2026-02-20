use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri_plugin_shell::ShellExt;

const LINK_INTERCEPT_SCRIPT: &str = include_str!("external_links/scripts/link_intercept.js");

pub fn link_intercept_script() -> &'static str {
    LINK_INTERCEPT_SCRIPT
}

pub fn log_external(msg: &str) {
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

pub fn open_external<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>, url: &str) {
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
    host.is_some_and(|value| value == "mobidevices.com" || value.ends_with(".mobidevices.com"))
}

fn is_youtube_embed_url(url: &url::Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };

    let host = host.to_ascii_lowercase();
    let is_youtube_host = matches!(
        host.as_str(),
        "youtube.com" | "www.youtube.com" | "m.youtube.com" | "youtube-nocookie.com" | "www.youtube-nocookie.com"
    );

    is_youtube_host && url.path().starts_with("/embed/")
}

pub fn should_open_externally(url: &url::Url) -> bool {
    let scheme = url.scheme();
    if scheme == "mailto" || scheme == "tel" {
        return true;
    }

    if scheme == "http" || scheme == "https" {
        if is_internal_host(url.host_str()) || is_youtube_embed_url(url) {
            return false;
        }

        return true;
    }

    // Any other schemes should be handed off to the OS.
    true
}
