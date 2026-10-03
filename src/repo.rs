//! Git-backed repository operations.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::catalog::Package;
use crate::paths::managed_path;

/// The local state of a catalog entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalState {
    Missing,
    Installed,
    Conflict,
}

impl LocalState {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Missing => "not installed",
            Self::Installed => "installed",
            Self::Conflict => "path conflict",
        }
    }
}

/// An actionable failure from a repository operation.
#[derive(Debug)]
pub struct RepoError(String);

impl fmt::Display for RepoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for RepoError {}

/// Detects whether a package is absent, installed, or blocked by another path.
#[must_use]
pub fn local_state(root: &Path, package: Package) -> LocalState {
    let Ok(path) = managed_path(root, package.name) else {
        return LocalState::Conflict;
    };
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => LocalState::Missing,
        Ok(metadata) if !metadata.file_type().is_symlink() && path.join(".git").exists() => {
            LocalState::Installed
        }
        Err(_) | Ok(_) => LocalState::Conflict,
    }
}

/// Clones a missing package into the managed root.
///
/// # Errors
///
/// Returns an error when the destination exists, the root cannot be created,
/// Git cannot start, or the clone fails.
pub fn install(root: &Path, package: Package) -> Result<String, RepoError> {
    let destination = package_path(root, package)?;
    match fs::symlink_metadata(&destination) {
        Ok(_) => {
            return Err(RepoError(format!(
                "{} already exists; MamboTools will not overwrite it",
                destination.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(RepoError(format!(
                "could not inspect {}: {error}",
                destination.display()
            )));
        }
    }
    fs::create_dir_all(root).map_err(|error| {
        RepoError(format!(
            "could not create install directory {}: {error}",
            root.display()
        ))
    })?;

    let output = Command::new("git")
        .args(["clone", "--", &package.clone_url()])
        .arg(&destination)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .map_err(|error| RepoError(format!("could not start git: {error}")))?;
    command_result(&output, format!("installed {}", package.name))
}

/// Pulls the configured upstream only when Git can fast-forward.
///
/// # Errors
///
/// Returns an error when the package is not an installed Git repository, Git
/// cannot start, or the pull cannot fast-forward.
pub fn update(root: &Path, package: Package) -> Result<String, RepoError> {
    let destination = installed_package_path(root, package)?;
    verify_origin(&destination, package)?;
    let output = Command::new("git")
        .arg("-C")
        .arg(&destination)
        .args(["pull", "--ff-only"])
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .map_err(|error| RepoError(format!("could not start git: {error}")))?;
    command_result(&output, format!("updated {}", package.name))
}

/// Permanently removes an installed package after verifying its Git root.
///
/// The caller is responsible for obtaining explicit user confirmation.
///
/// # Errors
///
/// Returns an error when the package is not a safe managed Git root or when
/// its directory cannot be removed.
pub fn delete(root: &Path, package: Package) -> Result<String, RepoError> {
    let destination = installed_package_path(root, package)?;
    let metadata = fs::symlink_metadata(&destination).map_err(|error| {
        RepoError(format!(
            "could not inspect {}: {error}",
            destination.display()
        ))
    })?;
    if metadata.file_type().is_symlink() {
        return Err(RepoError("refusing to delete a symbolic link".to_owned()));
    }

    verify_git_root(&destination)?;
    verify_origin(&destination, package)?;
    fs::remove_dir_all(&destination).map_err(|error| {
        RepoError(format!(
            "could not delete {}: {error}",
            destination.display()
        ))
    })?;
    Ok(format!("deleted {}", package.name))
}

fn verify_origin(destination: &Path, package: Package) -> Result<(), RepoError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(destination)
        .args(["remote", "get-url", "origin"])
        .output()
        .map_err(|error| RepoError(format!("could not start git: {error}")))?;
    if !output.status.success() {
        return Err(command_error(&output));
    }

    let origin = String::from_utf8_lossy(&output.stdout);
    if !origin_matches(origin.trim(), package) {
        return Err(RepoError(format!(
            "refusing to manage {} because origin is not ProjectMambo/{}",
            destination.display(),
            package.name
        )));
    }
    Ok(())
}

fn origin_matches(origin: &str, package: Package) -> bool {
    let normalized = origin.trim_end_matches('/').trim_end_matches(".git");
    let repository = format!("ProjectMambo/{}", package.name);
    normalized == format!("https://github.com/{repository}")
        || normalized == format!("git@github.com:{repository}")
        || normalized == format!("ssh://git@github.com/{repository}")
}

fn package_path(root: &Path, package: Package) -> Result<PathBuf, RepoError> {
    managed_path(root, package.name).map_err(RepoError)
}

fn installed_package_path(root: &Path, package: Package) -> Result<PathBuf, RepoError> {
    let path = package_path(root, package)?;
    match local_state(root, package) {
        LocalState::Installed => Ok(path),
        LocalState::Missing => Err(RepoError(format!("{} is not installed", package.name))),
        LocalState::Conflict => Err(RepoError(format!(
            "{} exists but is not a Git repository; refusing to touch it",
            path.display()
        ))),
    }
}

fn verify_git_root(destination: &Path) -> Result<(), RepoError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(destination)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| RepoError(format!("could not start git: {error}")))?;
    if !output.status.success() {
        return Err(command_error(&output));
    }

    let reported = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    let expected = destination.canonicalize().map_err(|error| {
        RepoError(format!(
            "could not resolve {}: {error}",
            destination.display()
        ))
    })?;
    let reported = reported
        .canonicalize()
        .map_err(|error| RepoError(format!("could not resolve Git root: {error}")))?;
    if reported != expected {
        return Err(RepoError(format!(
            "refusing to delete {} because its Git root is {}",
            destination.display(),
            reported.display()
        )));
    }
    Ok(())
}

fn command_result(output: &Output, fallback: String) -> Result<String, RepoError> {
    if output.status.success() {
        let detail = concise_output(output);
        Ok(if detail.is_empty() { fallback } else { detail })
    } else {
        Err(command_error(output))
    }
}

fn command_error(output: &Output) -> RepoError {
    let detail = concise_output(output);
    let status = output.status.code().map_or_else(
        || "terminated by signal".to_owned(),
        |code| format!("exit {code}"),
    );
    if detail.is_empty() {
        RepoError(format!("git failed ({status})"))
    } else {
        RepoError(format!("git failed ({status}): {detail}"))
    }
}

fn concise_output(output: &Output) -> String {
    let bytes = if output.stderr.is_empty() {
        &output.stdout
    } else {
        &output.stderr
    };
    String::from_utf8_lossy(bytes)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(500)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitStatus;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

    #[test]
    #[cfg(unix)]
    fn concise_output_prefers_stderr_and_limits_length() {
        let output = Output {
            status: ExitStatus::from_raw(0),
            stdout: b"stdout".to_vec(),
            stderr: format!("  {}  ", "x".repeat(700)).into_bytes(),
        };

        let summary = concise_output(&output);
        assert_eq!(summary.len(), 500);
        assert!(!summary.contains("stdout"));
    }

    #[test]
    fn state_labels_are_clear() {
        assert_eq!(LocalState::Missing.label(), "not installed");
        assert_eq!(LocalState::Installed.label(), "installed");
        assert_eq!(LocalState::Conflict.label(), "path conflict");
    }

    #[test]
    fn origin_must_identify_the_selected_project_mambo_repository() {
        let package = Package {
            name: "MamboTools",
            summary: "",
        };

        assert!(origin_matches(
            "https://github.com/ProjectMambo/MamboTools.git",
            package
        ));
        assert!(origin_matches(
            "git@github.com:ProjectMambo/MamboTools.git",
            package
        ));
        assert!(!origin_matches(
            "https://github.com/someone-else/MamboTools.git",
            package
        ));
        assert!(!origin_matches(
            "https://github.com/ProjectMambo/MamboUI.git",
            package
        ));
    }

    #[test]
    fn delete_removes_only_a_matching_managed_clone() {
        let temporary = TemporaryDirectory::new();
        let package = test_package();
        initialize_repository(
            &temporary.0.join(package.name),
            "https://github.com/ProjectMambo/MamboTools.git",
        );

        let message = delete(&temporary.0, package).expect("matching clone should be deleted");
        assert_eq!(message, "deleted MamboTools");
        assert!(!temporary.0.join(package.name).exists());
    }

    #[test]
    fn delete_preserves_a_clone_with_the_wrong_origin() {
        let temporary = TemporaryDirectory::new();
        let package = test_package();
        let repository = temporary.0.join(package.name);
        initialize_repository(
            &repository,
            "https://github.com/someone-else/MamboTools.git",
        );

        let error = delete(&temporary.0, package).expect_err("wrong origin must be rejected");
        assert!(
            error
                .to_string()
                .contains("origin is not ProjectMambo/MamboTools")
        );
        assert!(repository.exists());
    }

    fn test_package() -> Package {
        Package {
            name: "MamboTools",
            summary: "",
        }
    }

    fn initialize_repository(path: &Path, origin: &str) {
        fs::create_dir_all(path).expect("test repository directory should be created");
        let initialized = Command::new("git")
            .args(["init", "--quiet"])
            .arg(path)
            .status()
            .expect("git should be available for repository tests");
        assert!(initialized.success());
        let configured = Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["remote", "add", "origin", origin])
            .status()
            .expect("git should configure the test origin");
        assert!(configured.success());
    }

    struct TemporaryDirectory(PathBuf);

    impl TemporaryDirectory {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should follow the Unix epoch")
                .as_nanos();
            let path = std::env::temp_dir()
                .join(format!("mambo-tools-test-{}-{unique}", std::process::id()));
            fs::create_dir(&path).expect("temporary test directory should be created");
            Self(path)
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.0) {
                assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
            }
        }
    }
}
