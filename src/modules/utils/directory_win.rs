extern crate winapi;

use std::ffi::OsStr;
use std::iter;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::handleapi;
use winapi::um::processthreadsapi;
use winapi::um::securitybaseapi;
use winapi::um::winnt::{
    SecurityImpersonation, DACL_SECURITY_INFORMATION, FILE_ALL_ACCESS, FILE_GENERIC_EXECUTE,
    FILE_GENERIC_READ, FILE_GENERIC_WRITE, GENERIC_MAPPING, GROUP_SECURITY_INFORMATION, HANDLE,
    OWNER_SECURITY_INFORMATION, PRIVILEGE_SET, PSECURITY_DESCRIPTOR, STANDARD_RIGHTS_READ,
    TOKEN_DUPLICATE, TOKEN_IMPERSONATE, TOKEN_QUERY,
};

pub fn is_write_allowed(folder_path: &str) -> std::result::Result<bool, &'static str> {
    let folder_name: Vec<u16> = OsStr::new(folder_path)
        .encode_wide()
        .chain(iter::once(0))
        .collect();
    let mut length: DWORD = 0;

    let rc = unsafe {
        securitybaseapi::GetFileSecurityW(
            folder_name.as_ptr(),
            OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            std::ptr::null_mut(),
            0,
            &mut length,
        )
    };
    if rc != 0 {
        return Err(
            "GetFileSecurityW returned non-zero when asked for the security descriptor size",
        );
    }

    let mut buf: Vec<u8> = Vec::with_capacity(length as usize);

    let rc = unsafe {
        securitybaseapi::GetFileSecurityW(
            folder_name.as_ptr(),
            OWNER_SECURITY_INFORMATION | GROUP_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            buf.as_mut_ptr() as *mut c_void,
            length,
            &mut length,
        )
    };

    if rc != 1 {
        return Err("GetFileSecurityW failed to retrieve the security descriptor");
    }

    let mut token: HANDLE = 0 as HANDLE;
    let rc = unsafe {
        processthreadsapi::OpenProcessToken(
            processthreadsapi::GetCurrentProcess(),
            TOKEN_IMPERSONATE | TOKEN_QUERY | TOKEN_DUPLICATE | STANDARD_RIGHTS_READ,
            &mut token,
        )
    };
    if rc != 1 {
        return Err("OpenProcessToken failed to retrieve current process' security token");
    }

    let mut impersonated_token: HANDLE = 0 as HANDLE;
    let rc = unsafe {
        securitybaseapi::DuplicateToken(token, SecurityImpersonation, &mut impersonated_token)
    };
    if rc != 1 {
        unsafe { handleapi::CloseHandle(token) };
        return Err("DuplicateToken failed");
    }

    let mut mapping: GENERIC_MAPPING = GENERIC_MAPPING {
        GenericRead: FILE_GENERIC_READ,
        GenericWrite: FILE_GENERIC_WRITE,
        GenericExecute: FILE_GENERIC_EXECUTE,
        GenericAll: FILE_ALL_ACCESS,
    };

    let mut priviledges: PRIVILEGE_SET = PRIVILEGE_SET::default();
    let mut priv_size = mem::size_of::<PRIVILEGE_SET>() as DWORD;
    let mut granted_access: DWORD = 0;
    let mut access_rights: DWORD = FILE_GENERIC_WRITE;
    let mut result: BOOL = 0 as BOOL;
    unsafe { securitybaseapi::MapGenericMask(&mut access_rights, &mut mapping) };
    let rc = unsafe {
        securitybaseapi::AccessCheck(
            buf.as_mut_ptr() as PSECURITY_DESCRIPTOR,
            impersonated_token,
            access_rights,
            &mut mapping,
            &mut priviledges,
            &mut priv_size,
            &mut granted_access,
            &mut result,
        )
    };
    unsafe {
        handleapi::CloseHandle(impersonated_token);
        handleapi::CloseHandle(token);
    }

    if rc != 1 {
        return Err("AccessCheck failed");
    }

    Ok(result != 0)
}
