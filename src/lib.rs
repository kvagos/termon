mod win {

extern crate windows_sys;
use std::io::{self};

use windows_sys::Win32::System::Console::GetConsoleMode;
use windows_sys::Win32::System::Console::SetConsoleMode;
use windows_sys::Win32::System::Console::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;

 
pub fn enable_virtual_terminal_processing() -> io::Result<()> {
    let handle: HANDLE =unsafe { windows_sys::Win32::System::Console::GetStdHandle(STD_OUTPUT_HANDLE)};
    if handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE{
        return Err(io::Error::last_os_error());
    }

    let mut mode: u32 = 0;
    if unsafe { GetConsoleMode(handle, &mut mode) } == 0 {
        return Err(io::Error::last_os_error());
    }

    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    
    if unsafe { SetConsoleMode(handle, mode) } == 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
}
