#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use clap::{App, Arg};
    use starship::{modules, print};
    use test::Bencher;

    // #[bench]
    // fn full_prompt_bench(b: &mut Bencher) {
    //     b.iter(|| {
    //         let args = App::new("starship")
    //             .arg(Arg::with_name("status_code"))
    //             .get_matches_from(vec!["starship", "0"]);

    //         starship::print::prompt(args)
    //     });
    // }

    #[bench]
    fn char_section_bench(b: &mut Bencher) {
        b.iter(|| {
            let args = App::new("starship")
                .arg(Arg::with_name("status_code"))
                .get_matches_from(vec!["starship", "0"]);

            let segment = modules::handle("char", &args);
            print::stringify_segment(segment)
        });
    }

    #[bench]
    fn dir_section_bench(b: &mut Bencher) {
        b.iter(|| {
            let args = App::new("starship")
                .arg(Arg::with_name("status_code"))
                .get_matches_from(vec!["starship", "0"]);

            let segment = modules::handle("dir", &args);
            print::stringify_segment(segment)
        });
    }

        #[bench]
    fn line_break_section_bench(b: &mut Bencher) {
        b.iter(|| {
            let args = App::new("starship")
                .arg(Arg::with_name("status_code"))
                .get_matches_from(vec!["starship", "0"]);

            let segment = modules::handle("line_break", &args);
            print::stringify_segment(segment)
        });
    }

        #[bench]
    fn nodejs_section_bench(b: &mut Bencher) {
        b.iter(|| {
            let args = App::new("starship")
                .arg(Arg::with_name("status_code"))
                .get_matches_from(vec!["starship", "0"]);

            let segment = modules::handle("nodejs", &args);
            print::stringify_segment(segment)
        });
    }
}
