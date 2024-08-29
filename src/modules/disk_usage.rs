use std::{
    error::Error,
    path::{Path, PathBuf},
};

use systemstat::{saturating_sub_bytes, Filesystem, Platform, System};

use super::{Context, Module};

use crate::segment::Segment;
use crate::{config::ModuleConfig, configs::disk_usage::DiskUsedConfig};
use crate::{formatter::StringFormatter, modules::memory_usage::display_bs};

fn get_disk_name(disk: &Filesystem) -> Option<&str> {
    Path::new(&disk.fs_mounted_on)
        .file_name()
        .and_then(|name| name.to_str())
}

fn format_disk_usage(
    disk: &Filesystem,
    config: &DiskUsedConfig,
    show_disk_name: bool,
    add_separator: bool,
    context: &Context,
) -> Result<Vec<Segment>, Box<dyn Error>> {
    let used_space = saturating_sub_bytes(disk.total, disk.avail);
    let total_space = disk.total;

    let used_percentage = used_space.as_u64() as f64 / total_space.as_u64() as f64 * 100f64;

    let formatted_usage = if config.show_percentage {
        format!("{:.2}%", used_percentage)
    } else {
        format!("{}/{}", display_bs(used_space), display_bs(disk.total))
    };

    let threshold_config = config
        .threshold_styles
        .iter()
        .find(|threshold_style| used_percentage >= (threshold_style.threshold as f64));

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
            .parse(None, Some(context))
    });

    match parsed {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(Box::new(e)),
    }
}

fn should_display_disk(disk: &Filesystem, threshold: i64) -> bool {
    let used = saturating_sub_bytes(disk.total, disk.avail);
    let percentage = used.as_u64() as f64 / disk.total.as_u64() as f64;
    percentage >= (threshold as f64 * 0.01f64)
}

fn get_drive_from_path<'a>(path: &PathBuf, disks: &'a [Filesystem]) -> Option<&'a Filesystem> {
    disks
        .iter()
        .find(|disk| path.starts_with(&disk.fs_mounted_on))
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("disk_usage");
    let config = DiskUsedConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let mut current_disk = None;
    let mut current_storage = None;
    let mut other_storage = None;
    let system = System::new();
    let all_disks = match system.mounts().ok() {
        Some(disks) => disks,
        None => {
            log::warn!("Failed to get disk information");
            return None;
        }
    };

    if let Some(threshold) = config.current_threshold {
        match get_drive_from_path(&context.current_dir, &all_disks) {
            Some(disk) => {
                current_disk = Some(disk);
                if should_display_disk(disk, threshold) {
                    match format_disk_usage(disk, &config, config.show_current_name, false, context)
                    {
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
            None => {
                log::warn!("Couldn't get disk from current path");
            }
        }
    }

    if let Some(threshold) = config.all_threshold {
        let display_disks: Vec<_> = all_disks
            .iter()
            .filter(|disk| match current_disk {
                Some(current) => disk.fs_mounted_on != current.fs_mounted_on,
                None => true,
            })
            .filter(|disk| should_display_disk(disk, threshold))
            .collect();
        let mut all_segments = Vec::new();

        for (i, disk) in display_disks.iter().enumerate() {
            match format_disk_usage(disk, &config, true, display_disks.len() != i + 1, context) {
                Ok(ref mut segments) => {
                    all_segments.append(segments);
                }
                Err(e) => log::warn!("Couldn't format disk {}: {}", disk.fs_mounted_on, e),
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
            .parse(None, Some(context))
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
