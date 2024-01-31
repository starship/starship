use windows::Win32::{
    Foundation::{BOOL, HANDLE, PSID},
    Security, System,
};

pub fn is_sudo() -> Result<bool, String> {
    let mut sid = PSID::default();
    let admin_group = Security::SECURITY_NT_AUTHORITY;

    let rc = unsafe {
        Security::AllocateAndInitializeSid(
            &admin_group,
            2,
            System::SystemServices::SECURITY_BUILTIN_DOMAIN_RID as u32,
            System::SystemServices::DOMAIN_ALIAS_RID_ADMINS as u32,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut sid,
        )
    };

    if let Err(e) = rc.ok() {
        return Err(format!("Failed to allocate and initialize sid: {e:?}"));
    }

    let mut is_member = BOOL::default();
    let nullptr = HANDLE::default();

    unsafe {
        if let Err(e) = Security::CheckTokenMembership(nullptr, sid, &mut is_member).ok() {
            return Err(format!("Failed to check token membership: {e:?}"));
        }
        Security::FreeSid(sid);
    }

    Ok(is_member.as_bool())
}
