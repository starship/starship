#![feature(pin)]
use ansi_term::ANSIStrings;
use clap::ArgMatches;
use futures::prelude::*;
use futures_util::{
    future::FutureExt,
    stream::{self, StreamExt},
};
use pin_utils::pin_mut;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use unicode_width::UnicodeWidthChar;

use crate::context::{Context, Shell};
use crate::module::Module;
use crate::module::ALL_MODULES;
use crate::modules;

pub async fn prompt(args: ArgMatches<'_>) {
    let context = Context::new(args);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", get_prompt(&context).await).unwrap();
}

pub async fn get_prompt<'a>(context: &'a Context<'_>) -> String {
    let config = context.config.get_root_config();
    let mut buf = String::new();

    // Write a new line before the prompt
    if config.add_newline {
        writeln!(buf).unwrap();
    }

    // A workaround for a fish bug (see #739,#279). Applying it to all shells
    // breaks things (see #808,#824,#834). Should only be printed in fish.
    if let Shell::Fish = context.shell {
        buf.push_str("\x1b[J"); // An ASCII control code to clear screen
    }

    let mut print_without_prefix = true;

    let list = compute_modules(&context);
    pin_mut!(list);
    while let Some(x) = list.next().await {
        if let Some(module) = x {
            if print_without_prefix {
                let module_without_prefix = module.to_string_without_prefix(context.shell.clone());
                write!(buf, "{}", module_without_prefix).unwrap();
            } else {
                let module = module.ansi_strings_for_shell(context.shell.clone());
                write!(buf, "{}", ANSIStrings(&module)).unwrap();
            }
            print_without_prefix = module.get_name() == "line_break";
        }
    }
    buf
}
pub async fn module(module_name: &str, args: ArgMatches<'_>) {
    let context = Context::new(args);
    let module = get_module(module_name, context).await.unwrap_or_default();
    print!("{}", module);
}

pub async fn get_module(module_name: &str, context: Context<'_>) -> Option<String> {
    modules::handle(module_name, &context)
        .await
        .map(|m| m.to_string())
}

pub async fn explain(args: ArgMatches<'_>) {
    let context = Context::new(args);

    struct ModuleInfo {
        value: String,
        value_len: usize,
        desc: String,
    }

    let dont_print = vec!["line_break", "character"];

    let computed = compute_modules(&context)
        .collect::<Vec<Option<Module>>>()
        .await;

    let modules = computed
        .into_iter()
        .filter_map(|x| x)
        .filter(|module| !dont_print.contains(&module.get_name().as_str()))
        .map(|module| {
            let ansi_strings = module.ansi_strings();
            let value = module.get_segments().join("");
            ModuleInfo {
                value: ansi_term::ANSIStrings(&ansi_strings[1..ansi_strings.len() - 1]).to_string(),
                value_len: value.chars().count() + count_wide_chars(&value),
                desc: module.get_description().to_owned(),
            }
        })
        .collect::<Vec<ModuleInfo>>();

    let mut max_ansi_module_width = 0;
    let mut max_module_width = 0;

    for info in &modules {
        max_ansi_module_width = std::cmp::max(
            max_ansi_module_width,
            info.value.chars().count() + count_wide_chars(&info.value),
        );
        max_module_width = std::cmp::max(max_module_width, info.value_len);
    }

    let desc_width = term_size::dimensions()
        .map(|(w, _)| w)
        .map(|width| width - std::cmp::min(width, max_ansi_module_width));

    println!("\n Here's a breakdown of your prompt:");
    for info in modules {
        let wide_chars = count_wide_chars(&info.value);

        if let Some(desc_width) = desc_width {
            let wrapped = textwrap::fill(&info.desc, desc_width);
            let mut lines = wrapped.split('\n');
            println!(
                " {:width$}  -  {}",
                info.value,
                lines.next().unwrap(),
                width = max_ansi_module_width - wide_chars
            );

            for line in lines {
                println!("{}{}", " ".repeat(max_module_width + 6), line.trim());
            }
        } else {
            println!(
                " {:width$}  -  {}",
                info.value,
                info.desc,
                width = max_ansi_module_width - wide_chars
            );
        };
    }
}

fn compute_modules<'a>(
    context: &'a Context,
) -> impl futures_util::stream::Stream<Item = Option<Module<'a>>> {
    let mut prompt_order: Vec<&str> = Vec::new();

    let handle = |v| modules::handle(v, &context);
    let isDisabled = |module| !context.is_module_disabled_in_config(module);

    // Write out a custom prompt order
    for module in context.config.get_root_config().prompt_order {
        if ALL_MODULES.contains(&module) {
            prompt_order.push(module);
        } else {
            log::debug!(
                "Expected prompt_order to contain value from {:?}. Instead received {}",
                ALL_MODULES,
                module,
            );
        }
    }

    futures_util::stream::unfold(
        prompt_order.into_iter().filter(|x| isDisabled(x)),
        |items| {
            async move {
                match items.next() {
                    Some(v) => Some((handle(v).await, items)),
                    _ => None,
                }
            }
        },
    )
}

fn count_wide_chars(value: &str) -> usize {
    value.chars().filter(|c| c.width().unwrap_or(0) > 1).count()
}
