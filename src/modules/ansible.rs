use super::{Context, Module, RootModuleConfig};
use crate::configs::ansible::AnsibleConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current Ansible version.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("ansible");
    let config = AnsibleConfig::try_load(module.config);

    let is_ansible_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_ansible_project {
        return None;
    }

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
                "version" => {
                    let ansible_version = parse_ansible_version(
                        &context.exec_cmd("ansible", &["--version"])?.stdout,
                    )?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &ansible_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `ansible`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_ansible_version(ansible_stdout: &str) -> Option<String> {
    // ansible version output looks like this:
    // ansible [core X.Y.Z]
    //   config file = /config/file/ansible.cfg
    //   configured module search path = /path/to/modules
    //   ansible python module location = /path/to/ansible/python
    //   ansible collection location = /path/to/ansible/collections
    //   executable location = /path/to/ansible
    //   python version = x.y.z
    //   jinja version = a.b.c
    //   libyaml = True

    return Some(
        ansible_stdout
            .split_whitespace()
            .nth(2)
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .to_string(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_ansible_cfg() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("ansible").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ansible_cfg() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("ansible.cfg"))?.sync_all()?;

        let actual = ModuleRenderer::new("ansible").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::White.bold().paint("Ⓐ v2.1.3")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_ansible_version() {
        let input = "\
ansible [core 1.2.3]
    config file = /config/file/ansible.cfg
    configured module search path = /path/to/modules
    ansible python module location = /path/to/ansible/python
    ansible collection location = /path/to/ansible/collections
    executable location = /path/to/ansible
    python version = 3.10.1
    jinja version = 3.2.1
    libyaml = True";

        assert_eq!(
            parse_ansible_version(input),
            Some("1.2.3".to_string())
        );
    }
}