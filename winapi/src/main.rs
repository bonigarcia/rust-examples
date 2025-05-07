use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::LPVOID;
use winapi::um::winver::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW};

fn get_file_version(file_path: &str) -> Option<String> {
    unsafe {
        // Convert the file path to a wide string
        let wide_path: Vec<u16> = OsStr::new(file_path).encode_wide().chain(Some(0)).collect();

        // Get the size of version info
        let mut dummy = 0;
        let size = GetFileVersionInfoSizeW(wide_path.as_ptr(), &mut dummy);
        if size == 0 {
            return None;
        }

        // Allocate buffer and get version info
        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
        if GetFileVersionInfoW(wide_path.as_ptr(), 0, size, buffer.as_mut_ptr() as LPVOID) == 0 {
            return None;
        }
        buffer.set_len(size as usize);

        // Query the product version
        let mut lang_and_codepage_ptr: LPVOID = ptr::null_mut();
        let mut lang_and_codepage_len: u32 = 0;

        if VerQueryValueW(
            buffer.as_ptr() as LPVOID,
            OsStr::new("\\VarFileInfo\\Translation")
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr(),
            &mut lang_and_codepage_ptr,
            &mut lang_and_codepage_len,
        ) == 0
        {
            return None;
        }

        if lang_and_codepage_len == 0 {
            return None;
        }

        let lang_and_codepage_slice = std::slice::from_raw_parts(
            lang_and_codepage_ptr as *const u16,
            lang_and_codepage_len as usize / 2,
        );
        let lang = lang_and_codepage_slice[0];
        let codepage = lang_and_codepage_slice[1];

        // Format the query string for the product version
        let query = format!(
            "\\StringFileInfo\\{:04x}{:04x}\\ProductVersion",
            lang, codepage
        );
        let query_wide: Vec<u16> = OsStr::new(&query).encode_wide().chain(Some(0)).collect();

        let mut product_version_ptr: LPVOID = ptr::null_mut();
        let mut product_version_len: u32 = 0;

        if VerQueryValueW(
            buffer.as_ptr() as LPVOID,
            query_wide.as_ptr(),
            &mut product_version_ptr,
            &mut product_version_len,
        ) == 0
        {
            return None;
        }

        if product_version_ptr.is_null() {
            return None;
        }

        let product_version_slice = std::slice::from_raw_parts(
            product_version_ptr as *const u16,
            product_version_len as usize,
        );
        let product_version = String::from_utf16_lossy(product_version_slice);

        Some(product_version.trim_end_matches('\0').to_string())
    }
}

fn main() {
    let browser_path = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
    //let browser_path = "C:\\Program Files\\Mozilla Firefox\\firefox.exe";
    match get_file_version(browser_path) {
        Some(version) => println!("Product version: {}", version),
        None => println!("Failed to get version information"),
    }
}
