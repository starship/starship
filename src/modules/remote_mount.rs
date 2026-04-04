use super::{Context, Module};
use crate::config::ModuleConfig;
use crate::configs::remote_mount::RemoteMountConfig;
use crate::formatter::StringFormatter;
#[cfg(target_os = "linux")]
use std::path::PathBuf;

fn alias_name(name: &str, aliases: &std::collections::HashMap<String, &str>) -> String {
    aliases
        .get(name)
        .map(|&a| a.to_string())
        .unwrap_or_else(|| name.to_string())
}

/// Creates a module with the current remote mount server info
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("remote_mount");
    let config: RemoteMountConfig = RemoteMountConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let mount_info = context
        .remote_mount_info_provider
        .get_remote_mount_info(context)?;

    let display_type = alias_name(&mount_info.fs_type, &config.type_aliases);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "hostname" => Some(Ok(mount_info.hostname.clone())),
                "user" => mount_info.user.clone().map(Ok),
                "type" => Some(Ok(display_type.clone())),
                "rtt" => mount_info.rtt.clone().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `remote_mount`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoteMountInfo {
    pub hostname: String,
    pub user: Option<String>,
    pub fs_type: String,
    pub rtt: Option<String>,
}

#[cfg_attr(test, mockall::automock)]
pub trait RemoteMountInfoProvider {
    fn get_remote_mount_info<'a>(&self, context: &'a Context<'a>) -> Option<RemoteMountInfo>;
}

pub struct RemoteMountInfoProviderImpl;

#[allow(dead_code)]
fn parse_mount_source(fs_type: &str, source: &str) -> Option<(String, Option<String>, String)> {
    let display_type;
    let hostname;
    let mut user = None;

    match fs_type {
        "nfs" | "nfs3" | "nfs4" => {
            display_type = "NFS".to_string();
            hostname = if let Some(end) = source.find("]:") {
                source[..end + 1].to_string()
            } else {
                source.split(':').next()?.to_string()
            };
        }
        "cifs" | "smb" | "smbfs" | "smb3" => {
            display_type = "SMB".to_string();
            let stripped = source.strip_prefix("//")?;
            let host_part = stripped.split('/').next()?;
            if let Some((u, h)) = host_part.split_once('@') {
                user = Some(u.to_string());
                hostname = h.to_string();
            } else {
                hostname = host_part.to_string();
            }
        }
        "fuse.sshfs" | "sshfs" | "osxfusefs" | "macfuse" => {
            display_type = "SSHFS".to_string();
            let before_colon = if let Some(end) = source.find("]:") {
                &source[..end + 1]
            } else {
                source.split(':').next()?
            };
            if let Some((u, h)) = before_colon.split_once('@') {
                user = Some(u.to_string());
                hostname = h.to_string();
            } else {
                hostname = before_colon.to_string();
            }
        }
        "davfs" | "webdav" => {
            display_type = "WebDAV".to_string();
            if let Some(stripped) = source
                .strip_prefix("https://")
                .or_else(|| source.strip_prefix("http://"))
            {
                hostname = stripped.split('/').next()?.to_string();
            } else {
                hostname = source.split('/').next()?.to_string();
            }
        }
        "fuse.s3fs" | "s3fs" => {
            display_type = "S3".to_string();
            if let Some(stripped) = source.strip_prefix("s3fs#") {
                hostname = stripped.to_string();
            } else {
                hostname = source.to_string();
            }
        }
        "fuse.rclone" | "rclone" => {
            display_type = "Rclone".to_string();
            hostname = source.split(':').next()?.to_string();
        }
        _ => return None,
    }

    if hostname.is_empty() {
        return None;
    }

    Some((hostname, user, display_type))
}

#[cfg(any(target_os = "windows", test))]
fn get_windows_root_path(current_dir_str: &str) -> Option<String> {
    if let Some(stripped) = current_dir_str.strip_prefix(r"\\?\UNC\") {
        let parts: Vec<&str> = stripped.split('\\').filter(|s| !s.is_empty()).collect();
        if parts.len() >= 2 {
            Some(format!(r"\\{}\{}\", parts[0], parts[1]))
        } else {
            None
        }
    } else if let Some(stripped) = current_dir_str.strip_prefix(r"\\?\") {
        if stripped.len() >= 2 && stripped.chars().nth(1) == Some(':') {
            let drive = &stripped[0..2];
            Some(format!("{}\\", drive.to_ascii_uppercase()))
        } else {
            None
        }
    } else if current_dir_str.starts_with(r"\\") {
        let parts: Vec<&str> = current_dir_str
            .split('\\')
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() >= 2 {
            Some(format!(r"\\{}\{}\", parts[0], parts[1]))
        } else {
            None
        }
    } else if current_dir_str.len() >= 2 && current_dir_str.chars().nth(1) == Some(':') {
        let drive = &current_dir_str[0..2];
        Some(format!("{}\\", drive.to_ascii_uppercase()))
    } else {
        None
    }
}
#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    test
))]
pub fn process_macos_mount(fstypename: &str, mntfromname: &str) -> Option<RemoteMountInfo> {
    if mntfromname.is_empty() {
        return None;
    }

    let (hostname, user, display_type) = parse_mount_source(fstypename, mntfromname)?;

    Some(RemoteMountInfo {
        hostname,
        user,
        fs_type: display_type,
        rtt: None,
    })
}

#[cfg(any(target_os = "windows", test))]
pub fn process_windows_mount(
    root_path: &str,
    fs_name: &str,
    wnet_get_connection: impl Fn(&str) -> Option<String>,
) -> Option<RemoteMountInfo> {
    #[allow(unused_assignments)]
    let mut display_type = String::new();
    let mut hostname = String::new();

    if fs_name == "NFS" {
        display_type = "NFS".to_string();
    } else if fs_name == "NTFS" || fs_name == "FAT32" || fs_name == "exFAT" || fs_name == "ReFS" {
        if root_path.starts_with(r"\\") {
            let parts: Vec<&str> = root_path.split('\\').filter(|s| !s.is_empty()).collect();
            if !parts.is_empty() {
                hostname = parts[0].to_string();
                if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("DavWWWRoot") {
                    display_type = "WebDAV".to_string();
                } else {
                    display_type = "SMB".to_string();
                }
            } else {
                return None;
            }
        } else {
            let local_name = if root_path.len() >= 2 && root_path.chars().nth(1) == Some(':') {
                &root_path[0..2]
            } else {
                return None;
            };

            if let Some(remote_name) = wnet_get_connection(local_name) {
                if remote_name.starts_with(r"\\") {
                    let parts: Vec<&str> =
                        remote_name.split('\\').filter(|s| !s.is_empty()).collect();
                    if !parts.is_empty() {
                        hostname = parts[0].to_string();
                        if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("DavWWWRoot") {
                            display_type = "WebDAV".to_string();
                        } else {
                            display_type = "SMB".to_string();
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    } else if fs_name.is_empty() {
        if root_path.starts_with(r"\\") {
            let parts: Vec<&str> = root_path.split('\\').filter(|s| !s.is_empty()).collect();
            if !parts.is_empty() {
                hostname = parts[0].to_string();
                if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("DavWWWRoot") {
                    display_type = "WebDAV".to_string();
                } else {
                    display_type = "SMB".to_string();
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None; // Unknown fs type
    }

    if display_type == "NFS" && hostname.is_empty() {
        if root_path.starts_with(r"\\") {
            let parts: Vec<&str> = root_path.split('\\').filter(|s| !s.is_empty()).collect();
            if !parts.is_empty() {
                hostname = parts[0].to_string();
            }
        } else {
            let local_name = &root_path[0..2];
            if let Some(remote_name) = wnet_get_connection(local_name) {
                if remote_name.starts_with(r"\\") {
                    let parts: Vec<&str> =
                        remote_name.split('\\').filter(|s| !s.is_empty()).collect();
                    if !parts.is_empty() {
                        hostname = parts[0].to_string();
                    }
                }
            }
        }
    }

    if !hostname.is_empty() {
        Some(RemoteMountInfo {
            hostname,
            user: None,
            fs_type: display_type,
            rtt: None,
        })
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn unescape_linux_mount_path(path: &str) -> String {
    path.replace("\\040", " ")
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\134", "\\")
}

#[cfg(target_os = "linux")]
fn parse_linux_mount_info_from_strings(
    mounts: &str,
    mountstats: Option<&str>,
    current_dir_str: &str,
) -> Option<RemoteMountInfo> {
    let mut best_match: Option<(String, PathBuf, String)> = None;
    let mut max_len = 0;

    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let fs_spec = parts[0];
            let fs_file = unescape_linux_mount_path(parts[1]);
            let fs_type = parts[2];

            if current_dir_str.starts_with(&fs_file)
                && fs_file.len() >= max_len
                && parse_mount_source(fs_type, fs_spec).is_some()
            {
                max_len = fs_file.len();
                best_match = Some((
                    fs_spec.to_string(),
                    PathBuf::from(fs_file),
                    fs_type.to_string(),
                ));
            }
        }
    }

    if let Some((fs_spec, fs_file, fs_type)) = best_match {
        let (hostname, user, display_type) = parse_mount_source(&fs_type, &fs_spec)?;

        let mut rtt = None;
        if display_type == "NFS" {
            if let Some(mountstats) = mountstats {
                let fs_file_str = fs_file.to_str()?;
                let mut in_mount = false;

                for line in mountstats.lines() {
                    if line.starts_with("device ") {
                        in_mount = line.contains(&format!(" mounted on {} ", fs_file_str));
                    }

                    if in_mount && line.trim().starts_with("execute:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 9
                            && let (Ok(ops), Ok(time)) =
                                (parts[1].parse::<f64>(), parts[8].parse::<f64>())
                            && ops > 0.0
                        {
                            let avg_time_ms = time / ops;
                            rtt = Some(format!("{:.0}ms", avg_time_ms.round()));
                        }
                    }
                }
            }
        }

        Some(RemoteMountInfo {
            hostname,
            user,
            fs_type: display_type,
            rtt,
        })
    } else {
        None
    }
}

impl RemoteMountInfoProvider for RemoteMountInfoProviderImpl {
    #[cfg(target_os = "linux")]
    fn get_remote_mount_info<'a>(&self, context: &'a Context<'a>) -> Option<RemoteMountInfo> {
        let mounts_path = crate::utils::context_path(context, "/proc/self/mounts");
        let mounts = std::fs::read_to_string(mounts_path).ok()?;

        let mountstats_path = crate::utils::context_path(context, "/proc/self/mountstats");
        let mountstats = std::fs::read_to_string(mountstats_path).ok();

        let current_dir = context.current_dir.as_path();
        let current_dir_str = current_dir.to_str()?;

        parse_linux_mount_info_from_strings(&mounts, mountstats.as_deref(), current_dir_str)
    }

    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    fn get_remote_mount_info<'a>(&self, context: &'a Context<'a>) -> Option<RemoteMountInfo> {
        use std::ffi::CStr;
        use std::os::unix::ffi::OsStrExt;

        let current_dir = context.current_dir.as_path();
        let current_dir_bytes = current_dir.as_os_str().as_bytes();

        let mut path_c = Vec::with_capacity(current_dir_bytes.len() + 1);
        path_c.extend_from_slice(current_dir_bytes);
        path_c.push(0);

        let mut stat: nix::libc::statfs = unsafe { std::mem::zeroed() };
        let res =
            unsafe { nix::libc::statfs(path_c.as_ptr() as *const nix::libc::c_char, &mut stat) };

        if res != 0 {
            return None;
        }

        let fstypename_bytes = unsafe {
            std::slice::from_raw_parts(
                stat.f_fstypename.as_ptr() as *const u8,
                stat.f_fstypename.len(),
            )
        };
        let fstypename = CStr::from_bytes_until_nul(fstypename_bytes)
            .unwrap_or_default()
            .to_string_lossy();

        let mntfromname_bytes = unsafe {
            std::slice::from_raw_parts(
                stat.f_mntfromname.as_ptr() as *const u8,
                stat.f_mntfromname.len(),
            )
        };
        let mntfromname = CStr::from_bytes_until_nul(mntfromname_bytes)
            .unwrap_or_default()
            .to_string_lossy();
        process_macos_mount(&fstypename, &mntfromname)
    }

    #[cfg(target_os = "windows")]
    fn get_remote_mount_info<'a>(&self, context: &'a Context<'a>) -> Option<RemoteMountInfo> {
        use windows::Win32::Foundation::WIN32_ERROR;
        use windows::Win32::NetworkManagement::WNet::WNetGetConnectionW;
        use windows::Win32::Storage::FileSystem::GetVolumeInformationW;
        use windows::core::{HSTRING, PWSTR};

        let current_dir = context.current_dir.as_path();
        let current_dir_str = current_dir.to_str()?;

        let root_path = get_windows_root_path(current_dir_str)?;

        let mut fs_name_buf = [0u16; 256];
        let root_path_hstring = HSTRING::from(root_path.clone());

        let res = unsafe {
            GetVolumeInformationW(
                &root_path_hstring,
                None,
                None,
                None,
                None,
                Some(&mut fs_name_buf),
            )
        };

        if res.is_ok() {
            let fs_name = String::from_utf16_lossy(&fs_name_buf)
                .trim_end_matches('\0')
                .to_string();
            process_windows_mount(&root_path, &fs_name, |local_name| {
                let local_name_hstring = HSTRING::from(local_name);
                let mut remote_name_buf = [0u16; 1024];
                let mut buf_len = remote_name_buf.len() as u32;

                let wnet_res = unsafe {
                    WNetGetConnectionW(
                        &local_name_hstring,
                        Some(PWSTR::from_raw(remote_name_buf.as_mut_ptr())),
                        &mut buf_len,
                    )
                };

                if wnet_res == WIN32_ERROR(0) {
                    Some(
                        String::from_utf16_lossy(&remote_name_buf)
                            .trim_end_matches('\0')
                            .to_string(),
                    )
                } else {
                    None
                }
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;

    #[test]
    fn test_parse_mount_source() {
        assert_eq!(
            parse_mount_source("nfs", "server:/path"),
            Some(("server".to_string(), None, "NFS".to_string()))
        );
        assert_eq!(
            parse_mount_source("nfs", "[2001:db8::1]:/path"),
            Some(("[2001:db8::1]".to_string(), None, "NFS".to_string()))
        );
        assert_eq!(
            parse_mount_source("cifs", "//server/share"),
            Some(("server".to_string(), None, "SMB".to_string()))
        );
        assert_eq!(
            parse_mount_source("cifs", "//user@server/share"),
            Some((
                "server".to_string(),
                Some("user".to_string()),
                "SMB".to_string()
            ))
        );
        assert_eq!(
            parse_mount_source("sshfs", "server:/path"),
            Some(("server".to_string(), None, "SSHFS".to_string()))
        );
        assert_eq!(
            parse_mount_source("fuse.sshfs", "user@server:/path"),
            Some((
                "server".to_string(),
                Some("user".to_string()),
                "SSHFS".to_string()
            ))
        );
        assert_eq!(
            parse_mount_source("sshfs", "user@[2001:db8::1]:/path"),
            Some((
                "[2001:db8::1]".to_string(),
                Some("user".to_string()),
                "SSHFS".to_string()
            ))
        );
        assert_eq!(
            parse_mount_source("davfs", "https://server/path"),
            Some(("server".to_string(), None, "WebDAV".to_string()))
        );
        assert_eq!(
            parse_mount_source("webdav", "http://server/path"),
            Some(("server".to_string(), None, "WebDAV".to_string()))
        );
        assert_eq!(
            parse_mount_source("webdav", "server/path"),
            Some(("server".to_string(), None, "WebDAV".to_string()))
        );
        assert_eq!(
            parse_mount_source("s3fs", "s3fs#bucket"),
            Some(("bucket".to_string(), None, "S3".to_string()))
        );
        assert_eq!(
            parse_mount_source("fuse.s3fs", "bucket"),
            Some(("bucket".to_string(), None, "S3".to_string()))
        );
        assert_eq!(
            parse_mount_source("rclone", "remote:path"),
            Some(("remote".to_string(), None, "Rclone".to_string()))
        );
        assert_eq!(parse_mount_source("ext4", "/dev/sda1"), None);
        assert_eq!(parse_mount_source("nfs", ":/path"), None);
    }

    #[test]
    fn test_alias_name() {
        let mut aliases = std::collections::HashMap::new();
        aliases.insert("NFS".to_string(), "☁️ NFS");
        assert_eq!(alias_name("NFS", &aliases), "☁️ NFS".to_string());
        assert_eq!(alias_name("SMB", &aliases), "SMB".to_string());
    }

    #[test]
    fn test_module_disabled() {
        let renderer = ModuleRenderer::new("remote_mount");
        let renderer = renderer.config(toml::toml! {
            [remote_mount]
            disabled = true
        });
        assert_eq!(renderer.collect(), None);
    }

    #[test]
    fn test_get_windows_root_path() {
        assert_eq!(
            get_windows_root_path(r"\\?\UNC\server\share\folder"),
            Some(r"\\server\share\".to_string())
        );
        assert_eq!(
            get_windows_root_path(r"\\?\C:\folder"),
            Some(r"C:\".to_string())
        );
        assert_eq!(
            get_windows_root_path(r"\\?\c:\folder"),
            Some(r"C:\".to_string())
        );
        assert_eq!(
            get_windows_root_path(r"\\server\share\folder"),
            Some(r"\\server\share\".to_string())
        );
        assert_eq!(
            get_windows_root_path(r"C:\folder"),
            Some(r"C:\".to_string())
        );
        assert_eq!(
            get_windows_root_path(r"c:\folder"),
            Some(r"C:\".to_string())
        );
        assert_eq!(get_windows_root_path(r"\\server"), None);
        assert_eq!(get_windows_root_path(r"\\?\UNC\server"), None);
        assert_eq!(get_windows_root_path(r"\\?\C"), None);
        assert_eq!(get_windows_root_path(r"/mnt/linux/path"), None);
        assert_eq!(get_windows_root_path(r"C"), None);
    }

    #[test]
    fn test_module_format_errors() {
        let mut mock = MockRemoteMountInfoProvider::new();
        mock.expect_get_remote_mount_info().times(1).returning(|_| {
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: Some("user".to_string()),
                fs_type: "NFS".to_string(),
                rtt: Some("10ms".to_string()),
            })
        });

        // Line 50: Invalid format string
        let renderer1 = ModuleRenderer::new("remote_mount")
            .remote_mount_info_provider(&mock)
            .config(toml::toml! {
                [remote_mount]
                format = "[$symbol"
            });
        assert_eq!(renderer1.collect(), None);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_nfs_with_rtt() {
        let mounts = "myserver:/export /mnt/nfs_share nfs4 rw,relatime 0 0\n";
        let mountstats = "device myserver:/export mounted on /mnt/nfs_share with fstype nfs4 statvers=1.1\n\t\
            execute: 100 0 0 0 0 0 0 1000 0\n";

        let info = parse_linux_mount_info_from_strings(mounts, Some(mountstats), "/mnt/nfs_share");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: Some("10ms".to_string()),
            })
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_smb() {
        let mounts = "//smbserver/share /mnt/smb_share cifs rw,relatime 0 0\n";
        let info = parse_linux_mount_info_from_strings(mounts, None, "/mnt/smb_share/subfolder");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_smb_with_user() {
        let mounts = "//user@smbserver/share /mnt/smb_share cifs rw,relatime 0 0\n";
        let info = parse_linux_mount_info_from_strings(mounts, None, "/mnt/smb_share");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: Some("user".to_string()),
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_not_remote_mount() {
        let mounts = "/dev/sda1 /mnt/local_disk ext4 rw,relatime 0 0\n";
        let info = parse_linux_mount_info_from_strings(mounts, None, "/mnt/local_disk");
        assert_eq!(info, None);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_longest_path_match() {
        let mounts = "invalid_mount_line_no_spaces\n\
            //smbserver1/share /mnt cifs rw,relatime 0 0\n\
            //smbserver2/share /mnt/smb_share cifs rw,relatime 0 0\n";

        let info = parse_linux_mount_info_from_strings(mounts, None, "/mnt/smb_share/deep");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "smbserver2".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_malformed_mountstats() {
        let mounts = "myserver:/export /mnt/nfs_share nfs4 rw,relatime 0 0\n";
        let mountstats = "device myserver:/export mounted on /mnt/nfs_share with fstype nfs4 statvers=1.1\n\t\
            execute: 0 0 0 0 0 0 0 0 1000 0\n"; // ops is 0

        let info = parse_linux_mount_info_from_strings(mounts, Some(mountstats), "/mnt/nfs_share");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: None, // should fail to calculate RTT
            })
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_linux_mount_info_path_with_spaces() {
        let mounts = "//smbserver/share /mnt/my\\040share cifs rw,relatime 0 0\n";
        let info = parse_linux_mount_info_from_strings(mounts, None, "/mnt/my share");
        assert_eq!(
            info,
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_format_nfs_with_rtt() {
        let mut mock = MockRemoteMountInfoProvider::new();
        mock.expect_get_remote_mount_info().times(1).returning(|_| {
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: Some("100ms".to_string()),
            })
        });

        let renderer = ModuleRenderer::new("remote_mount").remote_mount_info_provider(&mock);

        let expected = Some(format!(
            "{} ",
            nu_ansi_term::Style::new()
                .bold()
                .fg(nu_ansi_term::Color::Cyan)
                .paint("☁︎ myserver")
        ));

        assert_eq!(renderer.collect(), expected);
    }

    #[test]
    fn test_format_smb() {
        let mut mock = MockRemoteMountInfoProvider::new();
        mock.expect_get_remote_mount_info().times(1).returning(|_| {
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        });

        let renderer = ModuleRenderer::new("remote_mount")
            .config(toml::toml! {
                [remote_mount]
                format = "[$symbol($type )($user@)$hostname]($style) "
            })
            .remote_mount_info_provider(&mock);

        let expected = Some(format!(
            "{} ",
            nu_ansi_term::Style::new()
                .bold()
                .fg(nu_ansi_term::Color::Cyan)
                .paint("☁︎ SMB smbserver")
        ));

        assert_eq!(renderer.collect(), expected);
    }

    #[test]
    fn test_format_smb_with_user() {
        let mut mock = MockRemoteMountInfoProvider::new();
        mock.expect_get_remote_mount_info().times(1).returning(|_| {
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: Some("user".to_string()),
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        });

        let renderer = ModuleRenderer::new("remote_mount")
            .config(toml::toml! {
                [remote_mount]
                format = "[$symbol($type )($user@)$hostname]($style) "
            })
            .remote_mount_info_provider(&mock);

        let expected = Some(format!(
            "{} ",
            nu_ansi_term::Style::new()
                .bold()
                .fg(nu_ansi_term::Color::Cyan)
                .paint("☁︎ SMB user@smbserver")
        ));

        assert_eq!(renderer.collect(), expected);
    }

    #[test]
    fn test_not_remote_mount() {
        let mut mock = MockRemoteMountInfoProvider::new();
        mock.expect_get_remote_mount_info()
            .times(1)
            .returning(|_| None);

        let renderer = ModuleRenderer::new("remote_mount").remote_mount_info_provider(&mock);

        assert_eq!(renderer.collect(), None);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows_native_api_smoke() {
        use crate::test::default_context;
        use std::path::PathBuf;

        let mut context = default_context();
        context.current_dir = PathBuf::from(r"C:\");

        let provider = RemoteMountInfoProviderImpl;
        // This is a smoke test to ensure the FFI calls don't segfault or panic
        let _ = provider.get_remote_mount_info(&context);
        assert!(true);
    }

    #[test]
    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    fn test_macos_native_api_smoke() {
        use crate::test::default_context;
        use std::path::PathBuf;

        let mut context = default_context();
        context.current_dir = PathBuf::from("/");

        let provider = RemoteMountInfoProviderImpl;
        // This is a smoke test to ensure the FFI calls don't segfault or panic
        let _ = provider.get_remote_mount_info(&context);
        assert!(true);
    }

    #[test]
    fn test_process_macos_mount() {
        assert_eq!(
            super::process_macos_mount("nfs", "myserver:/export"),
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: None,
            })
        );

        assert_eq!(super::process_macos_mount("nfs", ""), None);

        assert_eq!(
            super::process_macos_mount("unknownfs", "myserver:/export"),
            None
        );
    }

    #[test]
    fn test_process_windows_mount_nfs_unc() {
        assert_eq!(
            super::process_windows_mount(r"\\myserver\share\", "NFS", |_| None),
            Some(RemoteMountInfo {
                hostname: "myserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_ntfs_unc_smb() {
        assert_eq!(
            super::process_windows_mount(r"\\smbserver\share\", "NTFS", |_| None),
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_ntfs_unc_webdav() {
        assert_eq!(
            super::process_windows_mount(r"\\davserver\DavWWWRoot\", "NTFS", |_| None),
            Some(RemoteMountInfo {
                hostname: "davserver".to_string(),
                user: None,
                fs_type: "WebDAV".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_mapped_drive_smb() {
        assert_eq!(
            super::process_windows_mount(r"Z:\", "NTFS", |local| {
                if local == "Z:" {
                    Some(r"\\smbserver\share".to_string())
                } else {
                    None
                }
            }),
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_mapped_drive_nfs() {
        assert_eq!(
            super::process_windows_mount(r"Z:\", "NFS", |local| {
                if local == "Z:" {
                    Some(r"\\nfsserver\share".to_string())
                } else {
                    None
                }
            }),
            Some(RemoteMountInfo {
                hostname: "nfsserver".to_string(),
                user: None,
                fs_type: "NFS".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_mapped_drive_webdav() {
        assert_eq!(
            super::process_windows_mount(r"Z:\", "NTFS", |local| {
                if local == "Z:" {
                    Some(r"\\davserver\DavWWWRoot\share".to_string())
                } else {
                    None
                }
            }),
            Some(RemoteMountInfo {
                hostname: "davserver".to_string(),
                user: None,
                fs_type: "WebDAV".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_empty_fs_name_unc() {
        assert_eq!(
            super::process_windows_mount(r"\\smbserver\share\", "", |_| None),
            Some(RemoteMountInfo {
                hostname: "smbserver".to_string(),
                user: None,
                fs_type: "SMB".to_string(),
                rtt: None,
            })
        );
    }

    #[test]
    fn test_process_windows_mount_empty_fs_name_mapped_drive_fails() {
        assert_eq!(
            super::process_windows_mount(r"Z:\", "", |local| {
                if local == "Z:" {
                    Some(r"\\smbserver\share".to_string())
                } else {
                    None
                }
            }),
            None
        );
    }

    #[test]
    fn test_process_windows_mount_unknown_fs_type() {
        assert_eq!(super::process_windows_mount(r"C:\", "CDFS", |_| None), None);
    }

    #[test]
    fn test_process_windows_mount_invalid_unc_paths() {
        assert_eq!(super::process_windows_mount(r"\\", "NTFS", |_| None), None);

        assert_eq!(
            super::process_windows_mount(r"Z:\", "NTFS", |_| Some(r"\\".to_string())),
            None
        );

        assert_eq!(
            super::process_windows_mount(r"Z:\", "NTFS", |_| Some(r"not_unc".to_string())),
            None
        );
    }
}
