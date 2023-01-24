use crate::config::parse_style_string;
use nu_ansi_term::AnsiString;
use std::iter;

pub fn show_check() {
    show_color_table();
    println!("");
    show_default_emoji();
    println!("");
    show_default_nerdfonts();
}

fn show_color_table() {
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
    for color in build_color_table(&predefined_colors) {
        println!("{}", color);
    }
}

/// When printed, this iterator will display all permutations of colors
/// as fg and bg such that it they will generally fit on a single screen of
/// width 80 columns
fn build_color_table<'a>(colors: &'a [&'a str]) -> impl Iterator<Item = AnsiString<'a>> {
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
                    parse_style_string(["fg:", fg, " ", "bg:", bg].join("").as_str(), None)
                        .unwrap()
                        .paint(format!("{: ^maxw$}", *fg)),
                ))
            })
        })
        .skip(1)
}

fn show_default_emoji() {
    println!("\u{01f680}");
}

fn show_default_nerdfonts() {
    println!("@_@");
}
