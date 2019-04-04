mod char;

use crate::Segment;

pub fn handle(module: &str) -> Segment {
    match module {
        "char" => char::segment(),

        _ => panic!("Unknown module: {}", module),
    }
}
