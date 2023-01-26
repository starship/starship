use crate::config::{get_palette, parse_style_string};
use crate::context::{Context, Properties, Target};
use crate::print::{compute_modules, UnicodeWidthGraphemes};
use crate::shadow;
use nu_ansi_term::AnsiString;
use std::cmp;
use std::collections::HashSet;
use std::iter;

pub fn show_check(args: Properties) {
    let context = &Context::new(args, Target::Main);

    println!("{}\n", build_predefined_color_table(context));

    if let Some(palette_table) = build_palette_color_table(context) {
        println!("{}\n", palette_table);
    }

    let user_modules = build_user_module_output(context);
    let preset_modules = build_preset_module_output();

    println!("{}\n", filter_emoji(&user_modules));
    println!("{}\n", filter_emoji(&preset_modules));

    println!("{}\n", filter_nerdfonts(&user_modules));
    println!("{}\n", filter_nerdfonts(&preset_modules));
}

fn build_predefined_color_table(context: &Context) -> String {
    build_color_table(
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
        context,
    )
}

fn build_palette_color_table(context: &Context) -> Option<String> {
    get_palette(
        &context.root_config.palettes,
        context.root_config.palette.as_deref(),
    )
    .map(|p| build_color_table(&p.keys().collect::<Vec<&String>>(), context))
}

fn build_color_table<T: AsRef<str> + std::fmt::Display>(colors: &[T], context: &Context) -> String {
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
                if ((colors.len() * i) + j) % row_width == 0 {
                    iter::once(AnsiString::from("\n"))
                } else {
                    iter::once(AnsiString::from(""))
                }
                .chain(iter::once(
                    parse_style_string(format!("fg:{fg} bg:{bg}").as_str(), Some(context))
                        .unwrap()
                        .paint(format!("{: ^maxw$}", *fg)),
                ))
            })
        })
        .map(|ansi| ansi.to_string())
        .skip(1)
        .collect::<Vec<String>>()
        .join("")
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
    Vec::from_iter::<HashSet<String>>(HashSet::from_iter(
        possible_graphemes
            .chars()
            .filter(|&c| {
                bounds
                    .iter()
                    .any(|[low, high]| low <= &(c as u32) && &(c as u32) <= high)
            })
            .map(|c| c.to_string()),
    ))
    .join(" ")
}
