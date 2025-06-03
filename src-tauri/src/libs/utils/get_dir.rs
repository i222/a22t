// src/lib/utils/get_dir.rs
use dirs;
// use std::path::PathBuf;

pub fn get_default_download_dir() -> String {
    #[cfg(target_os = "ios")]
    {
        if let Some(dir) = dirs::document_dir() {
            return dir.to_string_lossy().into_owned();
        }
        "./documents".into()
    }

    #[cfg(target_os = "android")]
    {
        if let Some(dir) = dirs::download_dir() {
            return dir.to_string_lossy().into_owned();
        }
        "./downloads".into()
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        // Windows/macOS/Linux
        if let Some(dir) = dirs::download_dir() {
            dir.to_string_lossy().into_owned()
        } else {
            "./downloads".into()
        }
    }
}
