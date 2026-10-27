use std::path::PathBuf;

pub fn config_dir(app_name: &str) -> PathBuf {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let home = home::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        home.join(".config").join(app_name)
    }
    #[cfg(target_os = "windows")]
    {
        let home = home::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        home.join("AppData").join("Roaming").join(app_name)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = app_name;
        PathBuf::from(".")
    }
}

pub fn config_path(app_name: &str, file_name: &str) -> PathBuf {
    config_dir(app_name).join(file_name)
}
