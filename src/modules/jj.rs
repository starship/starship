use super::{Context, Module, ModuleConfig};
use crate::configs::jj::JJConfig;
use crate::formatter::string_formatter::StringFormatterError;
use crate::formatter::StringFormatter;
use std::borrow::Cow;

/// Creates a module with the jj status in the current directory
///
/// Will display the current branch, commit and change id of the jj repository in the current directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_status");
    let config = JJConfig::try_load(module.config);
    // check if the current directory is a jj repo
    let repo_root = context
        .exec_cmd("jj", &["root", "--ignore-working-copy"])?
        .stdout;

    let template = jj_template(config.truncation_length);
    let output = context.exec_cmd(
        "jj",
        &[
            "log",
            "-r",
            "@",
            "-T",
            template.as_str(),
            "--no-graph",
            "--ignore-working-copy",
            "--no-pager",
            "--color",
            "never",
            "-R",
            repo_root.trim(),
        ],
    )?;
    let parsed = JJStatus::new(&output.stdout)?;
    let formatter = match StringFormatter::new(config.format) {
        Ok(formatter) => formatter,
        Err(e) => {
            log::error!("Error in parsing format string for jj module {}", e);
            return None;
        }
    };
    let formatted = formatter
        .map_meta(|s, _| match s {
            "symbol" => Some(config.symbol),
            "divergent_symbol" => parsed
                .divergent
                .eq("true")
                .then_some(config.divergent_symbol),
            "no_description_symbol" => parsed
                .description
                .is_empty()
                .then_some(config.no_description_symbol),
            _ => None,
        })
        .map_style(|s| {
            Some(Ok(match s {
                "change_id_prefix_style" => config.change_id_prefix_style,
                "change_id_suffix_style" => config.change_id_suffix_style,
                "commit_id_prefix_style" => config.commit_id_prefix_style,
                "commit_id_suffix_style" => config.commit_id_suffix_style,
                "branch_style" => config.branch_style,
                _ => return None,
            }))
        })
        .map(|v| parsed.format(v))
        .parse(None, Some(context));
    let formatted = match formatted {
        Ok(formatted) => formatted,
        Err(e) => {
            log::error!("Error in parsing format string for jj module {}", e);
            return None;
        }
    };
    module.set_segments(formatted);

    Some(module)
}

fn jj_template(len: u8) -> String {
    format!(
        r##"change_id.shortest({len}).prefix() ++ "#," ++ change_id.shortest({len}).rest() ++ "#," ++ commit_id.shortest({len}).prefix() ++ "#," ++ commit_id.shortest({len}).rest() ++ "#," ++ divergent ++ "#," ++ description ++ "#," ++ branches.join(",")"##,
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct JJStatus<'s> {
    change_id_prefix: &'s str,
    change_id_suffix: &'s str,
    commit_id_prefix: &'s str,
    commit_id_suffix: &'s str,
    divergent: &'s str,
    description: &'s str,
    branches: &'s str,
}

impl<'s> JJStatus<'s> {
    fn new(input: &'s str) -> Option<JJStatus<'s>> {
        let expected_parts = jj_template(8).matches("#,").count() + 1;
        let parsed: Vec<&str> = input.split("#,").collect();
        if parsed.len() != expected_parts {
            log::error!(
                "expected at least {expected_parts} parts, got {}",
                parsed.len()
            );
            return None;
        }
        let [change_id_prefix, change_id_suffix, commit_id_prefix, commit_id_suffix, divergent, description, branches] =
            parsed[..]
        else {
            return None;
        };
        Some(Self {
            change_id_prefix,
            change_id_suffix,
            commit_id_prefix,
            commit_id_suffix,
            divergent,
            description,
            branches,
        })
    }

    fn format(&self, var: &str) -> Option<Result<Cow<str>, StringFormatterError>> {
        let Self {
            change_id_prefix,
            change_id_suffix,
            commit_id_prefix,
            commit_id_suffix,
            branches,
            description,
            ..
        } = self;
        let branch = if branches.is_empty() {
            "<no branch>"
        } else {
            branches
        };

        match var {
            "change_id_prefix" => Some(Ok(Cow::Borrowed(change_id_prefix))),
            "change_id_suffix" => Some(Ok(Cow::Borrowed(change_id_suffix))),
            "commit_id_prefix" => Some(Ok(Cow::Borrowed(commit_id_prefix))),
            "commit_id_suffix" => Some(Ok(Cow::Borrowed(commit_id_suffix))),
            "branch" => Some(Ok(Cow::Borrowed(branch))),
            "description" => Some(Ok(Cow::Borrowed(description))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    static PATH_TO_REPO: &'static str = "/path/to/repo";
    use crate::modules::jj::JJStatus;
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;

    fn jj_log(output: &str) -> (String, Option<CommandOutput>) {
        (
            format!(
                "jj log -r @ -T {} --no-graph --ignore-working-copy --no-pager --color never -R {PATH_TO_REPO}",
                crate::modules::jj::jj_template(8)
            ),
            Some(CommandOutput {
                stdout: output.to_string(),
                stderr: "".to_string(),
            }),
        )
    }

    fn is_jj_repo() -> &'static str {
        "jj root --ignore-working-copy"
    }

    #[test]
    fn not_a_jj_repo() {
        let actual = ModuleRenderer::new("jj_status")
            .cmd(is_jj_repo(), None)
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_actual_jj_repo() {
        let (mocked_jj_log_command, mocked_jj_log_output) =
            jj_log("kxx#,zlovo#,be9#,d1825#,false#,#,br1,br2");
        let repo_root = Some(CommandOutput {
            stdout: PATH_TO_REPO.to_string(),
            stderr: "".to_string(),
        });
        let actual = ModuleRenderer::new("jj_status")
            .cmd(is_jj_repo(), repo_root.clone())
            .cmd(mocked_jj_log_command.as_str(), mocked_jj_log_output)
            .collect()
            .unwrap();
        assert_eq!("[🍐 \u{1b}[35mkxx\u{1b}[90mzlovo\u{1b}[0m \u{1b}[34mbe9\u{1b}[90md1825\u{1b}[0m on \u{1b}[35mbr1,br2\u{1b}[0m 📝]", actual);
    }

    #[test]
    fn test_actual_jj_repo_with_divergence() {
        // Mock input to represent a JJ repository status with divergence
        let (mocked_jj_log_command, mocked_jj_log_output) =
            jj_log("kxx#,zlovo#,be9#,d1825#,true#,#,br1,br2");

        let repo_root = Some(CommandOutput {
            stdout: PATH_TO_REPO.to_string(),
            stderr: "".to_string(),
        });

        let actual = ModuleRenderer::new("jj_status")
            .cmd(is_jj_repo(), repo_root)
            .cmd(mocked_jj_log_command.as_str(), mocked_jj_log_output)
            .collect()
            .unwrap();
        assert_eq!("[🍐 \u{1b}[35mkxx\u{1b}[90mzlovo\u{1b}[0m \u{1b}[34mbe9\u{1b}[90md1825\u{1b}[0m on \u{1b}[35mbr1,br2\u{1b}[0m 📝 💥]", actual);
    }

    #[test]
    fn test_parser() {
        let input = "kxx#,zlovo#,be9#,d1825#,false#,#,br1,br2";
        let expected = JJStatus {
            change_id_prefix: "kxx",
            change_id_suffix: "zlovo",
            commit_id_prefix: "be9",
            commit_id_suffix: "d1825",
            divergent: "false",
            description: "",
            branches: "br1,br2",
        };
        let actual = JJStatus::new(input).unwrap();
        assert_eq!(expected, actual);

        let input = "kxx#,zlovo#,be9#,d1825#,false#,#,";
        let expected = JJStatus {
            change_id_prefix: "kxx",
            change_id_suffix: "zlovo",
            commit_id_prefix: "be9",
            commit_id_suffix: "d1825",
            divergent: "false",
            branches: "",
            description: "",
        };
        let actual = JJStatus::new(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_minimal_input() {
        let jj_status = JJStatus::new("a#,b#,c#,d#,true#,e#,f").unwrap();
        assert_eq!(jj_status.change_id_prefix, "a");
        assert_eq!(jj_status.change_id_suffix, "b");
        assert_eq!(jj_status.commit_id_prefix, "c");
        assert_eq!(jj_status.commit_id_suffix, "d");
        assert_eq!(jj_status.divergent, "true");
        assert_eq!(jj_status.description, "e");
        assert_eq!(jj_status.branches, "f");
    }

    #[test]
    fn test_no_branches() {
        let jj_status = JJStatus::new("a#,b#,c#,d#,false#,e#,").unwrap();
        let formatted_branch = jj_status.format("branch").unwrap().unwrap();
        assert_eq!(formatted_branch, "<no branch>");
    }

    #[test]
    fn test_incorrect_input_format() {
        let jj_status = JJStatus::new("a,b,c");
        assert!(jj_status.is_none());
    }

    #[test]
    fn test_multiple_branches() {
        let jj_status = JJStatus::new("1#,2#,3#,4#,false#,desc#,branch1,branch2").unwrap();
        let formatted_branch = jj_status.format("branch").unwrap().unwrap();
        assert_eq!(formatted_branch, "branch1,branch2");
    }

    #[test]
    fn test_divergent_true() {
        let jj_status = JJStatus::new("1#,2#,3#,4#,true#,desc#,branch").unwrap();
        assert_eq!(jj_status.divergent, "true");
    }
}
