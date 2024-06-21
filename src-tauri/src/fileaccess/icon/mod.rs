use ptr::null;
use winapi::um::shellapi::{SHFILEINFOW, SHGetFileInfoW};
use winapi::um::shellapi::SHGFI_ICON;
use winapi::um::shellapi::SHGFI_LARGEICON;
use winapi::um::winuser::{DestroyIcon, GetIconInfo, GetUserObjectSecurity, ICONINFO};
use winapi::um::wingdi::{BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, GetDIBits, GetObjectW, RGBQUAD};
use widestring::U16CString;
use std::{ptr, result};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use image::{ImageBuffer, ImageEncoder, Rgba};
use image::codecs::png::PngEncoder;
use lazy_static::lazy_static;
use once_cell::unsync::Lazy;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub fn get_from_cache(file_ext: &str) -> Option<Vec<u8>> {
    let cache = CACHE.lock().unwrap();
    cache.get(file_ext).cloned()
}

pub fn set_to_cache(file_ext: &str, icon: &Vec<u8>) {
    let mut cache = CACHE.lock().unwrap();
    if cache.contains_key(file_ext) { return; }
    cache.insert(file_ext.to_string(), icon.clone());
}

pub fn get_icon(file_path: &str) -> Result<Vec<u8>,i32> {
    let file_path = file_path.replace("/", "\\");
    let file_path_u16 = U16CString::from_str(file_path.clone()).unwrap();
    
    let ext = Path::new(&file_path).extension().and_then(|s| s.to_str());
    if ext.is_some() { 
        let ext = ext.unwrap();
        if let Some(icon) = get_from_cache(ext) {
            return Ok(icon);
        }
    }
    

    let mut shfileinfo = SHFILEINFOW {
        hIcon: ptr::null_mut(),
        iIcon: 0,
        dwAttributes: 0,
        szDisplayName: [0; 260],
        szTypeName: [0; 80],
    };
    
    unsafe {
        let result = SHGetFileInfoW(
            file_path_u16.as_ptr(),
            0,
            &mut shfileinfo,
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON,
        );

        if shfileinfo.hIcon.is_null() { return Err(0); }
        
        let mut icon_info: ICONINFO = std::mem::zeroed();
        if GetIconInfo(shfileinfo.hIcon, &mut icon_info) == 0 {return Err(0); }
        let mut bitmap: BITMAP = std::mem::zeroed();
        GetObjectW(
            icon_info.hbmColor as *mut _,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bitmap as *mut _ as *mut _,
        );

        let hdc = winapi::um::wingdi::CreateCompatibleDC(ptr::null_mut());
        if hdc.is_null() {
            println!("CreateCompatibleDC 함수 호출에 실패했습니다.");
            return Err(0);
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bitmap.bmWidth,
                biHeight: -bitmap.bmHeight,
                biPlanes: 1,
                biBitCount: bitmap.bmBitsPixel,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD{
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }; 1],
        };

        let bits_size = ((bitmap.bmWidth * bitmap.bmBitsPixel as i32 + 31) / 32) * 4 * bitmap.bmHeight;
        let mut bits: Vec<u8> = vec![0; bits_size as usize];

        let scanlines_copied = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            bitmap.bmHeight as u32,
            bits.as_mut_ptr() as *mut _,
            &mut bmi,
            DIB_RGB_COLORS,
        );

        winapi::um::wingdi::DeleteDC(hdc);
        
        // 비트맵 데이터를 RGBA 형식으로 변환하여 이미지 버퍼로 저장
        let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::new(bitmap.bmWidth as u32, bitmap.bmHeight as u32);

        let pixel_size = (bitmap.bmBitsPixel / 8) as usize;
        for y in 0..bitmap.bmHeight {
            for x in 0..bitmap.bmWidth {
                let offset = (y * bitmap.bmWidth + x) as usize * pixel_size;
                let pixel = if pixel_size == 4 {
                    // ARGB 형식인 경우
                    Rgba([
                        bits[offset + 2], // Red
                        bits[offset + 1], // Green
                        bits[offset],     // Blue
                        bits[offset + 3], // Alpha
                    ])
                } else {
                    // 다른 형식인 경우 (예: RGB)
                    Rgba([
                        bits[offset + 2], // Red
                        bits[offset + 1], // Green
                        bits[offset],     // Blue
                        255,              // Alpha (불투명)
                    ])
                };
                image_buffer.put_pixel(x as u32, y as u32, pixel);
            }
        }

        DestroyIcon(shfileinfo.hIcon);
        let mut png_data: Vec<u8> = Vec::new();
        // 이미지 파일로 저장
         PngEncoder::new(&mut png_data)
            .write_image(&image_buffer, bitmap.bmWidth as u32, bitmap.bmHeight as u32, image::ExtendedColorType::Rgba8)
            .unwrap();
        
        if ext.is_some() {
            set_to_cache(ext.unwrap(), &png_data);
        }
        Ok(png_data)
    }
}