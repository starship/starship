use super::core::{LogLevel, Message};

pub const DEPRECATED_USE_FORMAT: Message = Message {
    message: "Starship uses \"format\" key to customize the appearance of modules from v0.41.0, \
              Check https://starship.rs for more info on updating your config.",
    level: LogLevel::Warning,
};
