use super::{Context, Module, ModuleConfig};
#[cfg(not(test))]
use mysql::Pool;
#[cfg(not(test))]
use mysql::prelude::Queryable;
use pretty_duration::pretty_duration;
use std::time::Duration;

use crate::configs::mysql::MySQLConfig;
use crate::configs::mysql::Symbol;
use crate::formatter::StringFormatter;

#[derive(Debug, Clone)]
struct MySQLInfo {
    is_running: bool,
    is_replica: bool,
    io_thread_state: String,
    sql_thread_state: String,
    replication_lag_s: f64,
}

/// Creates a module for MySQL instance
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("mysql");
    let config: MySQLConfig = MySQLConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let mysql_info = get_mysql_info(config.url);

    let mut service_symbol = config.get_symbol(&Symbol::Running);
    let mut service_style = config.style_ok;
    let mut io_symbol = config.get_symbol(&Symbol::Running);
    let mut io_style = config.style_ok;
    let mut sql_symbol = config.get_symbol(&Symbol::Running);
    let mut sql_style = config.style_ok;
    let mut lag_style = config.style_ok;

    if !mysql_info.is_running {
        service_symbol = config.get_symbol(&Symbol::Stopped);
        service_style = config.style_error;
        io_symbol = config.get_symbol(&Symbol::NotApplicable);
        io_style = config.style_not_applicable;
        sql_symbol = config.get_symbol(&Symbol::NotApplicable);
        sql_style = config.style_not_applicable;
    } else if mysql_info.is_replica {
        if mysql_info.io_thread_state != "ON" {
            io_symbol = config.get_symbol(&Symbol::Stopped);
            io_style = config.style_error;
        }
        if mysql_info.sql_thread_state != "ON" {
            sql_symbol = config.get_symbol(&Symbol::Stopped);
            sql_style = config.style_error;
        }
    } else {
        io_symbol = config.get_symbol(&Symbol::NotApplicable);
        io_style = config.style_not_applicable;
        sql_symbol = config.get_symbol(&Symbol::NotApplicable);
        sql_style = config.style_not_applicable;
    }

    if mysql_info.replication_lag_s > config.lag_threshold_s {
        lag_style = config.style_error;
    }

    let lag_str = pretty_duration(&Duration::from_secs_f64(mysql_info.replication_lag_s), None);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => config.get_symbol(&Symbol::Database),
                _ => None,
            })
            .map(|variable| match variable {
                "up" => service_symbol.map(Ok),
                "io" => io_symbol.map(Ok),
                "sql" => sql_symbol.map(Ok),
                "lag" => match mysql_info.is_replica {
                    true => Option::from(lag_str.as_str()).map(Ok),
                    false => None,
                },
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "up_style" => Some(Ok(service_style)),
                "io_style" => Some(Ok(io_style)),
                "sql_style" => Some(Ok(sql_style)),
                "lag_style" => Some(Ok(lag_style)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `mysql`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(not(test))]
fn get_mysql_info(url: &str) -> MySQLInfo {
    let pool = Pool::new(url);
    if pool.is_err() {
        return MySQLInfo {
            is_running: false,
            is_replica: false,
            io_thread_state: String::from(""),
            sql_thread_state: String::from(""),
            replication_lag_s: 0.0,
        };
    }

    let conn = pool.unwrap().get_conn();
    if conn.is_err() {
        return MySQLInfo {
            is_running: false,
            is_replica: false,
            io_thread_state: String::from(""),
            sql_thread_state: String::from(""),
            replication_lag_s: 0.0,
        };
    }

    let query = "
WITH IO_T AS (
    SELECT
        rcs.SERVICE_STATE AS io_thread_state
    FROM
        performance_schema.replication_connection_status rcs
    WHERE
        rcs.CHANNEL_NAME = ''
), SQL_T AS (
    SELECT
        ras.SERVICE_STATE AS sql_thread_state
    FROM
        performance_schema.replication_applier_status_by_coordinator ras
    WHERE
        ras.CHANNEL_NAME = ''
    LIMIT 1
), REPL_LAG AS (
    SELECT
        ras.LAST_APPLIED_TRANSACTION_END_APPLY_TIMESTAMP - ras.LAST_APPLIED_TRANSACTION_ORIGINAL_COMMIT_TIMESTAMP AS delay
    FROM
        performance_schema.replication_applier_status_by_worker ras
    WHERE
        ras.CHANNEL_NAME = ''
        AND ras.LAST_APPLIED_TRANSACTION <> ''
    ORDER BY
        ras.LAST_APPLIED_TRANSACTION_END_APPLY_TIMESTAMP DESC
    LIMIT 1
)
SELECT
    IFNULL(delay, 0) AS delay,
    io_thread_state,
    sql_thread_state
FROM
    IO_T
    LEFT OUTER JOIN SQL_T ON 1 = 1
    LEFT OUTER JOIN REPL_LAG ON 1 = 1
;
";

    let res = conn.unwrap().query_map(
        query,
        |(replication_lag_s, io_thread_state, sql_thread_state)| {
            let info = MySQLInfo {
                sql_thread_state,
                is_replica: false,
                io_thread_state,
                replication_lag_s,
                is_running: true,
            };
            return info;
        },
    );

    match res {
        Ok(_) => {
            let infos = res.unwrap();
            if !infos.is_empty() {
                let mut result = infos[0].clone();
                result.is_replica = true;
                result
            } else {
                MySQLInfo {
                    is_running: true,
                    is_replica: false,
                    io_thread_state: String::from(""),
                    sql_thread_state: String::from(""),
                    replication_lag_s: 0.0,
                }
            }
        }
        Err(_) => MySQLInfo {
            is_running: false,
            is_replica: false,
            io_thread_state: String::from(""),
            sql_thread_state: String::from(""),
            replication_lag_s: 0.0,
        },
    }
}

#[cfg(test)]
const URI_SOURCE: &str = "mysql://user@source/";
#[cfg(test)]
const URI_REPLICA_OK: &str = "mysql://user@replica_ok/";
#[cfg(test)]
const URI_REPLICA_IO_NOT_OK: &str = "mysql://user@replica_io_not_ok/";
#[cfg(test)]
const URI_REPLICA_SQL_NOT_OK: &str = "mysql://user@replica_sql_not_ok/";
#[cfg(test)]
const URI_REPLICA_LAGGING: &str = "mysql://user@replica_lagging/";

#[cfg(test)]
fn get_mysql_info(url: &str) -> MySQLInfo {
    match url {
        URI_SOURCE => MySQLInfo {
            is_running: true,
            is_replica: false,
            io_thread_state: "".to_string(),
            sql_thread_state: "".to_string(),
            replication_lag_s: 0.0,
        },
        URI_REPLICA_OK => MySQLInfo {
            is_running: true,
            is_replica: true,
            io_thread_state: "ON".to_string(),
            sql_thread_state: "ON".to_string(),
            replication_lag_s: 5.0,
        },
        URI_REPLICA_IO_NOT_OK => MySQLInfo {
            is_running: true,
            is_replica: true,
            io_thread_state: "OFF".to_string(),
            sql_thread_state: "ON".to_string(),
            replication_lag_s: 5.0,
        },
        URI_REPLICA_SQL_NOT_OK => MySQLInfo {
            is_running: true,
            is_replica: true,
            io_thread_state: "ON".to_string(),
            sql_thread_state: "OFF".to_string(),
            replication_lag_s: 5.0,
        },
        URI_REPLICA_LAGGING => MySQLInfo {
            is_running: true,
            is_replica: true,
            io_thread_state: "ON".to_string(),
            sql_thread_state: "ON".to_string(),
            replication_lag_s: 15.0,
        },
        _ => MySQLInfo {
            is_running: false,
            is_replica: false,
            io_thread_state: "".to_string(),
            sql_thread_state: "".to_string(),
            replication_lag_s: 0.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::{AnsiString, AnsiStrings, Color};
    #[test]
    fn disabled() {
        let actual = ModuleRenderer::new("mysql").collect();
        assert_eq!(actual, None)
    }

    #[test]
    fn enabled_source() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_SOURCE
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("●"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("●"),
            AnsiString::from(" "),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }
    #[test]
    fn enabled_replica_ok() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_REPLICA_OK
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            AnsiString::from(" "),
            Color::Green.bold().paint("5s"),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }
    #[test]
    fn enabled_replica_io_not_ok() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_REPLICA_IO_NOT_OK
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Red.bold().paint("■"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            AnsiString::from(" "),
            Color::Green.bold().paint("5s"),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }
    #[test]
    fn enabled_replica_sql_not_ok() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_REPLICA_SQL_NOT_OK
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Red.bold().paint("■"),
            AnsiString::from(" "),
            Color::Green.bold().paint("5s"),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }

    #[test]
    fn enabled_replica_lagging() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_REPLICA_LAGGING
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            Color::White.bold().paint("/"),
            Color::Green.bold().paint("▶"),
            AnsiString::from(" "),
            Color::Red.bold().paint("15s"),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }
    #[test]
    fn enabled_mysql_down() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = "mysql://root@mysqldown"
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🐬 "),
            Color::Red.bold().paint("■"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("●"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("●"),
            AnsiString::from(" "),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }

    #[test]
    fn enabled_source_custom_symbols() {
        let actual = ModuleRenderer::new("mysql")
            .config(toml::toml! {
                [mysql]
                disabled = false
                url = URI_SOURCE
                [mysql.symbols]
                Database = "🛢 "
                Running = "⏵"
                Stopped = "⏹"
                NotApplicable = "ⁿ̷ₐ"
            })
            .collect();

        let expected: &[AnsiString<'static>] = &[
            Color::White.bold().paint("🛢 "),
            Color::Green.bold().paint("⏵"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("ⁿ̷ₐ"),
            Color::White.bold().paint("/"),
            Color::White.bold().paint("ⁿ̷ₐ"),
            AnsiString::from(" "),
        ];

        assert_eq!(actual, Option::from(AnsiStrings(expected).to_string()));
    }
}
