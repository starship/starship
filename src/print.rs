use crate::modules;
use crate::modules::Segment;
use clap::ArgMatches;

pub fn prompt(args: ArgMatches) {
    let default_prompt = vec!["dir", "line_sep", "char"];

    default_prompt.into_iter()
        .map(|module| modules::handle(module, &args))
        .map(|segment| stringify_segment(segment))
        .for_each(|segment_string| print!("{}", segment_string));
}

pub fn stringify_segment(segment: Segment) -> String {
    let Segment {
        prefix,
        value,
        style,
        suffix,
    } = segment;    

    let mut segment_string = String::new();

    if let Some(prefix) = prefix {
        segment_string += &stringify_segment(*prefix);
    }
    
    segment_string += &style.paint(value).to_string();

    if let Some(suffix) = suffix {
        segment_string += &stringify_segment(*suffix);
    }

    segment_string
}
