use super::{Context, Module};
use crate::config::RootModuleConfig;
use crate::configs::task_warrior::TaskWarriorConfig;
use crate::formatter::StringFormatter;
use crate::utils;
use std::path::Path;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    if !task_warrior_installed() {
        log::debug!("Task warrior is not installed");
        return None;
    }

    let mut module = context.new_module("task_warrior");
    let config: TaskWarriorConfig = TaskWarriorConfig::try_load(module.config);
    if config.disabled {
        return None;
    }

    let current_task = get_current_task();

    let mut symbol = String::from(config.no_tasks_symbol);
    let mut symbol_style = String::from(config.no_tasks_symbol_style);

    let result = utils::exec_cmd("task", &["+OVERDUE"]);
    if result.is_some() {
        symbol = String::from(config.overdue_symbol);
        symbol_style = String::from(config.overdue_symbol_style);
    } else {
        let result = utils::exec_cmd("task", &["+DUETODAY"]);
        if result.is_some() {
            symbol = String::from(config.today_symbol);
            symbol_style = String::from(config.today_symbol_style);
        } else {
            let result = utils::exec_cmd("task", &["+WEEK"]);
            if result.is_some() {
                symbol = String::from(config.week_symbol);
                symbol_style = String::from(config.week_symbol_style);
            }
        }
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "symbol_style" => Some(Ok(symbol_style.as_str())),
                "current_task_style" => Some(Ok(config.current_task_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(&symbol)),
                "current_task" => match &current_task {
                    Some(task) => Some(Ok(&task)),
                    None => None,
                },
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `task warrior`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn task_warrior_installed() -> bool {
    let home_dir = dirs_next::home_dir().unwrap();
    let task_dir = Path::new(&home_dir).join(".task");
    task_dir.exists()
}

fn get_current_task() -> Option<String> {
    let result = utils::exec_cmd("task", &["+ACTIVE", "uuids"])?;
    let uuid = result.stdout.lines().next()?.split(' ').next()?;
    let arg = String::from(uuid) + ".description";
    let result = utils::exec_cmd("task", &["_get", arg.as_str()])?;
    Some(String::from(result.stdout.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use std::io;

    #[test]
    fn disabled_by_default() -> io::Result<()> {
        let expected = None;

        let actual = ModuleRenderer::new("task_warrior").collect();
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_current_task() {
        let expected = String::from("Task Warrior test task");
        let actual = get_current_task().unwrap();
        assert_eq!(expected, actual);
    }
}
