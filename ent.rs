extern crate winapi;

use winapi::shared::minwindef::{DWORD, PDWORD, PULONG, ULONG, WORD};
use winapi::shared::ntdef::{LIST_ENTRY, PLIST_ENTRY, PUNICODE_STRING, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::um::winnt::IMAGE_EXPORT_DIRECTORY;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::processthreadsapi::GetCurrentProcess;

pub type HashType = usize;

pub struct LdrDataTableEntry {
    pub in_load_order_module_list: LIST_ENTRY,
    pub in_memory_order_module_list: LIST_ENTRY,
    pub in_initialization_order_module_list: LIST_ENTRY,
    pub dll_base: *mut ::std::os::raw::c_void,
    pub entry_point: *mut ::std::os::raw::c_void,
    pub size_of_image: ULONG,
    pub full_dll_name: UNICODE_STRING,
    pub base_dll_name: UNICODE_STRING,
    pub flags: ULONG,
    pub load_count: WORD,
    pub tls_index: WORD,
    pub hash_links: LIST_ENTRY,
    pub section_pointer: *mut ::std::os::raw::c_void,
    pub check_sum: ULONG,
    pub time_date_stamp: ULONG,
}

pub fn get_system_module_entry(module_name: &str) -> Option<LdrDataTableEntry> {
    unimplemented!(); 
}

pub fn hash_string(s: &str) -> HashType {
    let mut ret = 0;

    for (i, c) in s.chars().enumerate() {
        ret ^= (c as usize * c as usize) << ((i + 1) % 8);
        ret *= i + 1;
    }

    ret
}

pub fn get_module_export(hash: HashType) -> *mut ::std::os::raw::c_void {
    unimplemented!(); 
}

pub fn get_cached_module_export<const HASH: HashType>() -> *mut ::std::os::raw::c_void {
    static mut CACHED: *mut ::std::os::raw::c_void = std::ptr::null_mut();

    if CACHED.is_null() {
        unsafe {
            CACHED = get_module_export(HASH);
        }
    }

    CACHED
}

pub struct MODULEENTRY32 {
    pub dwSize: DWORD,
    pub th32ModuleID: DWORD,
    pub th32ProcessID: DWORD,
    pub GlblcntUsage: DWORD,
    pub ProccntUsage: DWORD,
    pub modBaseAddr: *mut BYTE,
    pub modBaseSize: DWORD,
    pub hModule: HMODULE,
    pub szModule: [CHAR; MAX_MODULE_NAME32 + 1],
    pub szExePath: [CHAR; MAX_PATH],
}
