use toml::{Table, Value, toml};

use crate::context::JJRepo;
use crate::test::ModuleRenderer;

/// Helper to test JJ modules
#[must_use]
pub struct JJTester {
    /// JJ module being tested
    module: &'static str,
    /// 'Repo' (see `JJ_REPO` constants) to test in
    repo: &'static str,
    /// Options to add to the module's config, if any
    options: Option<Table>,
    /// Expected output, None by default
    expected: Option<String>,
}

impl JJTester {
    pub fn new(module: &'static str) -> Self {
        Self {
            module,
            repo: "",
            options: None,
            expected: None,
        }
    }

    pub fn repo(mut self, repo: &'static str) -> Self {
        self.repo = repo;
        self
    }

    pub fn options(mut self, options: Table) -> Self {
        self.options = Some(options);
        self
    }

    pub fn expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Test rendering against `self.expected`
    #[track_caller]
    pub fn render(self) {
        assert_ne!(
            self.repo, "",
            "You forgot to set a repository for this `Tester` instance"
        );

        let rendered_output = ModuleRenderer::new(self.module)
            .jj_repo(self.repo)
            .config({
                let mut config = Table::with_capacity(1);
                if let Some(options) = self.options {
                    config.insert(self.module.into(), Value::Table(options));
                }
                config
            })
            .collect();

        assert_eq!(rendered_output, self.expected);
    }

    /// A collection of basic tests, usually run in a `fn test_render_basics()` that will ensure
    /// some sane defaults are respected, like actually disabling on `disabled = true` in the
    /// config or not rendering anything when an unknown var is used or no output is produced
    /// by the `jj` command.
    #[track_caller]
    pub fn basic_tests(module: &'static str) {
        // No JJ repo -> no command output to parse
        Self::new(module).repo(JJRepo::NONE).render();
        // JJ repo but empty output -> nothing to parse
        Self::new(module).repo(JJRepo::EMPTY_OUTPUT).render();
        // JJ repo and invalid output -> parsing fails
        Self::new(module).repo(JJRepo::INVALID_OUTPUT).render();
        // Invalid format
        Self::new(module)
            .repo(JJRepo::BASE)
            .options(toml! { format = "[" })
            .render();
        // Non existent variable
        Self::new(module)
            .repo(JJRepo::BASE)
            .options(toml! { format = "$not_a_valid_jj_variable_in_any_module" })
            .render();
        // Non existent style
        Self::new(module)
            .repo(JJRepo::BASE)
            .options(toml! { format = "[*]($not_a_valid_jj_style_variable_in_any_module)" })
            .expected("*")
            .render();
        // Disabled module
        Self::new(module)
            .repo(JJRepo::BASE)
            .options(toml! { disabled = true })
            .render();
    }
}
