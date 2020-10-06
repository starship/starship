use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

use byte_unit::{Byte, ByteError, ByteUnit};
use sysinfo::{Disk, DiskExt, RefreshKind, System, SystemExt};

use super::{Context, Module, RootModuleConfig, Shell};

use crate::configs::disk_used::DiskUsedConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;

fn format_byte(n_byte: f64) -> Result<String, ByteError> {
    let byte = Byte::from_unit(n_byte, ByteUnit::B)?;
    let mut display_bytes = byte.get_appropriate_unit(false).format(0);
    display_bytes.retain(|c| c != ' ');
    Ok(display_bytes)
}

fn get_disk_name(disk: &'_ Disk) -> Option<&'_ str> {
    let full_disk_name = disk.get_name();
    let disk_name = Path::new(full_disk_name).file_name();
    match disk_name {
        Some(disk_name) => disk_name.to_str(),
        None => full_disk_name.to_str(),
    }
}

fn format_disk_used(
    disk: &Disk,
    config: &DiskUsedConfig,
    show_disk_name: bool,
    add_separator: bool,
    percentage_char: &'_ str,
) -> Result<Vec<Segment>, Box<dyn Error>> {
    let used_space = (disk.get_total_space() - disk.get_available_space()) as f64;
    let total_space = disk.get_total_space() as f64;

    let formatted_usage = if config.show_percentage {
        format!(
            "{:.2}{}",
            (used_space / total_space) * 100f64,
            percentage_char
        )
    } else {
        format!("{}/{}", format_byte(used_space)?, format_byte(total_space)?)
    };

    let threshold_config = config.threshold_styles.iter().find(|threshold_style| {
        (used_space / total_space) >= (threshold_style.threshold as f64 * 0.01f64)
    });

    let style = match threshold_config {
        Some(threshold_config) => threshold_config.style,
        None => config.default_style,
    };

    let parsed = StringFormatter::new(
        "([$name:]($default_style))[$usage]($style)([$separator]($default_style))",
    )
    .and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(style)),
                "default_style" => Some(Ok(config.default_style)),
                _ => None,
            })
            .map(|var| match var {
                "name" if show_disk_name => match get_disk_name(disk) {
                    Some(name) => Some(Ok(name)),
                    _ => None,
                },
                "usage" => Some(Ok(formatted_usage.as_str())),
                "separator" if add_separator => Some(Ok(config.separator)),
                _ => None,
            })
            .parse(None)
    });

    match parsed {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(Box::new(e)),
    }
}

fn should_display_disk(disk: &Disk, threshold: i64) -> bool {
    let percentage = (disk.get_total_space() - disk.get_available_space()) as f64
        / disk.get_total_space() as f64;

    percentage >= (threshold as f64 * 0.01f64)
}

#[derive(Debug)]
struct DiskNotFoundError {
    path: PathBuf,
}
impl DiskNotFoundError {
    fn new(path: PathBuf) -> Self {
        DiskNotFoundError { path }
    }
}
impl fmt::Display for DiskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No disk found for path: {:?}!", self.path)
    }
}
impl Error for DiskNotFoundError {}

#[cfg(target_os = "windows")]
fn get_drive_from_path<'a>(path: &PathBuf, disks: &'a [Disk]) -> Result<&'a Disk, Box<dyn Error>> {
    use std::{ffi::OsString, io::Error, iter::once, os::windows::prelude::*};
    use winapi::um::fileapi::GetVolumePathNameW;

    let path: Vec<u16> = path.as_os_str().encode_wide().chain(once(0)).collect();
    let mut volume_path_name: Vec<u16> = vec![0; 260];

    let ret = unsafe {
        GetVolumePathNameW(
            path.as_ptr(),
            volume_path_name.as_mut_ptr(),
            volume_path_name.len() as u32,
        )
    };

    if ret != 0 {
        // Strip out the nulls from the end
        while let Some(last) = volume_path_name.last() {
            if *last == 0 {
                volume_path_name.pop();
            } else {
                break;
            }
        }
        let volume_path_name = OsString::from_wide(&volume_path_name);
        log::info!("Got path name: {:?}", volume_path_name);
        for disk in disks {
            if volume_path_name == disk.get_mount_point() {
                return Ok(disk);
            }
        }
        Err(Box::new(DiskNotFoundError::new(PathBuf::from(
            volume_path_name,
        ))))
    } else {
        Err(Box::new(Error::last_os_error()))
    }
}

#[cfg(any(target_os = "unix", target_os = "macos"))]
fn get_drive_from_path<'a>(path: &PathBuf, disks: &'a [Disk]) -> Result<&'a Disk, Box<dyn Error>> {
    use std::{fs, os::unix::fs::MetadataExt};

    let meta = fs::metadata(path)?;
    let dev_id = meta.dev();

    for disk in disks {
        let disk_meta = fs::metadata(disk.get_name())?;
        if disk_meta.rdev() == dev_id {
            return Ok(disk);
        }
    }

    Err(Box::new(DiskNotFoundError::new(path.to_owned())))
}

/// Uses the device id to get the drive
#[cfg(target_os = "linux")]
fn get_drive_from_path<'a>(path: &PathBuf, disks: &'a [Disk]) -> Result<&'a Disk, Box<dyn Error>> {
    use std::{fs, os::linux::fs::MetadataExt};

    let meta = fs::metadata(path)?;
    let dev_id = meta.st_dev();

    for disk in disks {
        let disk_meta = fs::metadata(disk.get_name())?;
        if disk_meta.st_rdev() == dev_id {
            return Ok(disk);
        }
    }

    Err(Box::new(DiskNotFoundError::new(path.to_owned())))
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("disk_used");
    let config = DiskUsedConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let percentage_char = match context.shell {
        Shell::Zsh => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        _ => "%",
    };

    let mut current_disk = None;
    let mut current_storage = None;
    let mut other_storage: Option<Vec<Segment>> = None;
    let system = System::new_with_specifics(RefreshKind::new().with_disks().with_disks_list());
    let all_disks = system.get_disks();

    if let Some(threshold) = config.current_threshold {
        match get_drive_from_path(&context.current_dir, all_disks) {
            Ok(disk) => {
                current_disk = Some(disk);
                if should_display_disk(disk, threshold) {
                    match format_disk_used(
                        &disk,
                        &config,
                        config.show_current_name,
                        false,
                        percentage_char,
                    ) {
                        Ok(segments) => {
                            if !segments.is_empty() {
                                current_storage = Some(segments);
                            }
                        }
                        Err(e) => {
                            log::warn!("Couldn't format disk from current path: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Couldn't get disk from current path: {}", e);
            }
        }
    }

    if let Some(threshold) = config.all_threshold {
        let display_disks: Vec<_> = all_disks
            .iter()
            .filter(|disk| match current_disk {
                Some(current) => disk.get_name() != current.get_name(),
                None => true,
            })
            .filter(|disk| should_display_disk(disk, threshold))
            .collect();
        let mut all_segments = Vec::new();

        for (i, disk) in display_disks.iter().enumerate() {
            match format_disk_used(
                &disk,
                &config,
                true,
                display_disks.len() != i + 1,
                percentage_char,
            ) {
                Ok(ref mut segments) => {
                    all_segments.append(segments);
                }
                Err(e) => match get_disk_name(&disk) {
                    Some(name) => log::warn!("Couldn't format disk {}: {}", name, e),
                    None => log::warn!("Couldn't get disk name or do formatting: {}", e),
                },
            }
        }

        if !all_segments.is_empty() {
            other_storage = Some(all_segments);
        }
    }

    // If there is no storage data return None
    if current_storage.is_none() && other_storage.is_none() {
        return None;
    }
    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|mvar, _| match mvar {
                "symbol" => Some(config.symbol),
                "prefix" => Some(config.prefix),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.default_style)),
                _ => None,
            })
            .map_variables_to_segments(|var| match var {
                "current_storage" => match &current_storage {
                    Some(current_storage) => Some(Ok(current_storage.to_vec())),
                    None => None,
                },
                "other_storage" => match &other_storage {
                    Some(other_storage) => Some(Ok(other_storage.to_vec())),
                    None => None,
                },
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `storage`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
