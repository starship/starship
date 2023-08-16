use super::{Context, Module, ModuleConfig};

use super::utils::directory::truncate;
use crate::configs::spack::SpackConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Spack environment
///
/// Will display the Spack environment if `$SPACK_ENV` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let spack_env = context.get_env("SPACK_ENV").unwrap_or_default();
    if spack_env.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("spack");
    let config: SpackConfig = SpackConfig::try_load(module.config);

    let spack_env = truncate(&spack_env, config.truncation_length).unwrap_or(spack_env);

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
                "environment" => Some(Ok(spack_env.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `spack`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn not_in_env() {
        let actual = ModuleRenderer::new("spack").collect();

        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_set() {
        let actual = ModuleRenderer::new("spack")
            .env("SPACK_ENV", "astronauts")
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🅢 astronauts")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn truncate() {
        let actual = ModuleRenderer::new("spack")
            .env("SPACK_ENV", "/some/really/long/and/really/annoying/path/that/shouldnt/be/displayed/fully/spack/my_env")
            .collect();

        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🅢 my_env")));

        assert_eq!(expected, actual);
    }
}
