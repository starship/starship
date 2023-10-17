use super::{Context, Module, ModuleConfig};

use crate::configs::cuda::CudaConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use std::ops::Deref;

const NVCC_MATCH_PATTERN: &str = r"release\s(?P<version>\d+\.\d+),";
const SMI_MATCH_PATTERN: &str = r"CUDA Version\s+:\s+(?P<version>\d+\.\d+)";

fn parse_cuda_version(s: &str) -> Option<String> {
    return [NVCC_MATCH_PATTERN, SMI_MATCH_PATTERN]
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .map(|re| re.captures(s))
        .fold(None, |acc, caps| {
            acc.or(caps
                .map(|caps| caps.name("version").map(|v| v.as_str().to_string()))
                .flatten())
        });
}

/// Creates a module with the current C compiler and version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cuda");
    let config: CudaConfig = CudaConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        // The cuda compiler or driver info
        let cuda_info = Lazy::new(|| context.exec_cmds_return_first(config.commands));

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
                "version" => {
                    let cuda_info = &cuda_info.deref().as_ref()?.stdout;
                    let parsed_info = parse_cuda_version(cuda_info)?;

                    // nvcc --version says ...
                    //   nvcc: NVIDIA (R) Cuda compiler driver
                    //   Copyright (c) 2005-2022 NVIDIA Corporation
                    //   Built on Thu_Feb_10_18:23:41_PST_2022
                    //   Cuda compilation tools, release 11.6, V11.6.112
                    //   Build cuda_11.6.r11.6/compiler.30978841_0
                    // We are trying to get the word after release

                    // nvidia-smi --query says ...
                    //   Timestamp                                 : Mon Oct 16 01:11:08 2023
                    //   Driver Version                            : 525.125.06
                    //   CUDA Version                              : 12.0
                    // We'd like to find the line with CUDA Version
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &parsed_info,
                        config.version_format,
                    )
                    .map(Cow::Owned)
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `c`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::io;

    const NVCC_SUCCESS_OUTPUT: &str = "\
nvcc: NVIDIA (R) Cuda compiler driver
Copyright (c) 2005-2022 NVIDIA Corporation
Built on Thu_Feb_10_18:23:41_PST_2022
Cuda compilation tools, release 11.6, V11.6.112
Build cuda_11.6.r11.6/compiler.30978841_0";

    const NVIDIA_SMI_SUCCESS_OUTPUT: &str = "\
==============NVSMI LOG==============

Timestamp                                 : Tue Oct 17 11:17:13 2023
Driver Version                            : 525.125.06
CUDA Version                              : 12.0

Attached GPUs                             : 1
GPU 00000000:01:00.0
    Product Name                          : NVIDIA GeForce RTX 3090
    Product Brand                         : GeForce
    Product Architecture                  : Ampere
    Display Mode                          : Disabled
    Display Active                        : Disabled
    Persistence Mode                      : Disabled
    MIG Mode
        Current                           : N/A
        Pending                           : N/A
    Accounting Mode                       : Disabled
    Accounting Mode Buffer Size           : 4000
    Driver Model
        Current                           : N/A
        Pending                           : N/A
    Serial Number                         : N/A
    GPU UUID                              : GPU-58f16df2-6eee-bfac-bdc1-13e0091441b5
    Minor Number                          : 0
    VBIOS Version                         : 94.02.42.80.9F
    MultiGPU Board                        : No
    Board ID                              : 0x100
    Board Part Number                     : N/A
    GPU Part Number                       : 2204-300-A1
    Module ID                             : 1
    Inforom Version
        Image Version                     : G001.0000.03.03
        OEM Object                        : 2.0
        ECC Object                        : N/A
        Power Management Object           : N/A
    GPU Operation Mode
        Current                           : N/A
        Pending                           : N/A
    GSP Firmware Version                  : N/A
    GPU Virtualization Mode
        Virtualization Mode               : None
        Host VGPU Mode                    : N/A
    IBMNPU
        Relaxed Ordering Mode             : N/A
    PCI
        Bus                               : 0x01
        Device                            : 0x00
        Domain                            : 0x0000
        Device Id                         : 0x220410DE
        Bus Id                            : 00000000:01:00.0
        Sub System Id                     : 0x161319DA
        GPU Link Info
            PCIe Generation
                Max                       : 4
                Current                   : 4
                Device Current            : 4
                Device Max                : 4
                Host Max                  : 4
            Link Width
                Max                       : 16x
                Current                   : 16x
        Bridge Chip
            Type                          : N/A
            Firmware                      : N/A
        Replays Since Reset               : 0
        Replay Number Rollovers           : 0
        Tx Throughput                     : 1829000 KB/s
        Rx Throughput                     : 12000 KB/s
        Atomic Caps Inbound               : N/A
        Atomic Caps Outbound              : N/A
    Fan Speed                             : 70 %
    Performance State                     : P3
    Clocks Throttle Reasons
        Idle                              : Active
        Applications Clocks Setting       : Not Active
        SW Power Cap                      : Not Active
        HW Slowdown                       : Not Active
            HW Thermal Slowdown           : Not Active
            HW Power Brake Slowdown       : Not Active
        Sync Boost                        : Not Active
        SW Thermal Slowdown               : Not Active
        Display Clock Setting             : Not Active
    FB Memory Usage
        Total                             : 24576 MiB
        Reserved                          : 316 MiB
        Used                              : 1788 MiB
        Free                              : 22471 MiB
    BAR1 Memory Usage
        Total                             : 256 MiB
        Used                              : 5 MiB
        Free                              : 251 MiB
    Compute Mode                          : Default
    Utilization
        Gpu                               : 35 %
        Memory                            : 7 %
        Encoder                           : 0 %
        Decoder                           : 0 %
    Encoder Stats
        Active Sessions                   : 0
        Average FPS                       : 0
        Average Latency                   : 0
    FBC Stats
        Active Sessions                   : 0
        Average FPS                       : 0
        Average Latency                   : 0
    Ecc Mode
        Current                           : N/A
        Pending                           : N/A
    ECC Errors
        Volatile
            SRAM Correctable              : N/A
            SRAM Uncorrectable            : N/A
            DRAM Correctable              : N/A
            DRAM Uncorrectable            : N/A
        Aggregate
            SRAM Correctable              : N/A
            SRAM Uncorrectable            : N/A
            DRAM Correctable              : N/A
            DRAM Uncorrectable            : N/A
    Retired Pages
        Single Bit ECC                    : N/A
        Double Bit ECC                    : N/A
        Pending Page Blacklist            : N/A
    Remapped Rows                         : N/A
    Temperature
        GPU Current Temp                  : 58 C
        GPU T.Limit Temp                  : N/A
        GPU Shutdown Temp                 : 98 C
        GPU Slowdown Temp                 : 95 C
        GPU Max Operating Temp            : 93 C
        GPU Target Temperature            : 83 C
        Memory Current Temp               : N/A
        Memory Max Operating Temp         : N/A
    Power Readings
        Power Management                  : Supported
        Power Draw                        : 83.48 W
        Power Limit                       : 350.00 W
        Default Power Limit               : 350.00 W
        Enforced Power Limit              : 350.00 W
        Min Power Limit                   : 100.00 W
        Max Power Limit                   : 385.00 W
    Clocks
        Graphics                          : 810 MHz
        SM                                : 810 MHz
        Memory                            : 5001 MHz
        Video                             : 945 MHz
    Applications Clocks
        Graphics                          : N/A
        Memory                            : N/A
    Default Applications Clocks
        Graphics                          : N/A
        Memory                            : N/A
    Deferred Clocks
        Memory                            : N/A
    Max Clocks
        Graphics                          : 2115 MHz
        SM                                : 2115 MHz
        Memory                            : 9751 MHz
        Video                             : 1950 MHz
    Max Customer Boost Clocks
        Graphics                          : N/A
    Clock Policy
        Auto Boost                        : N/A
        Auto Boost Default                : N/A
    Voltage
        Graphics                          : 725.000 mV
    Fabric
        State                             : N/A
        Status                            : N/A
    Processes
        GPU instance ID                   : N/A
        Compute instance ID               : N/A
        Process ID                        : 1624
            Type                          : G
            Name                          : /usr/lib/xorg/Xorg
            Used GPU Memory               : 630 MiB";

    const NVCC_NOT_FOUND_OUTPUT: &str = "\
zsh: command not found: nvcc
";

    const NVIDIA_SMI_NOT_FOUND_OUTPUT: &str = "\
zsh: command not found: nvidia-smi
";
    const NVIDIA_SMI_ERROR_OUTPUT: &str = "\
NVIDIA-SMI has failed because it couldn't communicate with the NVIDIA driver. Make sure that the latest NVIDIA driver is installed and running.";

    #[test]
    fn nvcc_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvcc --version",
                Some(CommandOutput {
                    stdout: String::from(NVCC_SUCCESS_OUTPUT),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(118, 185, 0).bold().paint("NV v11.6")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn nvidia_smi_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvidia-smi --query",
                Some(CommandOutput {
                    stdout: String::from(NVIDIA_SMI_SUCCESS_OUTPUT),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(118, 185, 0).bold().paint("NV v12.0")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_nvcc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvcc --version",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::from(NVCC_NOT_FOUND_OUTPUT),
                }),
            )
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(118, 185, 0).bold().paint("NV ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn nvidia_smi_error() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvidia-smi --query",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::from(NVIDIA_SMI_ERROR_OUTPUT),
                }),
            )
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(118, 185, 0).bold().paint("NV ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_nvidia_smi() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvidia-smi --query",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::from(NVIDIA_SMI_NOT_FOUND_OUTPUT),
                }),
            )
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(118, 185, 0).bold().paint("NV ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
}
