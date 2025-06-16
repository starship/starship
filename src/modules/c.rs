use super::{Context, Module};
use crate::modules::cc::{Lang, module as cc_module};

/// Creates a module with the current C compiler and version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    cc_module(context, Lang::C)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_c_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_c_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.c"))?.sync_all()?;

        // What happens when `cc --version` says it's modern gcc?
        // The case when it claims to be clang is covered in folder_with_h_file,
        // and uses the mock in src/test/mod.rs.
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cc (Debian 10.2.1-6) 10.2.1 20210110
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v10.2.1-gcc ")
        ));
        assert_eq!(expected, actual);

        // What happens when `cc --version` says it's ancient gcc?
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cc (GCC) 3.3.5 (Debian 1:3.3.5-13)
Copyright (C) 2003 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v3.3.5-gcc ")
        ));
        assert_eq!(expected, actual);

        // What happens with an unknown C compiler? Needless to say, we're
        // not running on a Z80 so we're never going to see this one in reality!
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from("HISOFT-C Compiler  V1.2\nCopyright © 1984 HISOFT"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C ")));
        assert_eq!(expected, actual);

        // What happens when 'cc --version' doesn't work, but 'gcc --version' does?
        // This stubs out `cc` but we'll fall back to `gcc --version` as defined in
        // src/test/mod.rs.
        // Also we don't bother to redefine the config for this one, as we've already
        // proved we can parse its version.
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v10.2.1-gcc ")
        ));
        assert_eq!(expected, actual);

        // Now with both 'cc' and 'gcc' not working, this should fall back to 'clang --version'
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .cmd("gcc --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v11.1.0-clang ")
        ));
        assert_eq!(expected, actual);

        // What happens when we can't find any of cc, gcc or clang?
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .cmd("gcc --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C ")));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_h_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.h"))?.sync_all()?;

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v11.0.1-clang ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
