use std::process::Command;
use tempfile::TempDir;

#[test]
#[ignore] // Requires network access
fn install_skill_from_github() {
    let temp = TempDir::new().unwrap();
    let dest = temp.path().join(".claude/skills/generate-sandbox-policy");

    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .args([
            "install",
            "https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy",
        ])
        .current_dir(temp.path())
        .output()
        .expect("Failed to execute skm");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dest.join("SKILL.md").exists(), "SKILL.md should exist");
}

#[test]
fn install_invalid_url_returns_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .args(["install", "https://gitlab.com/owner/repo"])
        .output()
        .expect("Failed to execute skm");

    assert!(!output.status.success());
}

#[test]
fn uninstall_nonexistent_skill_returns_error() {
    let temp = TempDir::new().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .args(["uninstall", "nonexistent-skill"])
        .current_dir(temp.path())
        .output()
        .expect("Failed to execute skm");

    assert!(!output.status.success());
}

#[test]
fn list_empty_returns_nothing() {
    let temp = TempDir::new().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .args(["list"])
        .current_dir(temp.path())
        .output()
        .expect("Failed to execute skm");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
