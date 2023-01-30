use crate::config::{get_palette, parse_style_string};
use crate::context::{Context, Properties, Target};
use crate::print::{compute_modules, UnicodeWidthGraphemes};
use crate::shadow;
use nu_ansi_term::{AnsiString, AnsiStrings};
use std::cmp;
use std::collections::HashSet;
use std::iter;

pub fn show_check(args: Properties) {
    let context = &Context::new(args, Target::Main);

    println!(
        "{}\n",
        AnsiStrings(&build_color_table(
            &[
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
            ],
            context
        ))
    );

    if let Some(palette_table) = get_palette(
        &context.root_config.palettes,
        context.root_config.palette.as_deref(),
    )
    .map(|p| p.keys().collect::<Vec<&String>>())
    {
        println!(
            "{}\n",
            AnsiStrings(&build_color_table(&palette_table, context))
        );
    }

    let user_modules = build_user_module_output(context);
    let preset_modules = build_preset_module_output();

    println!("{}\n", filter_emoji(&user_modules));
    println!("{}\n", filter_emoji(&preset_modules));

    println!("{}\n", filter_nerdfonts(&user_modules));
    println!("{}\n", filter_nerdfonts(&preset_modules));
}

fn build_color_table<'a, T: AsRef<str> + std::fmt::Display>(
    colors: &'a [T],
    context: &'a Context,
) -> Vec<AnsiString<'a>> {
    if colors.is_empty() {
        return vec![AnsiString::from("")];
    }
    let maxw: usize = colors
        .iter()
        .fold(4, |w, c| cmp::max(w, c.width_graphemes()));
    // color table looks much better when only one bg is used per row
    let row_width = colors.len() / ((colors.len() * maxw + context.width - 1) / context.width);
    colors
        .iter()
        .enumerate()
        .flat_map(move |(i, bg)| {
            colors.iter().enumerate().flat_map(move |(j, fg)| {
                if ((colors.len() * i) + j) % row_width == 0 && (i != 0 || j != 0) {
                    iter::once(AnsiString::from("\n"))
                } else {
                    iter::once(AnsiString::from(""))
                }
                .chain(iter::once(
                    parse_style_string(format!("fg:{fg} bg:{bg}").as_str(), Some(context))
                        .unwrap_or_else(|| panic!("Unprintable color found: {bg}"))
                        .paint(format!("{: ^maxw$}", *fg)),
                ))
            })
        })
        .collect::<Vec<AnsiString>>()
}

fn build_user_module_output(context: &Context) -> String {
    Vec::from_iter(
        compute_modules(context)
            .iter()
            .flat_map(|m| m.segments.iter())
            .map(|s| s.value()),
    )
    .join("")
}

fn build_preset_module_output() -> String {
    String::from_utf8(
        shadow::get_preset_list()
            .iter()
            .flat_map(|m| shadow::get_preset_content(m.0).to_vec())
            .collect(),
    )
    .unwrap()
}

fn filter_emoji(input: &str) -> String {
    // These are some overly-broad, not exhaustive, ranges to guess at emoji
    // from https://www.unicode.org/Public/emoji/15.0/emoji-sequences.txt
    filter_graphemes(input, vec![[0x002139, 0x003299], [0x01F004, 0x01FAF8]])
}

fn filter_nerdfonts(input: &str) -> String {
    // The unicode private block used by nerd-fonts plus the few glyphs that fall outside
    // from https://github.com/ryanoasis/nerd-fonts/wiki/Glyph-Sets-and-Code-Points
    filter_graphemes(
        input,
        vec![
            [0x23FB, 0x23FE],
            [0x02665, 0x02665],
            [0x026A1, 0x026A1],
            [0x02B58, 0x02B58],
            [0x00E000, 0x00F8FF],
        ],
    )
}

fn filter_graphemes(possible_graphemes: &str, bounds: Vec<[u32; 2]>) -> String {
    let mut graphemes = Vec::from_iter::<HashSet<String>>(HashSet::from_iter(
        possible_graphemes
            .chars()
            .filter(|&c| {
                bounds
                    .iter()
                    .any(|[low, high]| low <= &(c as u32) && &(c as u32) <= high)
            })
            .map(|c| c.to_string()),
    ));
    graphemes.sort();
    graphemes.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;
    use nu_ansi_term::unstyle;

    #[test]
    fn filters() {
        let mixed_string = "Starship 🚀 the cross  shell 🐚 prompt  written ✍🏿 in rust ";
        assert_eq!(filter_nerdfonts(mixed_string), "\u{E795} \u{E7A8} \u{F657}");
        assert_eq!(
            filter_emoji(mixed_string),
            "\u{270D} \u{1F3FF} \u{1F41A} \u{1F680}"
        );
    }

    #[test]
    fn color_table_short_name() {
        let cell_size = 4;
        let ctx = &Context::new(Properties::default(), Target::Main);
        let mut line_count = 0;

        let one_color_table = build_color_table(&["1"], ctx);
        for row in unstyle(&AnsiStrings(&one_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size);
            line_count = line_count + 1;
        }
        assert_eq!(line_count, 1);

        line_count = 0;
        let three_color_table = build_color_table(&["1", "2", "3"], ctx);
        for row in unstyle(&AnsiStrings(&three_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size * 3);
            line_count = line_count + 1;
        }
        assert_eq!(line_count, 3);
    }

    #[test]
    fn color_table_variable_len_name() {
        let cell_size = 13;
        let ctx = &mut Context::new(Properties::default(), Target::Main);
        // test with constant-width terminal
        ctx.width = 80;
        let mut line_count = 0;

        let variable_color_table = build_color_table(
            &[
                "red",
                "green",
                "bright-yellow",
                "bright-blue",
                "bright-purple",
            ],
            ctx,
        );
        for row in unstyle(&AnsiStrings(&variable_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size * 5);
            line_count += 1;
        }
        assert_eq!(line_count, 5);

        line_count = 0;
        //enough cells that a line wrap is forced at 80 columns
        let wrapped_color_table = build_color_table(
            &[
                "bright-black",
                "bright-red",
                "bright-green",
                "bright-yellow",
                "bright-blue",
                "bright-purple",
                "bright-cyan",
                "bright-white",
            ],
            ctx,
        );
        for row in unstyle(&AnsiStrings(&wrapped_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size * 4);
            line_count += 1;
        }
        assert_eq!(line_count, 16);
    }

    #[test]
    fn no_colors() {
        let ctx = &Context::new(Properties::default(), Target::Main);
        let table = build_color_table::<String>(&[], ctx);
        assert_eq!(unstyle(&AnsiStrings(&table)), "");
    }

    #[test]
    #[should_panic]
    fn unprintable_color() {
        let ctx = &Context::new(Properties::default(), Target::Main);
        let _table = build_color_table(&["rainbow"], ctx);
    }
}
