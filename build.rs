fn main() -> shadow_rs::SdResult<()> {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO},
        Windows::Win32::Foundation::PWSTR,
    };
    shadow_rs::new()
}
