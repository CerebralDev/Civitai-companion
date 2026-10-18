//! Remembers the window size between launches (size only; the minimum stays enforced by the window config).
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window};

const FILE_NAME: &str = "window-size.json";
const MIN: (f64, f64) = (430.0, 540.0);
const MAX: (f64, f64) = (4000.0, 3000.0);

#[derive(Serialize, Deserialize)]
struct Size {
    width: f64,
    height: f64,
}

fn path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_local_data_dir()
        .ok()
        .map(|dir| dir.join(FILE_NAME))
}

/// Saved logical size, clamped to the allowed range; `None` when missing or unreadable.
pub fn load(app: &AppHandle) -> Option<(f64, f64)> {
    let size: Size = serde_json::from_slice(&fs::read(path(app)?).ok()?).ok()?;
    clamp(size.width, size.height)
}

fn clamp(width: f64, height: f64) -> Option<(f64, f64)> {
    (width.is_finite() && height.is_finite())
        .then(|| (width.clamp(MIN.0, MAX.0), height.clamp(MIN.1, MAX.1)))
}

pub fn save(window: &Window) {
    if window.is_minimized().unwrap_or(true) || window.is_maximized().unwrap_or(false) {
        return;
    }
    let (Ok(size), Ok(scale), Some(path)) = (
        window.inner_size(),
        window.scale_factor(),
        path(window.app_handle()),
    ) else {
        return;
    };
    if let Some((width, height)) = clamp(
        f64::from(size.width) / scale,
        f64::from(size.height) / scale,
    ) && let Ok(json) = serde_json::to_vec(&Size { width, height })
    {
        let _ = fs::write(path, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_size_never_goes_below_the_minimum() {
        assert_eq!(clamp(200.0, 100.0), Some(MIN));
        assert_eq!(clamp(600.0, 800.0), Some((600.0, 800.0)));
        assert_eq!(clamp(f64::NAN, 800.0), None);
    }
}
