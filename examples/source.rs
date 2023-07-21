extern crate winapi;

use std::ptr::null_mut as NULL;
use winapi::um::winuser;

fn main() {
    let l_msg: Vec<u16> = "Wassa wassa wassup\0".encode_utf16().collect();
    let l_title: Vec<u16> = "\u{20BF}itconnect\0".encode_utf16().collect();

    println!("Waiting 3s");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Waiting 2s");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("Waiting 1s");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    unsafe {
        winuser::MessageBoxW(
            NULL(),
            l_msg.as_ptr(),
            l_title.as_ptr(),
            winuser::MB_OK | winuser::MB_ICONINFORMATION,
        );
    }
}
