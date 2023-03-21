extern crate winapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::{DWORD, PDWORD, PULONG, ULONG};
use winapi::shared::ntdef::{PUNICODE_STRING, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::um::winnt::IMAGE_EXPORT_DIRECTORY;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::processthreadsapi::GetCurrentProcess;

pub struct ZeroImport {
    ntoskrnl_base: usize,
    ntoskrnl_export_dir: usize,
}

impl ZeroImport {
    pub fn init() -> Option<Self> {
        let ntoskrnl_entry = get_system_module_entry("ntoskrnl.exe")?;
        let ntoskrnl_base = ntoskrnl_entry.dll_base as usize;
        
        let p_nt_header = (ntoskrnl_base + (*(ntoskrnl_base as *const i32) as usize + 0x3c)) as *const winapi::um::winnt::IMAGE_NT_HEADERS;

        let export_dir_rva = unsafe { (*p_nt_header).OptionalHeader.DataDirectory[winapi::um::winnt::IMAGE_DIRECTORY_ENTRY_EXPORT as usize].VirtualAddress };
        if export_dir_rva == 0 {
            return None;
        }

        let ntoskrnl_export_dir = ntoskrnl_base + export_dir_rva as usize;

        Some(Self {
            ntoskrnl_base,
            ntoskrnl_export_dir,
        })
    }

    pub fn get_module_export(&self, hash: u64) -> Option<usize> {
        let export_dir = self.ntoskrnl_export_dir as *const IMAGE_EXPORT_DIRECTORY;
        let name_rvas = (self.ntoskrnl_base + unsafe { (*export_dir).AddressOfNames } as usize) as *const DWORD;

        for i in 0..unsafe { (*export_dir).NumberOfNames } {
            let name_rva = unsafe { *name_rvas.add(i as usize) };
            if hash_string((self.ntoskrnl_base + name_rva as usize) as *const i8) != hash {
                continue;
            }

            let ordinals = (self.ntoskrnl_base + unsafe { (*export_dir).AddressOfNameOrdinals } as usize) as *const u16;
            let function_rvas = (self.ntoskrnl_base + unsafe { (*export_dir).AddressOfFunctions } as usize) as *const DWORD;

            let ordinal = unsafe { *ordinals.add(i as usize) };
            let function_rva = unsafe { *function_rvas.add(ordinal as usize) };

            return Some(self.ntoskrnl_base + function_rva as usize);
        }

        None
    }
}

fn get_system_module_entry(module_name: &str) -> Option<winapi::um::winnt::LDR_DATA_TABLE_ENTRY> {
    unimplemented!(); 
}

fn hash_string(s: *const i8) -> u64 {
    unimplemented!(); 
}
