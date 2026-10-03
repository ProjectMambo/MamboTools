//! The Project Mambo repository catalog.

/// A repository managed by `MamboTools`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Package {
    pub name: &'static str,
    pub summary: &'static str,
}

impl Package {
    /// Returns the public clone URL for this package.
    #[must_use]
    pub fn clone_url(self) -> String {
        format!("https://github.com/ProjectMambo/{}.git", self.name)
    }
}

/// Repositories currently published by Project Mambo.
pub const PACKAGES: &[Package] = &[
    Package {
        name: "MamboColour",
        summary: "Colour tools and palettes",
    },
    Package {
        name: "MamboDocs",
        summary: "Project standards and documentation",
    },
    Package {
        name: "MamboDot",
        summary: "Dotfile management",
    },
    Package {
        name: "MamboFinance",
        summary: "Personal finance tools",
    },
    Package {
        name: "MamboFolio",
        summary: "Portfolio tooling",
    },
    Package {
        name: "MamboFont",
        summary: "Font tools",
    },
    Package {
        name: "MamboMeme",
        summary: "Meme tools",
    },
    Package {
        name: "MamboSite",
        summary: "Project Mambo website",
    },
    Package {
        name: "MamboTools",
        summary: "Repository manager",
    },
    Package {
        name: "MamboUI",
        summary: "Shared terminal UI components",
    },
    Package {
        name: "MamboWiki",
        summary: "Knowledge and wiki tools",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_urls_target_the_project_mambo_organization() {
        let package = Package {
            name: "MamboTools",
            summary: "",
        };

        assert_eq!(
            package.clone_url(),
            "https://github.com/ProjectMambo/MamboTools.git"
        );
    }

    #[test]
    fn catalog_names_are_unique_and_safe() {
        for (index, package) in PACKAGES.iter().enumerate() {
            assert!(crate::paths::is_safe_package_name(package.name));
            assert!(!package.summary.is_empty());
            assert!(
                !PACKAGES[..index]
                    .iter()
                    .any(|item| item.name == package.name)
            );
        }
    }
}
