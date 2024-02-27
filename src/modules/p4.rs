use std::collections::HashMap;

use super::{Context, Module};

use crate::config::ModuleConfig;
use crate::configs::p4::P4Config;
use crate::formatter::StringFormatter;

/// Creates a module that displays Perforce repository currently in use
///
/// Will display the name of the current Perforce repository if one is active.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("p4");
    let config: P4Config = P4Config::try_load(module.config);

    if config.disabled {
        return None;
    }

    if !is_p4_logged(context) {
        log::debug!("User not logged in p4");
        return None;
    }

    log::debug!("User logged in p4");

    let info = get_p4_info(context)?;

    log::debug!("Got p4 info");

    if !context.current_dir.starts_with(&info.client_root) {
        log::debug!(
            "User outside p4 repository ({} outside {})",
            context.current_dir.display(),
            info.client_root
        );
        return None;
    }

    log::debug!(
        "User inside p4 repository ({} inside {})",
        context.current_dir.display(),
        info.client_root
    );

    let changelist = get_p4_last_changelist_number(context)?;

    log::debug!("Got p4 changelist number {}", changelist);

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
                "changelist" => Some(Ok(&changelist)),
                "user" => Some(Ok(&info.user_name)),
                "client" => Some(Ok(&info.client_name)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `p4`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

struct P4Info {
    user_name: String,
    client_name: String,
    client_root: String,
}

fn is_p4_logged(context: &Context) -> bool {
    context.exec_cmd("p4", &["login", "-s"]).is_some()
}

fn get_p4_info(context: &Context) -> Option<P4Info> {
    let info_output = context.exec_cmd("p4", &["info"])?.stdout;
    let info_map = parse_p4_info_output(&info_output);

    Some(P4Info {
        user_name: info_map.get("User name")?.to_string(),
        client_name: info_map.get("Client name")?.to_string(),
        client_root: info_map.get("Client root")?.to_string(),
    })
}

fn get_p4_last_changelist_number(context: &Context) -> Option<String> {
    context
        .exec_cmd("p4", &["changes", "-m1", "#have"])?
        .stdout
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}

fn parse_p4_info_output(output: &str) -> HashMap<&str, &str> {
    let mut info_map = HashMap::new();

    for line in output.lines() {
        if let Some((k, v)) = line.split_once(':') {
            info_map.insert(k, v.trim());
        };
    }

    info_map
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;

    use crate::{modules::p4::P4Info, test::ModuleRenderer, utils::CommandOutput};

    #[test]
    fn logged_deep_inside_p4_workspace() {
        let actual = ModuleRenderer::new("p4")
            .path(r"/home/human/p4/MyWorkspace/MyRepository/subdir")
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::Blue.bold().paint("p4 human@MyWorkspace#176579")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn logged_just_inside_p4_workspace() {
        let actual = ModuleRenderer::new("p4")
            .path(r"/home/human/p4/MyWorkspace")
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::Blue.bold().paint("p4 human@MyWorkspace#176579")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn not_logged_inside_p4_workspace() {
        let actual = ModuleRenderer::new("p4")
            .cmd("p4 login -s", None)
            .path(r"/home/human/p4/MyWorkspace/MyRepository")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn logged_outside_p4_workspace() {
        let actual = ModuleRenderer::new("p4")
            .path(r"/home/human/outside/perforce")
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn good_p4_username_and_workspace() {
        let user_name = "testusernamer";
        let client_name = "Workspace";
        let client_root = r"/home/human/p4/MyWorkspace";

        let info = P4Info {
            user_name: String::from(user_name),
            client_name: String::from(client_name),
            client_root: String::from(client_root),
        };

        let actual = ModuleRenderer::new("p4")
            .cmd("p4 info", build_mock_p4_info_output(&info))
            .path(client_root)
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::Blue
                .bold()
                .paint(format!("p4 {user_name}@{client_name}#176579"))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn good_p4_changelist_number() {
        let changelist = 12;
        let actual = ModuleRenderer::new("p4")
            .cmd(
                "p4 changes -m1 #have",
                build_mock_p4_changes_output(changelist),
            )
            .path(r"/home/human/p4/MyWorkspace/MyRepository")
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::Blue
                .bold()
                .paint(format!("p4 human@MyWorkspace#{changelist}"))
        ));

        assert_eq!(expected, actual);
    }

    fn build_mock_p4_changes_output(changelist: i32) -> Option<CommandOutput> {
        Some(CommandOutput {
            stdout: format!(
                "Change {changelist} on 2023/01/23 by human@MyWorkspace 'doing some work'\n"
            ),
            stderr: String::default(),
        })
    }

    fn build_mock_p4_info_output(info: &P4Info) -> Option<CommandOutput> {
        Some(CommandOutput {
            stdout: format!(
                r"\
User name: {}
Client name: {}
Client host: MyPC
Client root: {}
Current directory: /home/human
Peer address: 127.0.0.1:55855
Client address: 127.0.0.1
Server address: sc-helixa.test.com:1666
Server root: C:\Program Files\Perforce\Server
Server date: 2023/01/23 18:09:14 -0500 Eastern Standard Time
Server uptime: 661:14:26
Server version: P4D/NTX64/2020.1/1953492 (2020/04/24)
Server encryption: encrypted
Server cert expires: Jul  3 13:53:58 2024 GMT
Server license: University of Test 1000 users (support ends 2023/02/15) (expires 2023/02/15)
Server license-ip: 127.0.0.1:1666
Case Handling: insensitive
",
                info.user_name, info.client_name, info.client_root
            ),
            stderr: String::default(),
        })
    }
}
