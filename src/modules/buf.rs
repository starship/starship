use super::{Context, Module, RootModuleConfig};

use crate::configs::buf::BufConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
  let mut module = context.new_module("buf");
  let config: BufConfig = BufConfig::try_load(module.config);

  let is_buf_project = context
    .try_begin_scan()?
    .set_files(&config.detect_files)
    .set_extensions(&config.detect_extensions)
    .set_folders(&config.detect_folders)
    .is_match();

  if !is_buf_project {
    return None;
  }

  let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let buf_version = parse_buf_version(
                        &context.exec_cmd("buf", &["--version"])?.stdout,
                    )?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &buf_version,
                        config.version_format,
                    )
                }
                .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `buf`:\n{}", error);
            return None;
        }
    });

  Some(module)
}

fn parse_buf_version(buf_version: &str) -> Option<String> {
    Some(
        buf_version
            .split_whitespace()
            // return "1.0.0"
            .nth(0)?
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
  use crate::test::ModuleRenderer;
  use ansi_term::Color;
  use std::fs::File;
  use std::io;

  #[test]
  fn buf_not_installed() {
    let actual = ModuleRenderer::new("buf").collect();
    let expected = None;
    assert_eq!(expected, actual);
  }
}
