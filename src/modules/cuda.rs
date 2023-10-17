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

    #[test]
    fn nvcc_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cuda")
            .cmd(
                "nvcc --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
nvcc: NVIDIA (R) Cuda compiler driver
Copyright (c) 2005-2022 NVIDIA Corporation
Built on Thu_Feb_10_18:23:41_PST_2022
Cuda compilation tools, release 11.6, V11.6.112
Build cuda_11.6.r11.6/compiler.30978841_0",
                    ),
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
}
