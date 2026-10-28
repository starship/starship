//! A conservative reader of project declarations, never a build evaluator.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use quick_xml::{Reader, events::Event};

use crate::utils::read_file_with_limit;

const FILE_LIMIT: u64 = 64 * 1024;
const TOTAL_LIMIT: u64 = 512 * 1024;
const MAX_DEPTH: usize = 8;

#[derive(Default)]
pub(crate) struct Versions {
    pub kotlin: Option<String>,
    pub java: Option<String>,
}

struct Inputs {
    start: Instant,
    remaining: u64,
    files: BTreeMap<PathBuf, Option<String>>,
    failed: bool,
}

impl Inputs {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            remaining: TOTAL_LIMIT,
            files: BTreeMap::new(),
            failed: false,
        }
    }

    fn expired(&self) -> bool {
        self.start.elapsed() >= Duration::from_millis(10)
    }

    fn read(&mut self, path: PathBuf) -> Option<String> {
        if self.expired() {
            return None;
        }
        if let Some(contents) = self.files.get(&path) {
            return contents.clone();
        }
        let contents = match read_file_with_limit(&path, FILE_LIMIT.min(self.remaining)) {
            Ok(contents) => Some(contents),
            Err(error) => {
                self.failed |= error.kind() != std::io::ErrorKind::NotFound;
                None
            }
        };
        if let Some(contents) = &contents {
            self.remaining -= contents.len() as u64;
        }
        self.files.insert(path, contents.clone());
        contents
    }
}

pub(super) fn resolve(directory: &Path) -> Versions {
    let mut inputs = Inputs::new();
    let versions = discover(directory, &mut inputs).unwrap_or_default();
    // Never publish a partially resolved result after exhausting the cooperative budget.
    if inputs.expired() || inputs.failed {
        Versions::default()
    } else {
        versions
    }
}

fn discover(directory: &Path, inputs: &mut Inputs) -> Option<Versions> {
    let mut directories = Vec::new();
    for directory in directory.ancestors().take(MAX_DEPTH) {
        if inputs.expired() {
            return None;
        }
        directories.push(directory);
        if directory.join(".git").exists()
            || directory.join("settings.gradle").exists()
            || directory.join("settings.gradle.kts").exists()
        {
            break;
        }
    }
    for (index, directory) in directories.iter().enumerate() {
        let pom = inputs.read(directory.join("pom.xml"));
        let groovy = inputs.read(directory.join("build.gradle"));
        let kotlin = inputs.read(directory.join("build.gradle.kts"));
        if pom.is_some() && (groovy.is_some() || kotlin.is_some())
            || groovy.is_some() && kotlin.is_some()
        {
            return None;
        }
        if let Some(pom) = pom {
            return maven(&pom, &directories[index..], inputs);
        }
        if let Some(build) = groovy.or(kotlin) {
            return gradle(&build, &directories[index..], inputs);
        }
        // An existing but unreadable/oversized build must not fall back to a parent project.
        if ["pom.xml", "build.gradle", "build.gradle.kts"]
            .iter()
            .any(|name| directory.join(name).exists())
        {
            return None;
        }
    }
    None
}

fn exact_version(value: &str) -> Option<String> {
    let value = value.trim();
    if value.starts_with(|c: char| c.is_ascii_digit())
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
    {
        Some(value.to_owned())
    } else {
        None
    }
}

fn unique(values: Vec<Option<String>>) -> Option<String> {
    let mut values = values.into_iter();
    let first = values.next()??;
    values
        .all(|value| value.as_ref() == Some(&first))
        .then_some(first)
}

fn property(value: &str, properties: &BTreeMap<String, String>) -> Option<String> {
    let mut value = value.trim().to_owned();
    for _ in 0..MAX_DEPTH {
        if let Some(key) = value.strip_prefix("${").and_then(|s| s.strip_suffix('}')) {
            value = properties.get(key)?.clone();
        } else {
            return exact_version(&value);
        }
    }
    None
}

// Tokens preserve strings and statement boundaries. Comments cannot masquerade as declarations.
fn tokens(input: &str) -> Option<Vec<String>> {
    let bytes = input.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        match bytes[i] {
            b'/' if bytes.get(i + 1) == Some(&b'/') => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if bytes.get(i + 1) == Some(&b'*') => {
                i += 2;
                let mut depth = 1;
                while i + 1 < bytes.len() && depth > 0 {
                    if &bytes[i..i + 2] == b"/*" {
                        depth += 1;
                        i += 2;
                    } else if &bytes[i..i + 2] == b"*/" {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if depth != 0 {
                    return None;
                }
            }
            b'\'' | b'"' => {
                let quote = bytes[i];
                i += 1;
                while i < bytes.len() && bytes[i] != quote {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                if i >= bytes.len() {
                    return None;
                }
                i += 1;
                out.push(input[start..i].to_owned());
            }
            b'\n' | b';' => {
                out.push(";".into());
                i += 1;
            }
            c if c.is_ascii_whitespace() => {
                i += 1;
            }
            c if c.is_ascii_alphanumeric() || b"_.$-".contains(&c) => {
                i += 1;
                while i < bytes.len()
                    && (bytes[i].is_ascii_alphanumeric() || b"_.$-".contains(&bytes[i]))
                {
                    i += 1;
                }
                out.push(input[start..i].to_owned());
            }
            c if c.is_ascii() => {
                out.push((c as char).to_string());
                i += 1;
            }
            _ => return None,
        }
    }
    Some(out)
}

fn unquote(value: &str) -> Option<&str> {
    value
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| value.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
}

fn gradle_value(value: &str, properties: &BTreeMap<String, String>) -> Option<String> {
    if let Some(value) = unquote(value) {
        property(value, properties)
    } else {
        property(properties.get(value)?, properties)
    }
}

fn catalog_plugin<'a>(alias: &str, catalog: Option<&'a toml::Table>) -> Option<&'a toml::Value> {
    let catalog = catalog?;
    let plugins = catalog.get("plugins")?.as_table()?;
    let mut matches = plugins
        .iter()
        .filter(|(key, _)| key.replace(['-', '_'], ".") == alias);
    let (_, plugin) = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    Some(plugin)
}

fn catalog_version(plugin: &toml::Value, catalog: Option<&toml::Table>) -> Option<String> {
    let catalog = catalog?;
    let version = plugin.get("version")?;
    if let Some(version) = version.as_str() {
        return exact_version(version);
    }
    let key = version.get("ref")?.as_str()?;
    exact_version(catalog.get("versions")?.get(key)?.as_str()?)
}

#[derive(Default)]
struct GradleDeclarations {
    kotlin: Vec<Option<String>>,
    kotlin_unknown: bool,
    defaults: Vec<Option<String>>,
    java: Vec<Option<String>>,
}

fn gradle_declarations(
    input: &str,
    properties: &BTreeMap<String, String>,
    catalog: Option<&toml::Table>,
) -> Option<GradleDeclarations> {
    let tokens = tokens(input)?;
    let mut out = GradleDeclarations::default();
    let mut scopes = Vec::new();
    let mut statement = Vec::new();
    for token in tokens {
        if token == "{" {
            let scope = statement.join(" ");
            scopes.push(scope);
            if scopes.len() > 64 {
                return None;
            }
            statement.clear();
        } else if token == ";" || token == "}" {
            gradle_statement(&statement, &scopes, properties, catalog, &mut out);
            statement.clear();
            if token == "}" {
                scopes.pop()?;
            }
        } else {
            statement.push(token);
        }
    }
    if !scopes.is_empty() {
        return None;
    }
    Some(out)
}

fn gradle_statement(
    statement: &[String],
    scopes: &[String],
    properties: &BTreeMap<String, String>,
    catalog: Option<&toml::Table>,
    out: &mut GradleDeclarations,
) {
    let s: Vec<&str> = statement.iter().map(String::as_str).collect();
    let plugins = scopes == ["plugins"] || scopes == ["pluginManagement", "plugins"];
    if plugins {
        let mut rest = &s[..];
        let mut alias = None;
        let kotlin = match rest {
            ["kotlin", "(", kind, ")", tail @ ..] if unquote(kind).is_some() => {
                rest = tail;
                true
            }
            ["id", "(", id, ")", tail @ ..] | ["id", id, tail @ ..]
                if unquote(id).is_some_and(|id| id.starts_with("org.jetbrains.kotlin.")) =>
            {
                rest = tail;
                true
            }
            ["alias", "(", name, ")", tail @ ..] if name.starts_with("libs.plugins.") => {
                let plugin = catalog_plugin(&name[13..], catalog);
                if plugin
                    .and_then(|p| p.get("id"))
                    .and_then(toml::Value::as_str)
                    .is_some_and(|id| !id.starts_with("org.jetbrains.kotlin."))
                {
                    return;
                }
                alias = Some(plugin.and_then(|p| catalog_version(p, catalog)));
                rest = tail;
                true
            }
            _ => false,
        };
        if kotlin {
            let needs_default = alias.is_none() && rest.is_empty();
            let value = match rest {
                [] | ["apply", "false"] => alias.flatten(),
                ["version", value] | ["version", value, "apply", "false"] => {
                    gradle_value(value, properties)
                }
                _ => None,
            };
            out.kotlin_unknown |= value.is_none() && !needs_default;
            if rest.ends_with(&["apply", "false"]) || scopes.len() == 2 {
                out.defaults.push(value);
            } else {
                out.kotlin.push(value);
            }
        }
    }
    if scopes == ["kotlin"] && s.first() == Some(&"jvmToolchain") {
        out.java.push(match s.as_slice() {
            ["jvmToolchain", "(", version, ")"] => exact_version(version),
            _ => None,
        });
    }
    if (scopes == ["java", "toolchain"] || scopes == ["kotlin", "jvmToolchain"])
        && s.first()
            .is_some_and(|word| word.starts_with("languageVersion"))
    {
        out.java.push(match s.as_slice() {
            [
                "languageVersion",
                "=",
                "JavaLanguageVersion.of",
                "(",
                version,
                ")",
            ]
            | [
                "languageVersion.set",
                "(",
                "JavaLanguageVersion.of",
                "(",
                version,
                ")",
                ")",
            ] => exact_version(version),
            _ => None,
        });
    }
}

fn gradle(build: &str, directories: &[&Path], inputs: &mut Inputs) -> Option<Versions> {
    let root_index = directories
        .iter()
        .position(|directory| {
            directory.join("settings.gradle").exists()
                || directory.join("settings.gradle.kts").exists()
                || directory.join(".git").exists()
        })
        .unwrap_or(0);
    let directories = &directories[..=root_index];
    let root = *directories.last()?;
    let mut properties = BTreeMap::new();
    // Module properties override root properties; these are declarations, not Gradle evaluation.
    for directory in directories.iter().rev() {
        if let Some(contents) = inputs.read(directory.join("gradle.properties")) {
            for line in contents
                .lines()
                .filter(|line| !line.trim_start().starts_with(['#', '!']))
            {
                if let Some((key, value)) = line.split_once('=') {
                    properties.insert(key.trim().into(), value.trim().into());
                }
            }
        }
    }
    let catalog = inputs
        .read(root.join("gradle/libs.versions.toml"))
        .and_then(|text| toml::from_str::<toml::Table>(&text).ok());
    let declarations = gradle_declarations(build, &properties, catalog.as_ref())?;
    let kotlin = if declarations.kotlin_unknown || declarations.kotlin.is_empty() {
        None
    } else if declarations.kotlin.iter().all(Option::is_some) {
        unique(declarations.kotlin)
    } else {
        let mut defaults = declarations.defaults;
        for name in [
            "settings.gradle",
            "settings.gradle.kts",
            "build.gradle",
            "build.gradle.kts",
        ] {
            if let Some(text) = inputs.read(root.join(name)) {
                let parsed = gradle_declarations(&text, &properties, catalog.as_ref())?;
                defaults.extend(parsed.defaults);
                if name.starts_with("build.") {
                    if parsed.kotlin_unknown {
                        defaults.push(None);
                    } else {
                        defaults.extend(parsed.kotlin.into_iter().filter(Option::is_some));
                    }
                }
            }
        }
        let default = unique(defaults);
        unique(
            declarations
                .kotlin
                .into_iter()
                .map(|value| value.or_else(|| default.clone()))
                .collect(),
        )
    };
    Some(Versions {
        kotlin,
        java: unique(declarations.java),
    })
}

#[derive(Default)]
struct Element {
    name: String,
    text: String,
    children: Vec<Self>,
}

impl Element {
    fn child(&self, name: &str) -> Option<&Self> {
        let mut matching = self.children.iter().filter(|child| child.name == name);
        let child = matching.next()?;
        matching.next().is_none().then_some(child)
    }
    fn at(&self, path: &[&str]) -> Option<&Self> {
        path.iter()
            .try_fold(self, |element, name| element.child(name))
    }
    fn text(&self, path: &[&str]) -> Option<&str> {
        Some(self.at(path)?.text.trim())
    }
}

fn xml(input: &str) -> Option<Element> {
    let mut reader = Reader::from_str(input);
    let mut stack = vec![Element::default()];
    loop {
        match reader.read_event().ok()? {
            Event::Start(event) => {
                if stack.len() >= 64 {
                    return None;
                }
                stack.push(Element {
                    name: event.name().as_ref().into(),
                    ..Element::default()
                });
            }
            Event::Empty(event) => {
                stack.last_mut()?.children.push(Element {
                    name: event.name().as_ref().into(),
                    ..Element::default()
                });
            }
            Event::End(_) => {
                if stack.len() <= 1 {
                    return None;
                }
                let element = stack.pop()?;
                stack.last_mut()?.children.push(element);
            }
            Event::Text(event) => stack.last_mut()?.text.push_str(event.as_ref()),
            Event::Eof => break,
            Event::Decl(_) | Event::Comment(_) => (),
            _ => return None,
        }
    }
    if stack.len() != 1 {
        return None;
    }
    let mut root = stack.pop()?;
    if root.children.len() != 1 || root.children[0].name != "project" {
        return None;
    }
    Some(root.children.remove(0))
}

fn maven(pom: &str, directories: &[&Path], inputs: &mut Inputs) -> Option<Versions> {
    let mut projects = vec![xml(pom)?];
    // Only explicit/default ../pom.xml parents within the discovery boundary are followed.
    for pair in directories.windows(2) {
        if projects.last()?.child("parent").is_none() {
            break;
        }
        let parent = projects.last()?.child("parent")?;
        if parent
            .text(&["relativePath"])
            .is_some_and(|path| path != "../pom.xml")
        {
            return None;
        }
        let parent_project = xml(&inputs.read(pair[1].join("pom.xml"))?)?;
        for coordinate in ["groupId", "artifactId", "version"] {
            let expected = parent.text(&[coordinate])?;
            let actual = parent_project
                .text(&[coordinate])
                .or_else(|| parent_project.text(&["parent", coordinate]))?;
            if expected != actual || expected.contains('$') {
                return None;
            }
        }
        projects.push(parent_project);
    }
    if projects.last()?.child("parent").is_some() {
        return None;
    }
    if projects
        .iter()
        .any(|project| project.child("profiles").is_some())
    {
        return None;
    }
    let mut properties = BTreeMap::new();
    for project in projects.iter().rev() {
        if let Some(props) = project.child("properties") {
            for prop in &props.children {
                properties.insert(prop.name.clone(), prop.text.trim().to_owned());
            }
        }
    }
    let mut versions = Versions::default();
    // Use the nearest declaration for each plugin, retaining conflicts in that declaration.
    let mut kotlin_seen = false;
    let mut java_seen = false;
    for project in &projects {
        let Some(plugins) = project.at(&["build", "plugins"]) else {
            continue;
        };
        let mut kotlin = Vec::new();
        let mut java = Vec::new();
        for plugin in plugins
            .children
            .iter()
            .filter(|plugin| plugin.name == "plugin")
        {
            match (plugin.text(&["groupId"]), plugin.text(&["artifactId"])) {
                (Some("org.jetbrains.kotlin"), Some("kotlin-maven-plugin")) => {
                    kotlin.push(
                        plugin
                            .text(&["version"])
                            .and_then(|value| property(value, &properties)),
                    );
                }
                (None | Some("org.apache.maven.plugins"), Some("maven-compiler-plugin")) => {
                    java.push(
                        plugin
                            .text(&["configuration", "jdkToolchain", "version"])
                            .and_then(|value| property(value, &properties)),
                    );
                }
                _ => (),
            }
        }
        if !kotlin_seen && !kotlin.is_empty() {
            kotlin_seen = true;
            versions.kotlin = unique(kotlin);
        }
        if !java_seen && !java.is_empty() {
            java_seen = true;
            versions.java = unique(java);
        }
    }
    Some(versions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    fn fixture(files: &[(&str, &str)], cwd: &str) -> io::Result<Versions> {
        let dir = tempfile::tempdir()?;
        for (name, contents) in files {
            let path = dir.path().join(name);
            std::fs::create_dir_all(path.parent().unwrap())?;
            let mut file = std::fs::File::create(path)?;
            file.write_all(contents.as_bytes())?;
            file.sync_all()?;
        }
        let cwd = dir.path().join(cwd);
        std::fs::create_dir_all(&cwd)?;
        let versions = resolve(&cwd);
        dir.close()?;
        Ok(versions)
    }

    #[test]
    fn gradle_catalog_from_source_directory() -> io::Result<()> {
        let versions = fixture(
            &[
                ("settings.gradle.kts", "rootProject.name = \"sample\""),
                (
                    "build.gradle.kts",
                    "plugins { alias(libs.plugins.kotlin.jvm) }\nkotlin { jvmToolchain(21) }",
                ),
                (
                    "gradle/libs.versions.toml",
                    "[versions]\nkotlin = '2.1.20'\n[plugins]\nkotlin-jvm = { id = 'org.jetbrains.kotlin.jvm', version.ref = 'kotlin' }",
                ),
            ],
            "src/main/kotlin",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        assert_eq!(versions.java.as_deref(), Some("21"));
        Ok(())
    }

    #[test]
    fn gradle_root_defaults_and_module_properties() -> io::Result<()> {
        let versions = fixture(
            &[
                (
                    "settings.gradle",
                    "pluginManagement { plugins { id 'org.jetbrains.kotlin.jvm' version '2.1.20' } }",
                ),
                (
                    "app/build.gradle",
                    "plugins { id 'org.jetbrains.kotlin.jvm' }",
                ),
            ],
            "app/src",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        let versions = fixture(
            &[
                ("settings.gradle.kts", ""),
                ("gradle.properties", "kotlinVersion=2.1.21"),
                (
                    "build.gradle.kts",
                    "plugins { kotlin(\"jvm\") version \"${kotlinVersion}\" }",
                ),
            ],
            "",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.21"));
        Ok(())
    }

    #[test]
    fn gradle_ignores_comments_targets_and_unapplied_plugins() {
        let declarations = gradle_declarations(
            r#"
            // plugins { kotlin("jvm") version "9.9.9" }
            /* java { toolchain { languageVersion = JavaLanguageVersion.of(99) } } */
            plugins { kotlin("jvm") version "2.1.20" apply false }
            java { sourceCompatibility = JavaVersion.VERSION_17 }
            tasks.named("compileKotlin") { jvmTarget = "17" }
        "#,
            &BTreeMap::new(),
            None,
        )
        .unwrap();
        assert!(declarations.kotlin.is_empty());
        assert!(declarations.java.is_empty());
    }

    #[test]
    fn gradle_conflicting_and_computed_versions_are_unresolved() -> io::Result<()> {
        for build in [
            "plugins { kotlin(\"jvm\") version computeVersion() }",
            "plugins { kotlin(\"jvm\") version \"2.1.20\" + suffix }",
            "plugins { kotlin(\"jvm\") version \"2.1.20\"; kotlin(\"plugin.serialization\") version \"2.1.21\" }",
        ] {
            let versions = fixture(
                &[("settings.gradle.kts", ""), ("build.gradle.kts", build)],
                "",
            )?;
            assert!(versions.kotlin.is_none(), "{build}");
        }
        Ok(())
    }

    #[test]
    fn computed_module_version_must_not_use_root_default() -> io::Result<()> {
        let versions = fixture(
            &[
                (
                    "settings.gradle.kts",
                    "pluginManagement { plugins { kotlin(\"jvm\") version \"2.1.20\" } }",
                ),
                (
                    "app/build.gradle.kts",
                    "plugins { kotlin(\"jvm\") version computeVersion() }",
                ),
            ],
            "app",
        )?;
        assert!(versions.kotlin.is_none());
        Ok(())
    }

    #[test]
    fn root_build_can_use_settings_plugin_defaults() -> io::Result<()> {
        let versions = fixture(
            &[
                (
                    "settings.gradle.kts",
                    "pluginManagement { plugins { kotlin(\"jvm\") version \"2.1.20\" } }",
                ),
                ("build.gradle.kts", "plugins { kotlin(\"jvm\") }"),
            ],
            "",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        Ok(())
    }

    #[test]
    fn standalone_build_uses_its_own_catalog() -> io::Result<()> {
        let versions = fixture(
            &[
                (
                    "build.gradle.kts",
                    "plugins { alias(libs.plugins.kotlin.jvm) }",
                ),
                (
                    "gradle/libs.versions.toml",
                    "[plugins]\nkotlin-jvm = { id = 'org.jetbrains.kotlin.jvm', version = '2.1.20' }",
                ),
            ],
            "src",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        Ok(())
    }

    #[test]
    fn non_kotlin_catalog_plugins_do_not_hide_kotlin() -> io::Result<()> {
        let versions = fixture(
            &[
                ("settings.gradle.kts", ""),
                (
                    "build.gradle.kts",
                    "plugins { alias(libs.plugins.android); alias(libs.plugins.kotlin.jvm) }",
                ),
                (
                    "gradle/libs.versions.toml",
                    "[plugins]\nandroid = { id = 'com.android.application', version = '8.9.0' }\nkotlin-jvm = { id = 'org.jetbrains.kotlin.jvm', version = '2.1.20' }",
                ),
            ],
            "",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        Ok(())
    }

    #[test]
    fn mismatched_local_maven_parent_is_not_used() -> io::Result<()> {
        let versions = fixture(
            &[
                (".git", ""),
                (
                    "pom.xml",
                    "<project><groupId>wrong</groupId><artifactId>parent</artifactId><version>1</version><properties><kotlin.version>9.9.9</kotlin.version></properties></project>",
                ),
                (
                    "app/pom.xml",
                    "<project><parent><groupId>expected</groupId><artifactId>parent</artifactId><version>1</version></parent><build><plugins><plugin><groupId>org.jetbrains.kotlin</groupId><artifactId>kotlin-maven-plugin</artifactId><version>${kotlin.version}</version></plugin></plugins></build></project>",
                ),
            ],
            "app",
        )?;
        assert!(versions.kotlin.is_none());
        Ok(())
    }

    #[test]
    fn maven_plugins_and_properties_from_local_parent() -> io::Result<()> {
        let versions = fixture(
            &[
                (
                    "pom.xml",
                    r#"<project><groupId>sample</groupId><artifactId>parent</artifactId><version>1</version><properties><kotlin.version>2.1.20</kotlin.version><jdk.version>21</jdk.version></properties></project>"#,
                ),
                (
                    "app/pom.xml",
                    r#"<project><parent><groupId>sample</groupId><artifactId>parent</artifactId><version>1</version></parent><build><plugins>
                <plugin><groupId>org.jetbrains.kotlin</groupId><artifactId>kotlin-maven-plugin</artifactId><version>${kotlin.version}</version></plugin>
                <plugin><artifactId>maven-compiler-plugin</artifactId><configuration><jdkToolchain><version>${jdk.version}</version></jdkToolchain></configuration></plugin>
                </plugins></build></project>"#,
                ),
                (".git", "gitdir: unused"),
            ],
            "app/src/main",
        )?;
        assert_eq!(versions.kotlin.as_deref(), Some("2.1.20"));
        assert_eq!(versions.java.as_deref(), Some("21"));
        Ok(())
    }

    #[test]
    fn maven_does_not_confuse_targets_dependencies_or_plugin_management() -> io::Result<()> {
        let versions = fixture(
            &[(
                "pom.xml",
                r#"<project><version>9.9.9</version>
            <properties><maven.compiler.release>17</maven.compiler.release></properties>
            <dependencies><dependency><artifactId>kotlin-stdlib</artifactId><version>9.9.9</version></dependency></dependencies>
            <build><pluginManagement><plugins><plugin><groupId>org.jetbrains.kotlin</groupId><artifactId>kotlin-maven-plugin</artifactId><version>9.9.9</version></plugin></plugins></pluginManagement></build>
            </project>"#,
            )],
            "",
        )?;
        assert!(versions.kotlin.is_none());
        assert!(versions.java.is_none());
        Ok(())
    }

    #[test]
    fn malformed_or_profile_dependent_maven_is_unresolved() -> io::Result<()> {
        for pom in [
            "<project><build></project>",
            "<project><profiles/></project>",
            "<!DOCTYPE project><project/>",
        ] {
            let versions = fixture(&[("pom.xml", pom)], "")?;
            assert!(versions.kotlin.is_none());
            assert!(versions.java.is_none());
        }
        Ok(())
    }

    #[test]
    fn unreadable_or_oversized_nearer_build_does_not_use_parent() -> io::Result<()> {
        let large = " ".repeat(FILE_LIMIT as usize + 1);
        let versions = fixture(
            &[
                ("settings.gradle.kts", ""),
                (
                    "build.gradle.kts",
                    "plugins { kotlin(\"jvm\") version \"2.1.20\" }",
                ),
                ("app/build.gradle.kts", &large),
            ],
            "app",
        )?;
        assert!(versions.kotlin.is_none());
        Ok(())
    }

    #[test]
    fn git_boundary_and_depth_prevent_unrelated_parent_detection() -> io::Result<()> {
        for (marker, cwd) in [("app/.git", "app/src"), ("other", "a/b/c/d/e/f/g/h/i")] {
            let versions = fixture(
                &[
                    (
                        "build.gradle.kts",
                        "plugins { kotlin(\"jvm\") version \"2.1.20\" }",
                    ),
                    (marker, ""),
                ],
                cwd,
            )?;
            assert!(versions.kotlin.is_none());
        }
        Ok(())
    }

    #[test]
    fn file_edits_are_visible_on_the_next_render() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let build = dir.path().join("build.gradle.kts");
        for version in ["2.1.20", "2.1.21"] {
            let mut file = std::fs::File::create(&build)?;
            write!(file, "plugins {{ kotlin(\"jvm\") version \"{version}\" }}")?;
            file.sync_all()?;
            assert_eq!(resolve(dir.path()).kotlin.as_deref(), Some(version));
        }
        std::fs::remove_file(build)?;
        assert!(resolve(dir.path()).kotlin.is_none());
        dir.close()
    }

    #[cfg(unix)]
    #[test]
    fn named_pipe_input_does_not_block_resolution() -> io::Result<()> {
        use nix::{sys::stat::Mode, unistd::mkfifo};
        let dir = tempfile::tempdir()?;
        mkfifo(
            &dir.path().join("build.gradle.kts"),
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;
        assert!(resolve(dir.path()).kotlin.is_none());
        dir.close()
    }

    #[test]
    fn budget_exhaustion_and_property_cycles_are_unresolved() {
        let mut inputs = Inputs::new();
        inputs.start = Instant::now() - Duration::from_secs(1);
        assert!(discover(Path::new("."), &mut inputs).is_none());
        let properties = BTreeMap::from([("a".into(), "${b}".into()), ("b".into(), "${a}".into())]);
        assert!(property("${a}", &properties).is_none());
        assert!(exact_version("[17,22)").is_none());
    }
}
