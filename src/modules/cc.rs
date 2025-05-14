use super::{Context, Module, ModuleConfig};

use crate::configs::c::{CConfig, CConfigMarker};
use crate::configs::cc::CcConfig;
use crate::configs::cpp::{CppConfig, CppConfigMarker};
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use crate::segment::Segment;

use semver::Version;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::LazyLock;

pub enum Lang {
    C,
    Cpp,
}

fn is_cc_project<T>(config: &CcConfig<T>, context: &Context) -> Option<bool> {
    Some(
        context
            .try_begin_scan()?
            .set_extensions(&config.detect_extensions)
            .set_files(&config.detect_files)
            .set_folders(&config.detect_folders)
            .is_match(),
    )
}

fn parse_module<T>(
    context: &Context,
    module: &mut Module,
    config: CcConfig<T>,
    compilers: [(&str, &str); 2],
) -> Result<Vec<Segment>, crate::formatter::string_formatter::StringFormatterError> {
    StringFormatter::new(config.format).and_then(|formatter| {
        let cc_compiler_info = LazyLock::new(|| context.exec_cmds_return_first(config.commands));

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
                "name" => {
                    let cc_compiler_info = &cc_compiler_info.deref().as_ref()?.stdout;

                    let cc_compiler =
                        compilers
                            .iter()
                            .find_map(|(compiler_name, compiler_hint)| {
                                cc_compiler_info
                                    .contains(compiler_hint)
                                    .then_some(*compiler_name)
                            })?;
                    Some(Ok(Cow::Borrowed(cc_compiler)))
                }
                "version" => {
                    let cc_compiler_info = &cc_compiler_info.deref().as_ref()?.stdout;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        cc_compiler_info
                            .split_whitespace()
                            .find(|word| Version::parse(word).is_ok())?,
                        config.version_format,
                    )
                    .map(Cow::Owned)
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    })
}

fn create_module<'a, T>(
    context: &'a Context,
    lang: &str,
    compilers: [(&str, &str); 2],
    mut module: Module<'a>,
    config: CcConfig<T>,
) -> Option<Module<'a>> {
    if config.disabled {
        return None;
    }

    if !is_cc_project(&config, context)? {
        return None;
    }

    let parsed = parse_module(context, &mut module, config, compilers);

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cc` for `lang: {}` :\n{}", lang, error);
            return None;
        }
    });

    Some(module)
}

pub fn module<'a>(context: &'a Context, lang: Lang) -> Option<Module<'a>> {
    match lang {
        Lang::C => {
            let lang = "c";
            let compilers = [("clang", "clang"), ("gcc", "Free Software Foundation")];
            let module = context.new_module(lang);
            let config = CConfig::try_load(module.config);

            create_module::<CConfigMarker>(context, lang, compilers, module, config)
        }
        Lang::Cpp => {
            let lang = "cpp";
            let compilers = [("clang++", "clang"), ("g++", "Free Software Foundation")];
            let module = context.new_module(lang);
            let config = CppConfig::try_load(module.config);

            create_module::<CppConfigMarker>(context, lang, compilers, module, config)
        }
    }
}
