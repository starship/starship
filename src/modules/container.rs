use super::{Context, Module};

#[cfg(not(target_os = "linux"))]
pub fn module<'a>(_context: &'a Context) -> Option<Module<'a>> {
    None
}

#[cfg(target_os = "linux")]
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    use super::ModuleConfig;
    use crate::configs::container::ContainerConfig;
    use crate::formatter::StringFormatter;
    use crate::utils::read_file;

    pub fn container_name(context: &Context) -> Option<String> {
        use crate::utils::context_path;

        if context_path(context, "/proc/vz").exists() && !context_path(context, "/proc/bc").exists()
        {
            // OpenVZ
            return Some("OpenVZ".into());
        }

        if context_path(context, "/run/host/container-manager").exists() {
            // OCI
            return Some("OCI".into());
        }

        if context_path(context, "/run/systemd/container").exists() {
            // systemd
            return Some("Systemd".into());
        }

        let container_env_path = context_path(context, "/run/.containerenv");

        if container_env_path.exists() {
            // podman and others

            let image_res = read_file(container_env_path)
                .map(|buf| {
                    buf.lines()
                        .find_map(|line| {
                            if config.use_container_name && line.starts_with("name") {
                                let (_, name) = line.split_once('=').unwrap_or(("", "podman"));
                                Some(String::from(name.trim_matches('"')))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| {
                            buf.lines()
                                .find_map(|line| {
                                    if line.starts_with("image=\"") {
                                        let (_, val) =
                                            line.split_once('=').unwrap_or(("", "podman"));
                                        let (_, name) = val.rsplit_once('/').unwrap_or(("", val));
                                        Some(String::from(name.trim_matches('"')))
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_else(|| "podman".into())
                        })
                })
                .unwrap_or_else(|_| "podman".into());

            return Some(image_res);
        }

        if context_path(context, "/.dockerenv").exists() {
            // docker
            return Some("Docker".into());
        }

        None
    }

    let mut module = context.new_module("container");
    let config: ContainerConfig = ContainerConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let container_name = container_name(context)?;

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
                "name" => Some(Ok(&container_name)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `container`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::path::PathBuf;

    #[test]
    fn test_none_if_disabled() {
        let expected = None;
        let actual = ModuleRenderer::new("container")
            // For a custom config
            .config(toml::toml! {
               [container]
               disabled = true
            })
            // Run the module and collect the output
            .collect();

        assert_eq!(expected, actual);
    }

    fn containerenv(
        container_name: Option<&str>,
        image_name: Option<&str>,
        use_con_name: bool,
    ) -> std::io::Result<(Option<String>, Option<String>)> {
        use std::io::Write;

        let renderer = ModuleRenderer::new("container")
            // For a custom config
            .config(toml::toml! {
               [container]
               disabled = false
            });

        let root_path = renderer.root_path();

        let mut containerenv = PathBuf::from(root_path);

        let mut display_name: Option<&str> = None;

        containerenv.push("run");
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&containerenv)?;

        containerenv.push(".containerenv");
        let mut file = std::fs::File::create(&containerenv)?;
        if let Some(name) = image_name {
            file.write_all(format!("image=\"{}\"\n", name).as_bytes())?;
            display_name = Some(name);
        }
        if let Some(name) = container_name {
            file.write_all(format!("name=\"{}\"\n", name).as_bytes())?;
            // Custom container name takes precedence
            display_name = Some(name);
        }

        // The output of the module
        let actual = renderer
            // Run the module and collect the output
            .collect();

        // The value that should be rendered by the module.
        let expected = Some(format!(
            "{} ",
            Color::Red
                .bold()
                .dimmed()
                .paint(format!("⬢ [{}]", display_name.unwrap_or("podman")))
        ));

        Ok((actual, expected))
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv() -> std::io::Result<()> {
        let (actual, expected) = containerenv(None, None)?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv_with_name() -> std::io::Result<()> {
        // Display container image name
        let (actual, expected) = containerenv(None, Some("fedora-toolbox:35"))?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        // Display custom container name
        let (actual, expected) =
            containerenv(Some("container-name"), Some("fedora-toolbox:35"), true)?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        // Display custom container name
        let (actual, expected) =
            containerenv(Some("container-name"), Some("fedora-toolbox:35"), false)?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);
        // Display custom container name
        let (actual, expected) = containerenv(Some("container-name"), None, true)?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn test_containerenv() -> std::io::Result<()> {
        let (actual, expected) = containerenv(None)?;

        // Assert that the actual and expected values are not the same
        assert_ne!(actual, expected);

        Ok(())
    }
}
