#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use starship::{modules, print};
    use test::Bencher;
    use clap::{App, Arg};

    #[bench]
    fn full_prompt_bench(b: &mut Bencher) {
        b.iter(||{
            let args = App::new("starship")
                .arg(Arg::with_name("status_code"))
                .get_matches_from(vec!["starship", "0"]);

            starship::print::prompt(args)
        });
    }

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
}
