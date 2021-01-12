use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TaskWarriorConfig<'a> {
    pub format: &'a str,
    pub current_task_style: &'a str,
    pub no_tasks_symbol: &'a str,
    pub no_tasks_symbol_style: &'a str,
    pub today_symbol: &'a str,
    pub today_symbol_style: &'a str,
    pub week_symbol: &'a str,
    pub week_symbol_style: &'a str,
    pub overdue_symbol: &'a str,
    pub overdue_symbol_style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TaskWarriorConfig<'a> {
    fn new() -> Self {
        TaskWarriorConfig {
            format: "[$symbol]($symbol_style)[$current_task]($current_task_style) ",
            current_task_style: "white",
            no_tasks_symbol: "✓",
            no_tasks_symbol_style: "green",
            today_symbol: "⚡",
            today_symbol_style: "cyan",
            week_symbol: "📅",
            week_symbol_style: "green",
            overdue_symbol: "❌",
            overdue_symbol_style: "bold red",
            disabled: true,
        }
    }
}
