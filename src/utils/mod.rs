use std::env::var_os;

pub mod capture;
pub mod config;
pub mod ipc;
pub mod key_listener;
pub mod tray_icon;

pub fn shorten_path(path: String) -> String {
    #[cfg(target_os = "windows")]
    let home_path = format!(
        "{}{}",
        var_os("HOMEDRIVE").unwrap().to_string_lossy(),
        var_os("HOMEPATH").unwrap().to_string_lossy()
    );

    #[cfg(not(target_os = "windows"))]
    let home_path = format!("{}", var_os("HOME").unwrap().to_string_lossy());

    let replaced_path = path.replace(&home_path, "~");

    if replaced_path.len() > 20 {
        format!("...{}", &replaced_path[replaced_path.len() - 17..])
    } else {
        replaced_path
    }
}
