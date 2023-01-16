use super::{Context, Module, ModuleConfig};

use crate::configs::fossil_branch::FossilBranchConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::truncate::truncate_text;

/// Creates a module with the Fossil branch of the check-out in the current directory
///
/// Will display the branch name if the current directory is a Fossil check-out
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_checkout = context
        .try_begin_scan()?
        .set_files(&[".fslckout"])
        .is_match();

    if !is_checkout {
        return None;
    }

    let mut module = context.new_module("fossil_branch");
    let config = FossilBranchConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

    let truncated_branch_name = {
        let output = context.exec_cmd("fossil", &["branch", "current"])?.stdout;
        truncate_text(output.trim(), len, config.truncation_symbol)
    };

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
                "branch" => Some(Ok(truncated_branch_name.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `fossil_branch`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::test::ModuleRenderer;

    #[test]
    fn show_nothing_on_empty_dir() -> io::Result<()> {
        let checkout_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("fossil_branch")
            .path(checkout_dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        checkout_dir.close()
    }
}
