use std::{env::var_os, fs::{DirBuilder, File}, path::Path};

pub fn open_config() -> Result<File, std::io::Error> {
    #[cfg(target_os = "windows")]
    let path = format!(
        "{}{}\\.config\\capter.toml",
        var_os("HOMEDRIVE").unwrap().to_string_lossy(),
        var_os("HOMEPATH").unwrap().to_string_lossy()
    );

    #[cfg(not(target_os = "windows"))]
    let path = format!(
        "{}/.config/capter.toml",
        var_os("HOME").unwrap().to_string_lossy()
    );

    if !Path::new(&path).exists() {
        DirBuilder::new()
            .recursive(true)
            .create(Path::new(&path).parent().unwrap())
            .unwrap();
    }
    return File::options()
        .read(true)
        .create(true)
        .truncate(false)
        .write(true)
        .open(path);
}

pub fn default_path() -> String {
    // TODO: REPLACE the .unwrap() calls with .unwrap_or(DEFAULT_VALUE)

    #[cfg(target_os = "windows")]
    let path = format!(
        "{}{}\\Pictures\\Capter",
        var_os("HOMEDRIVE").unwrap().to_string_lossy(),
        var_os("HOMEPATH").unwrap().to_string_lossy()
    );

    #[cfg(not(target_os = "windows"))]
    let path = format!(
        "{}/Pictures/Capter",
        var_os("HOME").unwrap().to_string_lossy()
    );

    DirBuilder::new()
        .recursive(true)
        .create(Path::new(&path))
        .expect("error creating directory.");

    path
}

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