#[cfg(target_os = "windows")]
use {
    failure::{format_err, Error},
    std::{
        env::var,
        path::{Path, PathBuf},
    },
    winreg::{enums::*, RegKey},
};

/// Returns the path to the `Windows Kits` directory. It's by default at
/// `C:\Program Files (x86)\Windows Kits\10`.
#[cfg(target_os = "windows")]
fn get_windows_kits_dir() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

    Ok(dir.into())
}

/// Returns the path to the kernel mode libraries. The path may look like this:
/// `C:\Program Files (x86)\Windows Kits\10\lib\10.0.18362.0\km`.
#[cfg(target_os = "windows")]
fn get_km_dir(windows_kits_dir: &PathBuf) -> Result<PathBuf, Error> {
    let readdir = Path::new(windows_kits_dir).join("lib").read_dir()?;

    let max_libdir = readdir
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                .unwrap_or(false)
        })
        .max()
        .ok_or_else(|| format_err!("Can not find a valid km dir in `{:?}`", windows_kits_dir))?;

    Ok(max_libdir.join("km"))
}

#[cfg(target_os = "windows")]
fn internal_link_search() {
    let windows_kits_dir = get_windows_kits_dir().unwrap();
    let km_dir = get_km_dir(&windows_kits_dir).unwrap();
    let target = var("TARGET").unwrap();

    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("Only support x86_64 and i686!");
    };

    let lib_dir = km_dir.join(arch);
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
}

#[cfg(target_os = "windows")]
fn main() {
    internal_link_search();
    println!("cargo:rustc-link-search=native=../wfp_lib/x64/Debug");
}

#[cfg(target_os = "linux")]
fn main() {}
