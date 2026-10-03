//! Install-root discovery and managed-path validation.

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Resolves the directory where managed repositories are stored.
///
/// `MAMBO_REPOS_DIR` overrides the XDG location. Otherwise this uses
/// `$XDG_DATA_HOME/mambo/repos`, then `$HOME/.local/share/mambo/repos`.
///
/// # Errors
///
/// Returns an error when none of those environment variables supplies a root.
pub fn install_root() -> Result<PathBuf, String> {
    install_root_from(
        env::var_os("MAMBO_REPOS_DIR").as_deref(),
        env::var_os("XDG_DATA_HOME").as_deref(),
        env::var_os("HOME").as_deref(),
    )
}

fn install_root_from(
    mambo_repos_dir: Option<&OsStr>,
    xdg_data_home: Option<&OsStr>,
    home: Option<&OsStr>,
) -> Result<PathBuf, String> {
    if let Some(root) = mambo_repos_dir.filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(root));
    }
    if let Some(root) = xdg_data_home.filter(|value| !value.is_empty()) {
        return Ok(PathBuf::from(root).join("mambo/repos"));
    }
    home.filter(|value| !value.is_empty())
        .map(|root| PathBuf::from(root).join(".local/share/mambo/repos"))
        .ok_or_else(|| "cannot choose an install directory: set MAMBO_REPOS_DIR or HOME".to_owned())
}

/// Returns true when a catalog name can safely become one path component.
#[must_use]
pub fn is_safe_package_name(name: &str) -> bool {
    !name.is_empty()
        && name != "."
        && name != ".."
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// Builds a path guaranteed to be one direct child of the install root.
///
/// # Errors
///
/// Returns an error when `name` is not one safe path component.
pub fn managed_path(root: &Path, name: &str) -> Result<PathBuf, String> {
    if !is_safe_package_name(name) {
        return Err(format!("unsafe package name: {name:?}"));
    }
    Ok(root.join(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_root_wins_over_xdg_and_home() {
        assert_eq!(
            install_root_from(
                Some(OsStr::new("/custom")),
                Some(OsStr::new("/xdg")),
                Some(OsStr::new("/home/me")),
            ),
            Ok(PathBuf::from("/custom"))
        );
    }

    #[test]
    fn xdg_root_precedes_home_fallback() {
        assert_eq!(
            install_root_from(None, Some(OsStr::new("/xdg")), Some(OsStr::new("/home/me"))),
            Ok(PathBuf::from("/xdg/mambo/repos"))
        );
    }

    #[test]
    fn home_fallback_matches_the_xdg_default() {
        assert_eq!(
            install_root_from(None, None, Some(OsStr::new("/home/me"))),
            Ok(PathBuf::from("/home/me/.local/share/mambo/repos"))
        );
    }

    #[test]
    fn managed_paths_reject_traversal_and_separators() {
        let root = Path::new("/repos");

        assert_eq!(managed_path(root, "MamboUI"), Ok(root.join("MamboUI")));
        assert!(managed_path(root, "../outside").is_err());
        assert!(managed_path(root, "nested/repo").is_err());
        assert!(managed_path(root, "").is_err());
    }
}
