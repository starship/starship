use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run_git(args: &[&str], current_dir: Option<&Path>) -> Output {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(current_dir) = current_dir {
        command.current_dir(current_dir);
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .expect("git must be available for this integration test")
}

fn assert_git_success(output: Output, args: &[&str]) {
    assert!(
        output.status.success(),
        "git {args:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn git_status_uses_worktree_from_environment_for_bare_repository() {
    let root = tempfile::tempdir().expect("temporary directory must be created");
    let bare_repo = root.path().join("dotfiles.git");
    let seed_repo = root.path().join("seed");
    let worktree = root.path().join("worktree");
    let config = root.path().join("starship.toml");

    assert_git_success(
        run_git(
            &["init", "--bare", "--quiet", bare_repo.to_str().unwrap()],
            None,
        ),
        &["init", "--bare"],
    );
    assert_git_success(
        run_git(&["init", "--quiet", seed_repo.to_str().unwrap()], None),
        &["init"],
    );
    assert_git_success(
        run_git(&["config", "user.name", "Starship Test"], Some(&seed_repo)),
        &["config", "user.name"],
    );
    assert_git_success(
        run_git(
            &["config", "user.email", "starship@example.test"],
            Some(&seed_repo),
        ),
        &["config", "user.email"],
    );

    fs::write(seed_repo.join("tracked.txt"), "tracked\n").expect("seed file must be written");
    assert_git_success(
        run_git(&["add", "tracked.txt"], Some(&seed_repo)),
        &["add", "tracked.txt"],
    );
    assert_git_success(
        run_git(
            &["commit", "--quiet", "-m", "initial", "--no-gpg-sign"],
            Some(&seed_repo),
        ),
        &["commit"],
    );
    assert_git_success(
        run_git(&["branch", "-M", "main"], Some(&seed_repo)),
        &["branch", "-M", "main"],
    );
    assert_git_success(
        run_git(
            &["remote", "add", "origin", bare_repo.to_str().unwrap()],
            Some(&seed_repo),
        ),
        &["remote", "add"],
    );
    assert_git_success(
        run_git(&["push", "--quiet", "origin", "main"], Some(&seed_repo)),
        &["push"],
    );
    assert_git_success(
        run_git(
            &[
                "--git-dir",
                bare_repo.to_str().unwrap(),
                "symbolic-ref",
                "HEAD",
                "refs/heads/main",
            ],
            None,
        ),
        &["symbolic-ref", "HEAD"],
    );

    fs::create_dir(&worktree).expect("worktree directory must be created");
    assert_git_success(
        run_git(
            &[
                "--git-dir",
                bare_repo.to_str().unwrap(),
                "--work-tree",
                worktree.to_str().unwrap(),
                "checkout",
                "--quiet",
                "--force",
                "main",
            ],
            None,
        ),
        &["checkout", "main"],
    );
    fs::write(worktree.join("untracked.txt"), "untracked\n")
        .expect("untracked file must be written");
    fs::write(&config, "format = \"$git_status\"\n").expect("Starship config must be written");

    let starship = env::var_os("CARGO_BIN_EXE_starship")
        .expect("Cargo must provide the Starship binary to integration tests");
    let output = Command::new(starship)
        .args(["module", "git_status", "--path"])
        .arg(&worktree)
        .env("GIT_DIR", &bare_repo)
        .env("GIT_WORK_TREE", &worktree)
        .env_remove("GIT_INDEX_FILE")
        .env("STARSHIP_CONFIG", &config)
        .env("TERM", "xterm-256color")
        .output()
        .expect("Starship must run");

    assert!(
        output.status.success(),
        "Starship failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains('?'),
        "git_status should report the untracked worktree file, got: {stdout:?}"
    );
}
