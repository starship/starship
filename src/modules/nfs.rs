use super::{Context, Module};
use crate::config::ModuleConfig;
use crate::configs::nfs::NfsConfig;
use crate::formatter::StringFormatter;
use std::path::PathBuf;

/// Creates a module with the current NFS server info
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nfs");
    let config: NfsConfig = NfsConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let nfs_info = get_nfs_info(context)?;

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
                "hostname" => Some(Ok(nfs_info.hostname.clone())),
                "rtt" => nfs_info.rtt.clone().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nfs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

struct NfsInfo {
    hostname: String,
    rtt: Option<String>,
}

#[cfg(target_os = "linux")]
fn get_nfs_info(context: &Context) -> Option<NfsInfo> {
    let mounts_path = crate::utils::context_path(context, "/proc/self/mounts");
    let mounts = std::fs::read_to_string(mounts_path).ok()?;

    let current_dir = context.current_dir.as_path();
    let current_dir_str = current_dir.to_str()?;

    let mut best_match: Option<(String, PathBuf, String)> = None;
    let mut max_len = 0;

    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let fs_spec = parts[0];
            let fs_file = parts[1];
            let fs_type = parts[2];

            if (fs_type == "nfs" || fs_type == "nfs4")
                && current_dir_str.starts_with(fs_file)
                    && fs_file.len() >= max_len {
                        max_len = fs_file.len();
                        best_match = Some((
                            fs_spec.to_string(),
                            PathBuf::from(fs_file),
                            fs_type.to_string(),
                        ));
                    }
        }
    }

    let (fs_spec, fs_file, _) = best_match?;

    // Extract hostname from "server:/path" or "server"
    let hostname = fs_spec.split(':').next().unwrap_or(&fs_spec).to_string();

    let mut rtt = None;

    // Attempt to calculate RTT from mountstats
    if let Ok(stats) =
        std::fs::read_to_string(crate::utils::context_path(context, "/proc/self/mountstats"))
    {
        let mut in_target_mount = false;
        let mut ops_sum: u64 = 0;
        let mut rtt_sum: u64 = 0;

        for line in stats.lines() {
            if line.starts_with("device ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                in_target_mount = parts.len() >= 5 && parts[4] == fs_file.to_str().unwrap_or("");
                continue;
            }

            if in_target_mount {
                let trimmed = line.trim();
                if trimmed.starts_with("READ:") || trimmed.starts_with("WRITE:") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 8
                        && let (Ok(ops), Ok(rtt_val)) =
                            (parts[1].parse::<u64>(), parts[7].parse::<u64>())
                        {
                            ops_sum += ops;
                            rtt_sum += rtt_val;
                        }
                }
            }
        }

        if ops_sum > 0 {
            // (rtt / ops) to get ms, but rtt in procfs is in milliseconds? Let's check kernel docs.
            // Actually, rtt in mountstats is in milliseconds, so (rtt / ops) directly gives average ms.
            let ms = rtt_sum / ops_sum;
            rtt = Some(format!("{}ms", ms));
        }
    }

    Some(NfsInfo { hostname, rtt })
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
fn get_nfs_info(context: &Context) -> Option<NfsInfo> {
    use nix::sys::statfs::statfs;

    let current_dir = context.current_dir.as_path();
    let stat = statfs(current_dir).ok()?;

    let fstypename = stat.filesystem_type_name();
    if fstypename != "nfs" && fstypename != "nfs4" {
        return None;
    }

    let mntfromname = stat.source_name().unwrap_or_default();
    if mntfromname.is_empty() {
        return None;
    }

    let hostname = mntfromname
        .split(':')
        .next()
        .unwrap_or(&mntfromname)
        .to_string();

    Some(NfsInfo {
        hostname,
        rtt: None,
    })
}

#[cfg(target_os = "windows")]
fn get_nfs_info(context: &Context) -> Option<NfsInfo> {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::NetworkManagement::WNet::{UNIVERSAL_NAME_INFO_W, WNetGetConnectionW};
    use windows::Win32::Storage::FileSystem::GetVolumeInformationW;
    use windows::core::{HSTRING, PCWSTR};

    let current_dir = context.current_dir.as_path();
    let current_dir_str = current_dir.to_str()?;

    let root_path = if current_dir_str.starts_with(r"\\") {
        // UNC path root: \\server\share\
        let parts: Vec<&str> = current_dir_str
            .split('\\')
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() >= 2 {
            format!(r"\\{}\{\}\", parts[0], parts[1])
        } else {
            return None;
        }
    } else if current_dir_str.len() >= 2 && current_dir_str.chars().nth(1) == Some(':') {
        // Drive letter: Z:\
        let drive = &current_dir_str[0..2];
        format!("{}\\", drive)
    } else {
        return None;
    };

    let mut fs_name_buf = [0u16; 256];
    let root_path_hstring = HSTRING::from(root_path.clone());

    let success = unsafe {
        GetVolumeInformationW(
            &root_path_hstring,
            None,
            None,
            None,
            None,
            Some(&mut fs_name_buf),
        )
    };

    if success.is_err() {
        return None;
    }

    let fs_name = String::from_utf16_lossy(&fs_name_buf)
        .trim_matches(char::from(0))
        .to_string();

    if fs_name.to_uppercase() != "NFS" {
        return None;
    }

    let hostname = if root_path.starts_with(r"\\") {
        // UNC path, extract server
        let parts: Vec<&str> = root_path.split('\\').filter(|s| !s.is_empty()).collect();
        if !parts.is_empty() {
            parts[0].to_string()
        } else {
            return None;
        }
    } else {
        // Drive letter mapped to NFS
        // We need to resolve it using WNetGetConnectionW
        let drive_hstring = HSTRING::from(&root_path[0..2]); // e.g. "Z:"

        let mut buffer = vec![0u8; 1024];
        let mut buffer_size = buffer.len() as u32;

        let result = unsafe {
            WNetGetConnectionW(
                &drive_hstring,
                Some(buffer.as_mut_ptr() as *mut u16),
                &mut buffer_size,
            )
        };

        if result == 0 {
            // NO_ERROR
            let unc_path_utf16: Vec<u16> = buffer
                .chunks_exact(2)
                .map(|c| u16::from_ne_bytes([c[0], c[1]]))
                .take_while(|&c| c != 0)
                .collect();

            let unc_path = String::from_utf16_lossy(&unc_path_utf16);
            let parts: Vec<&str> = unc_path.split('\\').filter(|s| !s.is_empty()).collect();
            if !parts.is_empty() {
                parts[0].to_string()
            } else {
                return None;
            }
        } else {
            return None;
        }
    };

    Some(NfsInfo {
        hostname,
        rtt: None,
    })
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use std::fs;
    use std::io::Write;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_nfs() {
        let renderer = ModuleRenderer::new("nfs");
        let root_path = renderer.root_path();

        // Mock /proc/self/mounts
        let proc_path = root_path.join("proc").join("self");
        fs::create_dir_all(&proc_path).unwrap();
        let mounts_path = proc_path.join("mounts");
        let mountstats_path = proc_path.join("mountstats");

        let current_dir = root_path.join("mnt").join("nfs_share");
        fs::create_dir_all(&current_dir).unwrap();

        let mut mounts_file = fs::File::create(&mounts_path).unwrap();
        writeln!(
            mounts_file,
            "myserver:/export {} nfs4 rw,relatime 0 0",
            current_dir.to_string_lossy()
        )
        .unwrap();
        mounts_file.sync_all().unwrap();

        let mut mountstats_file = fs::File::create(&mountstats_path).unwrap();
        writeln!(
            mountstats_file,
            "device myserver:/export mounted on {} with fstype nfs4 statvers=1.1\n\
            per-op statistics\n\
            \tREAD: 100 100 0 0 0 0 0 200000 0\n\
            \tWRITE: 100 100 0 0 0 0 0 100000 0",
            current_dir.to_string_lossy()
        )
        .unwrap();
        mountstats_file.sync_all().unwrap();

        let renderer = renderer.path(&current_dir);

        let expected = Some(format!(
            "{} ",
            nu_ansi_term::Style::new()
                .bold()
                .fg(nu_ansi_term::Color::Cyan)
                .paint("☁︎ myserver")
        ));

        assert_eq!(renderer.collect(), expected);
    }
}
