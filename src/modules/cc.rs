use super::{Context, Module, ModuleConfig};

use serde::{de::value::Error as ValueError, de::Error as SerdeError};

use crate::configs::c::CConfig;
use crate::configs::cc::LangConfig;
use crate::configs::cpp::CppConfig;
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

trait CommonConfig<'a, E>: ModuleConfig<'a, E> + LangConfig<'a>
where
    Self: Default + Sync,
    E: SerdeError,
{
    fn is_cc_project(config: &Self, context: &'a Context) -> Option<bool> {
        Some(
            context
                .try_begin_scan()?
                .set_extensions(config.detect_extensions())
                .set_files(config.detect_files())
                .set_folders(config.detect_folders())
                .is_match(),
        )
    }

    fn parse_module(
        context: &'a Context,
        module: &mut Module<'a>,
        config: Self,
        compilers: [(&str, &str); 2],
    ) -> Result<Vec<Segment>, crate::formatter::string_formatter::StringFormatterError> {
        StringFormatter::new(config.format()).and_then(|formatter| {
            let cc_compiler_info =
                LazyLock::new(|| context.exec_cmds_return_first(config.commands().clone()));

            formatter
                .map_meta(|var, _| match var {
                    "symbol" => Some(config.symbol()),
                    _ => None,
                })
                .map_style(|variable| match variable {
                    "style" => Some(Ok(config.style())),
                    _ => None,
                })
                .map(|variable| match variable {
                    "name" => {
                        let cc_compiler_info = &cc_compiler_info.deref().as_ref()?.stdout;

                        let cc_compiler = 'search: {
                            for (compiler_name, compiler_hint) in compilers {
                                if cc_compiler_info.contains(compiler_hint) {
                                    break 'search Some(compiler_name);
                                }
                            }
                            None
                        };
                        Some(Ok(Cow::Borrowed(cc_compiler?)))
                    }
                    "version" => {
                        let cc_compiler_info = &cc_compiler_info.deref().as_ref()?.stdout;

                        VersionFormatter::format_module_version(
                            module.get_name(),
                            cc_compiler_info
                                .split_whitespace()
                                .find(|word| Version::parse(word).is_ok())?,
                            config.version_format(),
                        )
                        .map(Cow::Owned)
                        .map(Ok)
                    }
                    _ => None,
                })
                .parse(None, Some(context))
        })
    }
}

impl<'a> CommonConfig<'a, ValueError> for CConfig<'a> {}
impl<'a> CommonConfig<'a, ValueError> for CppConfig<'a> {}

fn create_module<'a, T: CommonConfig<'a, ValueError>>(
    context: &'a Context,
    lang: &str,
    compilers: [(&str, &str); 2],
) -> Option<Module<'a>> {
    let mut module = context.new_module(lang);
    let config: T = T::try_load(module.config);

    if !T::is_cc_project(&config, context)? {
        return None;
    }

    let parsed = T::parse_module(context, &mut module, config, compilers);

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
        Lang::C => create_module::<CConfig>(
            context,
            "c",
            [("clang", "clang"), ("gcc", "Free Software Foundation")],
        ),
        Lang::Cpp => create_module::<CppConfig>(
            context,
            "cpp",
            [("clang++", "clang"), ("g++", "Free Software Foundation")],
        ),
    }
}
