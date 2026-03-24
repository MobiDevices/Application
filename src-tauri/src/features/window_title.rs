fn normalized_window_title<R: tauri::Runtime>(
    window: &tauri::WebviewWindow<R>,
    title: &str,
) -> Option<String> {
    let normalized = title.trim();
    if normalized.is_empty() {
        return None;
    }

    #[cfg(target_os = "macos")]
    let final_title = {
        let window_width = window
            .inner_size()
            .map(|size| size.width as usize)
            .unwrap_or(1200);

        let reserved_titlebar_width = 220usize;
        let available_width = window_width.saturating_sub(reserved_titlebar_width);
        let max_chars = (available_width / 9).clamp(18, 80);

        let title_chars_count = normalized.chars().count();
        if title_chars_count > max_chars {
            let visible_chars = max_chars.saturating_sub(1);
            let truncated = normalized.chars().take(visible_chars).collect::<String>();
            format!("{truncated}…")
        } else {
            normalized.to_string()
        }
    };

    #[cfg(not(target_os = "macos"))]
    let final_title = normalized.to_string();

    Some(final_title)
}

pub fn apply_window_title<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>, title: &str) {
    if let Some(final_title) = normalized_window_title(window, title) {
        let _ = window.set_title(&final_title);
    }
}
