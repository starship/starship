use std::io;
use std::path::PathBuf;

use once_cell::sync::Lazy;

use crate::common::{self, TestCommand, MANIFEST_DIR};
use ansi_term::{Color, Style};

static KUBECTL_CONFIG: Lazy<PathBuf> =
    Lazy::new(|| MANIFEST_DIR.join("tests/fixtures/kubeconfig.yaml"));

#[test]
fn config_disabled() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert!(actual.is_empty());
    Ok(())
}

#[test]
fn config_enabled() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_empty() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = []
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_empty_obj() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [{}]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_invalid() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = "foobar!"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_no_name() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [
                { style = "red", symbol = "[k8s] " }
            ]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_none_matched() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [
                { name = "this-is-not-set", style = "red", symbol = "[k8s] " }
            ]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_default(), actual);
    Ok(())
}

#[test]
fn custom_envs_all_values() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [
                { name = "test", style = "red", symbol = "[k8s] " }
            ]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "on {} ",
        Color::Red
            .normal()
            .paint("[k8s] test_context (test_namespace)")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn custom_envs_no_symbol() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [
                { name = "test", style = "red" }
            ]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "on {} ",
        Color::Red.normal().paint("☸ test_context (test_namespace)")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn custom_envs_no_style() -> io::Result<()> {
    let output = common::render_module("kubernetes")
        .env("KUBECONFIG", KUBECTL_CONFIG.as_os_str())
        .use_config(toml::toml! {
            [kubernetes]
            disabled = false
            environments = [
                { name = "test", symbol = "[k8s] " }
            ]
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "on {} ",
        default_style().paint("[k8s] test_context (test_namespace)")
    );
    assert_eq!(expected, actual);
    Ok(())
}

fn default_style() -> Style {
    Color::Cyan.bold()
}

fn expected_default() -> String {
    format!(
        "on {} ",
        default_style().paint("☸ test_context (test_namespace)")
    )
}
