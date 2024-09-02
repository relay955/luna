use std::ptr;
use widestring::U16CString;
use windows::core::PWSTR;
use windows::{
    core::PCWSTR,
    Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
    Win32::UI::Shell::AssocQueryStringW,
    Win32::UI::Shell::ASSOCF_NONE,
    Win32::UI::Shell::ASSOCSTR_EXECUTABLE,
};

pub fn get_associated_program(extension: &str) -> Option<String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().expect("TODO: panic message");
    }

    let extension_wide = U16CString::from_str(extension).unwrap();
    let mut buffer_size: u32 = 0;

    // First, get the buffer size needed
    unsafe {
        let result = AssocQueryStringW(
            ASSOCF_NONE,
            ASSOCSTR_EXECUTABLE,
            PCWSTR(extension_wide.as_ptr()),
            PCWSTR(ptr::null()),
            PWSTR(ptr::null_mut()),
            &mut buffer_size,
        );

        if result.is_err() {
            return None;
        }
    }

    // Allocate buffer and retrieve the associated program
    let mut buffer: Vec<u16> = vec![0; buffer_size as usize];

    unsafe {
        let result = AssocQueryStringW(
            ASSOCF_NONE,
            ASSOCSTR_EXECUTABLE,
            PCWSTR(extension_wide.as_ptr()),
            PCWSTR(ptr::null()),
            PWSTR(buffer.as_mut_ptr()),
            &mut buffer_size,
        );

        if result.is_err() {
            return None;
        }
    }

    let path = U16CString::from_vec(buffer)
        .ok()
        .and_then(|s| s.to_string().ok());

    path
}