use crate::modules;
use crate::Segment;

pub fn prompt() {
    let default_prompt = vec!["char"];

    for module in default_prompt {
        let segment = modules::handle(module);
        print_segment(segment);
    }
}

fn print_segment(segment: Segment) {
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
