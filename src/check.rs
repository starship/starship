use crate::config::{get_palette, parse_style_string};
use crate::context::{Context, Properties, Target};
use nu_ansi_term::AnsiString;
use std::iter;

pub fn show_check(args: Properties) {
    let context = Context::new(args, Target::Main);

    let predefined_colors = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "purple",
        "cyan",
        "white",
        "bright-black",
        "bright-red",
        "bright-green",
        "bright-yellow",
        "bright-blue",
        "bright-purple",
        "bright-cyan",
        "bright-white",
    ];
    for color in build_color_table(&predefined_colors, None) {
        print!("{}", color);
    }
    print!("\n\n");

    if let Some(palette) = get_palette(
        &context.root_config.palettes,
        context.root_config.palette.as_deref(),
    ) {
        for color in build_color_table(&palette.keys().collect::<Vec<&String>>(), Some(&context)) {
            print!("{}", color);
        }
        print!("\n\n");
    }

    print!("{}", build_used_graphemes_line());
    print!("\n");
}

/// When printed, this iterator will display all permutations of colors
/// as fg and bg such that it they will generally fit on a single screen of
/// width 80 columns
fn build_color_table<'a, T: AsRef<str> + std::fmt::Display>(
    colors: &'a [T],
    context: Option<&'a Context>,
) -> impl Iterator<Item = AnsiString<'a>> + 'a {
    let maxw: usize = 16;
    let rown = 80 / maxw;
    colors
        .iter()
        .enumerate()
        .flat_map(move |(i, bg)| {
            colors.iter().enumerate().flat_map(move |(j, fg)| {
                if ((colors.len() * i) + j) % rown == 0 {
                    iter::once(AnsiString::from("\n"))
                } else {
                    iter::once(AnsiString::from(""))
                }
                .chain(iter::once(
                    parse_style_string(format!("fg:{fg} bg:{bg}").as_str(), context)
                        .unwrap()
                        .paint(format!("{: ^maxw$}", *fg)),
                ))
            })
        })
        .skip(1)
}

fn build_used_graphemes_line() -> AnsiString {
    AnsiString::from("\u{01f680}")
}
