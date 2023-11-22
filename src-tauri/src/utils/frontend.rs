use std::time::UNIX_EPOCH;
use tauri::{Runtime, Window};

pub fn set_spinner_text<R: Runtime>(window: &Window<R>, text: &str) {
    window.eval(&format!("setSpinnerText('{}')", text)).unwrap();
}

pub fn set_loadingbar_progress<R: Runtime>(window: &Window<R>, current: u64, max: u64) {
    let percent = current as f64 / max as f64 * 100.0;
    window.eval(&format!("setLoadingbarProgress({})", percent)).unwrap();
}
