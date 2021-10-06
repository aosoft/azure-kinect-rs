use std::ffi::c_void;
use std::ptr;
use crate::Error;

#[link(name = "kernel32")]
extern "stdcall" {
    fn GetLastError() -> u32;
    fn LoadLibraryExW(
        lpLibFileName: *const u16,
        hFile: *const c_void,
        dwFlags: u32,
    ) -> *const c_void;
    fn FreeLibrary(hLibModule: *const c_void) -> i32;
    fn GetProcAddress(hModule: *const c_void, lpProcName: *const u8) -> *const c_void;
    fn GetModuleHandleW(lpModuleName: *const u16) -> *const c_void;
}

pub(crate) struct Module {
    handle: *const c_void,
    require_free_library: bool,
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.require_free_library && self.handle != ptr::null() {
            unsafe { FreeLibrary(self.handle); }
            self.handle = ptr::null();
        }
    }
}

impl Module {
    pub fn new(handle: *const c_void, require_free_library: bool) -> Module {
        Module { handle, require_free_library }
    }

    pub fn load_library(lib_dir: &str, dll_file_name: &str) -> Result<Module, Error> {
        let full_path =
            std::path::Path::new(if lib_dir.len() > 0 { lib_dir } else { "." }).join(dll_file_name);

        unsafe {
            let p = LoadLibraryExW(
                full_path
                    .to_str()
                    .ok_or(Error::Failed)?
                    .encode_utf16()
                    .chain(Some(0))
                    .collect::<Vec<u16>>()
                    .as_ptr(),
                ptr::null(),
                0x000,
            );
            if p != ptr::null() {
                Ok(Module::new(p, true))
            } else {
                Err(Error::Win32Error(GetLastError()))
            }
        }
    }

    pub fn get_module(module_name: &str) -> Result<Module, Error> {
        unsafe {
            let p = GetModuleHandleW(module_name.encode_utf16()
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr());
            if p != ptr::null() {
                Ok(Module::new(p, false))
            } else {
                Err(Error::Win32Error(GetLastError()))
            }
        }
    }

    pub fn get_proc_address(&self, lpProcName: *const u8) -> Result<*const c_void, Error> {
        unsafe { let p = GetProcAddress(self.handle, lpProcName);
            if p != ptr::null() {
                Ok(p)
            } else {
                Err(Error::Win32Error(GetLastError()))
            }
        }
    }
}
