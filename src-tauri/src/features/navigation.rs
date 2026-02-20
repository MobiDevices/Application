#[cfg(desktop)]
use serde::Deserialize;
#[cfg(desktop)]
use std::collections::HashMap;
#[cfg(desktop)]
use std::sync::OnceLock;
#[cfg(desktop)]
use tauri::menu::{
    AboutMetadata, Menu, MenuItemBuilder, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID,
    WINDOW_SUBMENU_ID,
};
#[cfg(desktop)]
use tauri::Manager;

#[cfg(target_os = "macos")]
use objc2::runtime::AnyObject;
#[cfg(target_os = "macos")]
use objc2::sel;
#[cfg(target_os = "macos")]
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSButton, NSWindow, NSWindowButton, NSWindowTitleVisibility,
    NSWindowToolbarStyle,
};
#[cfg(target_os = "macos")]
use objc2_foundation::{ns_string, MainThreadMarker, NSPoint, NSRect, NSSize};

#[cfg(desktop)]
const MENU_NAV_BACK_ID: &str = "nav_back";
#[cfg(desktop)]
const MENU_NAV_FORWARD_ID: &str = "nav_forward";
#[cfg(desktop)]
const MENU_NAV_RELOAD_ID: &str = "nav_reload";
#[cfg(desktop)]
const MENU_DEFAULT_LANGUAGE: &str = "en";

#[cfg(desktop)]
const HISTORY_BACK_SCRIPT: &str = include_str!("navigation/scripts/history_back.js");
#[cfg(desktop)]
const HISTORY_FORWARD_SCRIPT: &str = include_str!("navigation/scripts/history_forward.js");

#[cfg(desktop)]
#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)] // Some fields are platform-specific and not always read on current target.
struct MenuTexts {
    nav_menu: String,
    nav_back: String,
    nav_forward: String,
    nav_reload: String,
    file_menu: String,
    file_close_window: String,
    file_quit: String,
    edit_menu: String,
    edit_undo: String,
    edit_redo: String,
    edit_cut: String,
    edit_copy: String,
    edit_paste: String,
    edit_select_all: String,
    view_menu: String,
    view_fullscreen: String,
    window_menu: String,
    window_minimize: String,
    window_maximize: String,
    window_close: String,
    help_menu: String,
    help_about: String,
    app_about: String,
    app_services: String,
    app_hide: String,
    app_hide_others: String,
    app_quit: String,
}

#[cfg(desktop)]
static MENU_TRANSLATIONS: OnceLock<HashMap<String, MenuTexts>> = OnceLock::new();

#[cfg(desktop)]
fn parse_menu_texts(language_key: &str, raw: &str) -> MenuTexts {
    serde_yaml::from_str::<MenuTexts>(raw)
        .unwrap_or_else(|err| panic!("invalid i18n/{language_key}/translate.yml format: {err}"))
}

#[cfg(desktop)]
fn load_menu_translations() -> &'static HashMap<String, MenuTexts> {
    MENU_TRANSLATIONS.get_or_init(|| {
        let mut translations = HashMap::new();
        translations.insert(
            "en".to_string(),
            parse_menu_texts("en", include_str!("../../i18n/en/translate.yml")),
        );
        translations.insert(
            "ru".to_string(),
            parse_menu_texts("ru", include_str!("../../i18n/ru/translate.yml")),
        );
        translations
    })
}

#[cfg(desktop)]
fn language_key_from_locale_tag(tag: &str) -> String {
    tag.trim()
        .split(['.', '@', ':'])
        .next()
        .unwrap_or_default()
        .split(['-', '_'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

#[cfg(desktop)]
fn detect_user_menu_language() -> String {
    if let Some(tag) = sys_locale::get_locale() {
        let language_key = language_key_from_locale_tag(&tag);
        if !language_key.is_empty() {
            return language_key;
        }
    }

    for key in ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"] {
        if let Ok(value) = std::env::var(key) {
            if let Some(first_tag) = value.split(':').find(|part| !part.trim().is_empty()) {
                return language_key_from_locale_tag(first_tag);
            }
        }
    }

    "en".to_string()
}

#[cfg(desktop)]
fn menu_texts_for_language(language_key: &str) -> MenuTexts {
    let translations = load_menu_translations();

    if let Some(texts) = translations.get(language_key) {
        return texts.clone();
    }

    if let Some(texts) = translations.get(MENU_DEFAULT_LANGUAGE) {
        return texts.clone();
    }

    translations
        .values()
        .next()
        .cloned()
        .unwrap_or_else(|| panic!("menu translations are not configured"))
}

#[cfg(desktop)]
pub fn build_navigation_menu<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> tauri::Result<Menu<R>> {
    let language_key = detect_user_menu_language();
    let texts = menu_texts_for_language(&language_key);
    let pkg_info = app.package_info();
    let config = app.config();
    let about_metadata = AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };

    let back = MenuItemBuilder::with_id(MENU_NAV_BACK_ID, &texts.nav_back)
        .accelerator("Alt+Left")
        .build(app)?;
    let forward = MenuItemBuilder::with_id(MENU_NAV_FORWARD_ID, &texts.nav_forward)
        .accelerator("Alt+Right")
        .build(app)?;
    let reload = MenuItemBuilder::with_id(MENU_NAV_RELOAD_ID, &texts.nav_reload)
        .accelerator("CmdOrCtrl+R")
        .build(app)?;

    let navigation_menu = Submenu::with_id_and_items(
        app,
        "navigation_menu",
        &texts.nav_menu,
        true,
        &[&back, &forward, &reload],
    )?;

    let window_menu = Submenu::with_id_and_items(
        app,
        WINDOW_SUBMENU_ID,
        &texts.window_menu,
        true,
        &[
            &PredefinedMenuItem::minimize(app, Some(&texts.window_minimize))?,
            &PredefinedMenuItem::maximize(app, Some(&texts.window_maximize))?,
            #[cfg(target_os = "macos")]
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::close_window(app, Some(&texts.window_close))?,
        ],
    )?;

    let help_menu = Submenu::with_id_and_items(
        app,
        HELP_SUBMENU_ID,
        &texts.help_menu,
        true,
        &[
            #[cfg(not(target_os = "macos"))]
            &PredefinedMenuItem::about(app, Some(&texts.help_about), Some(about_metadata.clone()))?,
        ],
    )?;

    let menu = Menu::with_items(
        app,
        &[
            #[cfg(target_os = "macos")]
            &Submenu::with_items(
                app,
                pkg_info.name.clone(),
                true,
                &[
                    &PredefinedMenuItem::about(
                        app,
                        Some(&texts.app_about),
                        Some(about_metadata.clone()),
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::services(app, Some(&texts.app_services))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::hide(app, Some(&texts.app_hide))?,
                    &PredefinedMenuItem::hide_others(app, Some(&texts.app_hide_others))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::quit(app, Some(&texts.app_quit))?,
                ],
            )?,
            #[cfg(not(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            )))]
            &Submenu::with_items(
                app,
                &texts.file_menu,
                true,
                &[
                    &PredefinedMenuItem::close_window(app, Some(&texts.file_close_window))?,
                    #[cfg(not(target_os = "macos"))]
                    &PredefinedMenuItem::quit(app, Some(&texts.file_quit))?,
                ],
            )?,
            &Submenu::with_items(
                app,
                &texts.edit_menu,
                true,
                &[
                    &PredefinedMenuItem::undo(app, Some(&texts.edit_undo))?,
                    &PredefinedMenuItem::redo(app, Some(&texts.edit_redo))?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::cut(app, Some(&texts.edit_cut))?,
                    &PredefinedMenuItem::copy(app, Some(&texts.edit_copy))?,
                    &PredefinedMenuItem::paste(app, Some(&texts.edit_paste))?,
                    &PredefinedMenuItem::select_all(app, Some(&texts.edit_select_all))?,
                ],
            )?,
            #[cfg(target_os = "macos")]
            &Submenu::with_items(
                app,
                &texts.view_menu,
                true,
                &[&PredefinedMenuItem::fullscreen(
                    app,
                    Some(&texts.view_fullscreen),
                )?],
            )?,
            &navigation_menu,
            &window_menu,
            &help_menu,
        ],
    )?;

    Ok(menu)
}

#[cfg(desktop)]
pub fn handle_menu_event<R: tauri::Runtime>(app: &tauri::AppHandle<R>, event_id: &str) {
    let Some(main_window) = app.get_webview_window("main") else {
        return;
    };

    match event_id {
        MENU_NAV_BACK_ID => {
            let _ = main_window.eval(HISTORY_BACK_SCRIPT);
        }
        MENU_NAV_FORWARD_ID => {
            let _ = main_window.eval(HISTORY_FORWARD_SCRIPT);
        }
        MENU_NAV_RELOAD_ID => {
            let _ = main_window.reload();
        }
        _ => {}
    }
}

#[cfg(target_os = "macos")]
pub fn install_macos_titlebar_nav<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
) -> tauri::Result<()> {
    window.with_webview(|webview| {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };

        let ns_window: &NSWindow = unsafe { &*webview.ns_window().cast() };
        let webview_target: &AnyObject = unsafe { &*webview.inner().cast() };

        ns_window.setTitleVisibility(NSWindowTitleVisibility::Visible);
        ns_window.setTitlebarAppearsTransparent(false);
        ns_window.setToolbarStyle(NSWindowToolbarStyle::Unified);

        let Some(close_button) = ns_window.standardWindowButton(NSWindowButton::CloseButton) else {
            return;
        };
        let Some(zoom_button) = ns_window.standardWindowButton(NSWindowButton::ZoomButton) else {
            return;
        };
        let Some(title_buttons_row) = (unsafe { close_button.superview() }) else {
            return;
        };
        let Some(titlebar_container) = (unsafe { title_buttons_row.superview() }) else {
            return;
        };

        let close_frame = close_button.frame();
        let zoom_frame = zoom_button.frame();
        let titlebar_frame = titlebar_container.frame();

        let button_width = 24.0;
        let button_height = close_frame.size.height + 4.0;
        let button_y = close_frame.origin.y - 2.0;
        let gap = 6.0;
        let total_width = button_width * 2.0 + gap;
        let right_margin = 14.0;
        let min_left_x = zoom_frame.origin.x + zoom_frame.size.width + 16.0;
        let left_x = (titlebar_frame.size.width - total_width - right_margin).max(min_left_x);

        let back_button = unsafe {
            NSButton::buttonWithTitle_target_action(
                ns_string!("←"),
                Some(webview_target),
                Some(sel!(goBack:)),
                mtm,
            )
        };
        back_button.setFrame(NSRect::new(
            NSPoint::new(left_x, button_y),
            NSSize::new(button_width, button_height),
        ));

        let forward_button = unsafe {
            NSButton::buttonWithTitle_target_action(
                ns_string!("→"),
                Some(webview_target),
                Some(sel!(goForward:)),
                mtm,
            )
        };
        forward_button.setFrame(NSRect::new(
            NSPoint::new(left_x + button_width + gap, button_y),
            NSSize::new(button_width, button_height),
        ));

        back_button.setAutoresizingMask(NSAutoresizingMaskOptions::ViewMinXMargin);
        forward_button.setAutoresizingMask(NSAutoresizingMaskOptions::ViewMinXMargin);

        titlebar_container.addSubview(&back_button);
        titlebar_container.addSubview(&forward_button);
    })?;

    Ok(())
}
