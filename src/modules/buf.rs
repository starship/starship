use super::{Context, Module, ModuleConfig};
use os_info;
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
                    let buf_version =
                        parse_buf_version(&context.exec_cmd("buf", &["--version"])?.stdout)?;
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
    Some(buf_version.split_whitespace().next()?.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_buf_version;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn buf_version() {
        let ok_versions = ["1.0.0", "1.1.0-dev"];
        let not_ok_versions = ["foo", "1.0"];

        let all_some = ok_versions.iter().all(|&v| parse_buf_version(v).is_some());
        let all_none = not_ok_versions
            .iter()
            .any(|&v| parse_buf_version(v).is_some());

        assert!(all_some);
        assert!(all_none);
    }

    #[test]
    fn folder_without_buf_config() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("buf").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_buf_config() {
        let ok_files = ["buf.yaml", "buf.gen.yaml", "buf.work.yaml"];
        let not_ok_files = ["buf.json"];

        for file in ok_files {
            let dir = tempfile::tempdir().unwrap();
            File::create(dir.path().join(file))
                .unwrap()
                .sync_all()
                .unwrap();
            let actual = ModuleRenderer::new("buf").path(dir.path()).collect();

            if cfg!(windows) {

                let version_str = os_info::get().version().to_string();
                let mut nums_of_version = version_str.split(".");

                // Gets either version 11 or not
                let version = nums_of_version.next().unwrap().parse::<i32>().unwrap();

                if version != 11 {
                    let expected = Some(format!("with {}", Color::Blue.bold().paint("🐃 v1.0.0 ")));
                    assert_eq!(expected, actual);
                }else{
                    let expected = Some(format!("with {}", Color::Blue.bold().paint("🦬 v1.0.0 ")));
                    assert_eq!(expected, actual);
                }


            }else{
                let expected = Some(format!("with {}", Color::Blue.bold().paint("🦬 v1.0.0 ")));
                assert_eq!(expected, actual);
            }

            dir.close().unwrap();
        }

        for file in not_ok_files {
            let dir = tempfile::tempdir().unwrap();
            File::create(dir.path().join(file))
                .unwrap()
                .sync_all()
                .unwrap();
            let actual = ModuleRenderer::new("buf").path(dir.path()).collect();
            let expected = None;
            assert_eq!(expected, actual);
            dir.close().unwrap();
        }
    }
}
