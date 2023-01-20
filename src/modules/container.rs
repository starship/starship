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
    use crate::utils::{self, read_file};

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

        let container_env_path = context_path(context, "/run/.containerenv");

        if container_env_path.exists() {
            // podman and others

            let image_res = read_file(container_env_path)
                .map(|s| {
                    s.lines()
                        .find_map(|l| {
                            l.starts_with("image=\"").then(|| {
                                let r = l.split_at(7).1;
                                let name = r.rfind('/').map(|n| r.split_at(n + 1).1);
                                String::from(name.unwrap_or(r).trim_end_matches('"'))
                            })
                        })
                        .unwrap_or_else(|| "podman".into())
                })
                .unwrap_or_else(|_| "podman".into());

            return Some(image_res);
        }

        // WSL with systemd will set the contents of this file to "wsl"
        // Avoid showing the container module in that case
        let systemd_path = context_path(context, "/run/systemd/container");
        if systemd_path.exists() {
            let systemd_name = match utils::read_file(systemd_path).ok() {
                Some(s) => match s.trim() {
                    "docker" => Some("Docker".into()),
                    "wsl" => None,
                    _ => Some("Systemd".into()),
                },
                _ => None,
            };

            if systemd_name.is_some() {
                // Systemd or Docker
                return systemd_name;
            }
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
    use crate::utils;
    use nu_ansi_term::Color;
    use std::fs;

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

    fn containerenv(name: Option<&str>) -> std::io::Result<(Option<String>, Option<String>)> {
        let renderer = ModuleRenderer::new("container")
            // For a custom config
            .config(toml::toml! {
               [container]
               disabled = false
            });

        let root_path = renderer.root_path();

        // simulate file found on ubuntu images to ensure podman containerenv is preferred
        let systemd_path = root_path.join("run/systemd/container");

        fs::create_dir_all(systemd_path.parent().unwrap())?;
        utils::write_file(&systemd_path, "docker\n")?;

        let containerenv = root_path.join("run/.containerenv");

        fs::create_dir_all(containerenv.parent().unwrap())?;

        let contents = match name {
            Some(name) => format!("image=\"{name}\"\n"),
            None => String::new(),
        };
        utils::write_file(&containerenv, contents)?;

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
                .paint(format!("⬢ [{}]", name.unwrap_or("podman")))
        ));

        Ok((actual, expected))
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv() -> std::io::Result<()> {
        let (actual, expected) = containerenv(None)?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv_fedora() -> std::io::Result<()> {
        let (actual, expected) = containerenv(Some("fedora-toolbox:35"))?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn containerenv_systemd(
        name: Option<&str>,
        display: Option<&str>,
    ) -> std::io::Result<(Option<String>, Option<String>)> {
        let renderer = ModuleRenderer::new("container")
            // For a custom config
            .config(toml::toml! {
               [container]
               disabled = false
            });

        let root_path = renderer.root_path();

        let systemd_path = root_path.join("run/systemd/container");

        fs::create_dir_all(systemd_path.parent().unwrap())?;

        let contents = match name {
            Some(name) => format!("{name}\n"),
            None => "systemd-nspawn\n".to_string(),
        };
        utils::write_file(&systemd_path, contents)?;

        // The output of the module
        let actual = renderer
            // Run the module and collect the output
            .collect();

        // The value that should be rendered by the module.
        let expected = display.map(|_| {
            format!(
                "{} ",
                Color::Red
                    .bold()
                    .dimmed()
                    .paint(format!("⬢ [{}]", display.unwrap_or("Systemd")))
            )
        });

        Ok((actual, expected))
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv_systemd() -> std::io::Result<()> {
        let (actual, expected) = containerenv_systemd(None, Some("Systemd"))?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv_docker_in_systemd() -> std::io::Result<()> {
        let (actual, expected) = containerenv_systemd(Some("docker"), Some("Docker"))?;

        // Assert that the actual and expected values are the same
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_containerenv_wsl_in_systemd() -> std::io::Result<()> {
        let (actual, expected) = containerenv_systemd(Some("wsl"), None)?;

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
