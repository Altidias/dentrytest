extern "system" {
    fn PsReferencePrimaryToken(Process: HANDLE) -> HANDLE;
    fn SeQueryInformationToken(
        TokenHandle: HANDLE,
        TokenInformationClass: TOKEN_INFORMATION_CLASS,
        TokenInformation: *mut PVOID,
    ) -> NTSTATUS;
    // WIP
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum TOKEN_INFORMATION_CLASS {
    TokenUser = 1,
    // WIP
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TOKEN_USER {
    // WIP
}
