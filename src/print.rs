use crate::modules;
use crate::modules::Segment;
use clap::ArgMatches;

pub fn prompt(args: ArgMatches) {
    let default_prompt = vec!["dir", "char"];

    for module in default_prompt {
        let segment = modules::handle(module, &args);
        print_segment(segment);
    }
}

pub fn print_segment(segment: Segment) {
    let Segment {
        prefix,
        value,
        style,
        suffix,
    } = segment;

    if let Some(prefix) = prefix {
        print_segment(*prefix);
    }

    print!("{}", style.paint(value));

    if let Some(suffix) = suffix {
        print_segment(*suffix);
    }
}
