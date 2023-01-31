use crate::config::{get_palette, parse_style_string};
use crate::context::{Context, Properties, Target};
use crate::print::{compute_modules, UnicodeWidthGraphemes};
use crate::shadow;
use nu_ansi_term::{AnsiString, AnsiStrings, Style};
use std::cmp;
use std::collections::HashSet;
use std::iter;

pub fn show_check(text: Option<String>, mut colors: bool, palette: Option<String>, mut fonts: bool, user_style: Option<String>) {
    let context = &Context::new(Properties::default(), Target::Main);
    let extra_style = &user_style.map_or("".to_string(), |style| {
        parse_style_string(&style, Some(context)).map_or("".to_string(), move |_s| style)
    });
    if !colors && !fonts && text.is_none() {
        colors = true;
        fonts = true;
    }

    if colors {
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
                extra_style,
                context
            ))
        );
        let check_palette = palette.as_deref().or(context.root_config.palette.as_deref());

        if let Some(palette_table) = get_palette(
            &context.root_config.palettes,
            check_palette,
        )
        .map(|p| p.keys().collect::<Vec<&String>>())
        {
            println!(
                "{}\n",
                AnsiStrings(&build_color_table(&palette_table, extra_style, context))
            );
        }

        println!("{}\n", AnsiStrings(&build_style_line(extra_style, context)));
    };

    if fonts {
        let user_modules = build_user_module_line(context);
        let preset_modules = build_preset_module_line();
        let ansi_style = parse_style_string(extra_style, Some(context)).unwrap_or_else(Style::new);

        println!("{}\n", ansi_style.paint(filter_emoji(&user_modules)));
        println!("{}\n", ansi_style.paint(filter_emoji(&preset_modules)));

        println!("{}\n", ansi_style.paint(filter_nerdfonts(&user_modules)));
        println!("{}\n", ansi_style.paint(filter_nerdfonts(&preset_modules)));
    };

    if let Some(check_text) = text {
        println!("{}\n", parse_style_string(extra_style, Some(context)).unwrap_or_else(Style::new).paint(check_text));
    };
}

fn build_color_table<'a, T: AsRef<str> + std::fmt::Display>(
    colors: &'a [T],
    extra_style: &str,
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
    let extra_attr = &extra_style
        .split_whitespace()
        .filter(|style| {
            [
                "bold",
                "italic",
                "underline",
                "dimmed",
                "inverted",
                "blink",
                "hidden",
                "strikethrough",
            ]
            .iter()
            .any(|non_color| style == non_color)
        })
        .collect::<Vec<&str>>()
        .join(" ");
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
                    parse_style_string(
                        format!("{extra_attr} fg:{fg} bg:{bg}").as_str(),
                        Some(context),
                    )
                    .unwrap_or_else(|| panic!("Unprintable color found: {bg}"))
                    .paint(format!("{: ^maxw$}", *fg)),
                ))
            })
        })
        .collect::<Vec<AnsiString>>()
}

fn build_style_line<'a>(extra_style: &str, context: &'a Context<'a>) -> Vec<AnsiString<'a>> {
    let width = 13;
    let non_color_styles = [
        "bold",
        "italic",
        "underline",
        "dimmed",
        "inverted",
        "blink",
        "hidden",
        "strikethrough",
        "none",
    ];
    let extra_color = extra_style
        .split_whitespace()
        .filter(|style| !non_color_styles.iter().any(|non_color| style == non_color))
        .collect::<Vec<&str>>()
        .join(" ");
    non_color_styles
        .iter()
        .map(|s| {
            parse_style_string(&format!("{extra_color} {s}"), Some(context))
                .unwrap_or_else(Style::new)
                .paint(format!("{: ^width$}", *s))
        })
        .collect::<Vec<AnsiString>>()
}

fn build_user_module_line(context: &Context) -> String {
    Vec::from_iter(
        compute_modules(context)
            .iter()
            .flat_map(|m| m.segments.iter())
            .map(|s| s.value()),
    )
    .join("")
}

fn build_preset_module_line() -> String {
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
    fn user_styled_style_line() {
        let ctx = &Context::new(Properties::default(), Target::Main);

        let none_style_length = AnsiStrings(&build_style_line("none", ctx))
            .to_string()
            .len();
        let missing_style_length = AnsiStrings(&build_style_line("", ctx)).to_string().len();
        let bold_style_length = AnsiStrings(&build_style_line("bold", ctx))
            .to_string()
            .len();
        let color_style_length = AnsiStrings(&build_style_line("red", ctx)).to_string().len();

        assert_eq!(none_style_length, missing_style_length);
        assert_eq!(missing_style_length, bold_style_length);
        assert!(color_style_length > none_style_length);
    }

    #[test]
    fn color_table_short_name() {
        let cell_size = 4;
        let ctx = &Context::new(Properties::default(), Target::Main);
        let mut line_count = 0;

        let one_color_table = build_color_table(&["1"], "", ctx);
        for row in unstyle(&AnsiStrings(&one_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size);
            line_count += 1;
        }
        assert_eq!(line_count, 1);

        line_count = 0;
        let three_color_table = build_color_table(&["1", "2", "3"], "", ctx);
        for row in unstyle(&AnsiStrings(&three_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size * 3);
            line_count += 1;
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
            "",
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
            "",
            ctx,
        );
        for row in unstyle(&AnsiStrings(&wrapped_color_table)).lines() {
            assert_eq!(row.width_graphemes(), cell_size * 4);
            line_count += 1;
        }
        assert_eq!(line_count, 16);
    }

    #[test]
    fn user_styled_color_table() {
        let ctx = &Context::new(Properties::default(), Target::Main);

        let none_style_length = AnsiStrings(&build_color_table(&["blue"], "none", ctx))
            .to_string()
            .len();
        let missing_style_length = AnsiStrings(&build_color_table(&["blue"], "", ctx))
            .to_string()
            .len();
        let color_style_length = AnsiStrings(&build_color_table(&["blue"], "red", ctx))
            .to_string()
            .len();
        let bold_style_length = AnsiStrings(&build_color_table(&["blue"], "bold", ctx))
            .to_string()
            .len();

        assert_eq!(none_style_length, missing_style_length);
        assert_eq!(missing_style_length, color_style_length);
        assert!(bold_style_length > none_style_length);
    }

    #[test]
    fn no_colors() {
        let ctx = &Context::new(Properties::default(), Target::Main);
        let table = build_color_table::<String>(&[], "", ctx);
        assert_eq!(unstyle(&AnsiStrings(&table)), "");
    }

    #[test]
    #[should_panic]
    fn unprintable_color() {
        let ctx = &Context::new(Properties::default(), Target::Main);
        let _table = build_color_table(&["rainbow"], "", ctx);
    }

    #[test]
    fn module_lines() {
        let ctx = &Context::new(Properties::default(), Target::Main);
        assert!(build_preset_module_line().len() > 0);
        assert!(build_user_module_line(ctx).len() > 0);
    }
}
