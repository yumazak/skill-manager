use std::process::Command;
use tempfile::TempDir;

#[test]
#[ignore] // Requires network access
fn download_skill_from_github() {
    let temp = TempDir::new().unwrap();
    let dest = temp.path().join(".claude/skills/generate-sandbox-policy");

    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .arg("https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy")
        .arg("--local")
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
fn invalid_url_returns_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_skm"))
        .arg("https://gitlab.com/owner/repo")
        .output()
        .expect("Failed to execute skm");

    assert!(!output.status.success());
}
