extern crate kernel32;
extern crate winapi;

use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use winapi::shared::ntdef::ULARGE_INTEGER;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{GetDiskFreeSpaceExW, GetLogicalDrives, GetVolumeInformationW};
use winapi::um::winbase::FormatMessageW;
use winapi::um::winnt::WCHAR;

#[derive(Clone)]
pub struct DriveInfo {
    pub name: String,
    pub full_path: String,
    pub total_space: u64,
    pub free_space: u64
}

impl Serialize for DriveInfo{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("DriveInfo", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("full_path", &self.full_path)?;
        state.serialize_field("total_space", &self.total_space)?;
        state.serialize_field("free_space", &self.free_space)?;
        state.end()
    }
}

pub fn get_drive_list() -> Vec<DriveInfo> {
    unsafe {
        let drives = GetLogicalDrives();
        let mut drive_names:Vec<DriveInfo> = Vec::new();
        for i in 0..26 {
            if drives & (1 << i) != 0 {
                let mut drive_name: [WCHAR; 4] = [0; 4];
                drive_name[0] = 'A' as WCHAR + i;
                drive_name[1] = ':' as WCHAR;
                drive_name[2] = '\\' as WCHAR;

                let drive_letter = (b'A' + (i as u8)) as char;
                let label = get_drive_label(drive_letter).unwrap_or(String::from("드라이브"));
                let drive_space = get_drive_space(drive_letter).unwrap_or((0, 0));
                drive_names.push(
                    DriveInfo{
                        name: label.replace("\0", ""),
                        full_path: drive_name.iter()
                            .map(|x| *x as u16 as u8 as char)
                            .filter(|x| *x != '\0')
                            .collect::<String>(),
                        total_space: drive_space.0,
                        free_space: drive_space.1
                    }
                );
            }
        }
        drive_names
    }
}

fn get_drive_space(drive_letter: char) -> Option<(u64, u64)> {
    let drive_name = format!("{}:\\", drive_letter);
    let drive_name_wide = widestring::U16CString::from_str(drive_name).unwrap();

    let mut free_bytes_available: ULARGE_INTEGER = u64_to_ularge_integer(0);
    let mut total_number_of_bytes: ULARGE_INTEGER = u64_to_ularge_integer(0);
    let mut total_number_of_free_bytes: ULARGE_INTEGER = u64_to_ularge_integer(0);

    let result = unsafe {
        GetDiskFreeSpaceExW(
            drive_name_wide.as_ptr(),
            &mut free_bytes_available,
            &mut total_number_of_bytes,
            &mut total_number_of_free_bytes,
        )
    };

    if result == 0 {
        let error_message = get_last_error_message();
        println!("Failed to get disk space information for drive {}: {}", drive_letter, error_message);
        None
    } else {
        Some((
            ularge_integer_to_u64(total_number_of_bytes),
            ularge_integer_to_u64(total_number_of_free_bytes)
         ))
    }
}

fn get_drive_label(drive_letter: char) -> Option<String> {
    let mut volume_name: [WCHAR; 261] = [0; 261];
    let mut file_system_name: [WCHAR; 261] = [0; 261];
    let mut serial_number: u32 = 0;
    let mut max_component_len: u32 = 0;
    let mut file_system_flags: u32 = 0;

    let drive_name = format!("{}:\\", drive_letter);
    
    let drive_name_wide = widestring::U16CString::from_str(drive_name).unwrap();

    let result = unsafe {
        GetVolumeInformationW(
            drive_name_wide.as_ptr(),
            volume_name.as_mut_ptr(),
            volume_name.len() as u32,
            &mut serial_number,
            &mut max_component_len,
            &mut file_system_flags,
            file_system_name.as_mut_ptr(),
            file_system_name.len() as u32,
        )
    };

    if result == 0 {
        let error_message = get_last_error_message();
        println!("Failed to get volume information for drive {}: {}", drive_letter, error_message);
        None
    } else {
        let os_string = OsString::from_wide(&volume_name);
        os_string.into_string().ok()
    }
}

fn u64_to_ularge_integer(value: u64) -> ULARGE_INTEGER {
    unsafe {
        mem::transmute(value)
    }
}

fn ularge_integer_to_u64(value: ULARGE_INTEGER) -> u64 {
    unsafe {
        mem::transmute(value)
    }
}
fn get_last_error_message() -> String {
    let error_code = unsafe { GetLastError() };
    if error_code == 0 {
        return "No error".to_string();
    }

    let mut buffer: [WCHAR; 256] = [0; 256];
    unsafe {
        let length = FormatMessageW(
            winapi::um::winbase::FORMAT_MESSAGE_FROM_SYSTEM,
            null_mut(),
            error_code,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            null_mut(),
        );

        if length == 0 {
            return format!("Unknown error code: {}", error_code);
        }

        let message = OsString::from_wide(&buffer[..length as usize]);
        message.to_string_lossy().into_owned()
    }
}
