#![feature(test)]

extern crate starship;
extern crate test;

#[cfg(test)]
mod benches {
    use ansi_term::Color;
    use starship::config::*;
    use starship_module_config_derive::ModuleConfig;
    use test::Bencher;

    #[bench]
    fn bench_load_config(b: &mut Bencher) {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig<'a> {
            pub prefix: &'a str,
            pub disabled: bool,
            pub some_array: Vec<&'a str>,
        }

        impl<'a> RootModuleConfig<'a> for TestConfig<'a> {
            fn new() -> Self {
                TestConfig {
                    prefix: "on ",
                    disabled: false,
                    some_array: vec!["A", "B", "C"],
                }
            }
        }

        let config = toml::toml! {
            prefix = "T "
            disabled = true
            some_array = ["A"]
        };

        b.iter(|| TestConfig::load(&config));
    }

    #[bench]
    fn bench_load_segment_config(b: &mut Bencher) {
        #[derive(Clone, ModuleConfig)]
        struct TestConfig<'a> {
            pub untracked: SegmentConfig<'a>,
            pub modified: SegmentConfig<'a>,
        }

        impl<'a> RootModuleConfig<'a> for TestConfig<'a> {
            fn new() -> Self {
                TestConfig {
                    untracked: SegmentConfig {
                        value: "?",
                        style: Some(Color::Red.bold()),
                    },
                    modified: SegmentConfig {
                        value: "!",
                        style: Some(Color::Red.bold()),
                    },
                }
            }
        }

        let config = toml::toml! {
            untracked.value = "x"
            modified = { value = "•", style = "red" }
        };

        b.iter(|| TestConfig::load(&config))
    }
}
