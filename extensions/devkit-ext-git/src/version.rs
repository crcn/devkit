//! Version parsing and management

use anyhow::{anyhow, Context, Result};
use devkit_core::AppContext;
use devkit_tasks::CmdBuilder;

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
}

impl Version {
    pub fn parse(s: &str) -> Result<Self> {
        // Strip v prefix (e.g., "v1.0.0" -> "1.0.0")
        let version_part = s.trim_start_matches('v');

        let (version_part, prerelease) = if let Some(idx) = version_part.find('-') {
            (
                &version_part[..idx],
                Some(version_part[idx + 1..].to_string()),
            )
        } else {
            (version_part, None)
        };

        let parts: Vec<&str> = version_part.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid version format: {}", s));
        }

        Ok(Version {
            major: parts[0].parse().context("Invalid major version")?,
            minor: parts[1].parse().context("Invalid minor version")?,
            patch: parts[2].parse().context("Invalid patch version")?,
            prerelease,
        })
    }

    pub fn to_tag(&self) -> String {
        let base = format!("v{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(ref pre) = self.prerelease {
            format!("{}-{}", base, pre)
        } else {
            base
        }
    }

    pub fn bump_major(&self) -> Self {
        Version {
            major: self.major + 1,
            minor: 0,
            patch: 0,
            prerelease: None,
        }
    }

    pub fn bump_minor(&self) -> Self {
        Version {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
            prerelease: None,
        }
    }

    pub fn bump_patch(&self) -> Self {
        Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
            prerelease: None,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.prerelease {
            write!(f, "-{}", pre)?;
        }
        Ok(())
    }
}

/// Get the current version from the latest v* tag
pub fn get_current_version(ctx: &AppContext) -> Result<Option<Version>> {
    let result = CmdBuilder::new("git")
        .args(["describe", "--tags", "--abbrev=0", "--match", "v*"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture();

    match result {
        Ok(output) if output.code == 0 => {
            let tag = output.stdout_string().trim().to_string();
            if tag.is_empty() {
                Ok(None)
            } else {
                Ok(Some(Version::parse(&tag)?))
            }
        }
        _ => Ok(None),
    }
}

/// Get list of recent version tags
pub fn get_recent_versions(ctx: &AppContext, count: u32) -> Result<Vec<String>> {
    let result = CmdBuilder::new("git")
        .args(["tag", "-l", "v*", "--sort=-version:refname"])
        .cwd(&ctx.repo)
        .capture_stdout()
        .run_capture()?;

    Ok(result
        .stdout_string()
        .lines()
        .take(count as usize)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parse() {
        let v = Version::parse("v1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
        assert_eq!(v.prerelease, None);

        let v = Version::parse("v1.2.3-beta.1").unwrap();
        assert_eq!(v.prerelease, Some("beta.1".to_string()));
    }

    #[test]
    fn test_version_bump() {
        let v = Version::parse("v1.2.3").unwrap();
        assert_eq!(v.bump_patch().to_tag(), "v1.2.4");
        assert_eq!(v.bump_minor().to_tag(), "v1.3.0");
        assert_eq!(v.bump_major().to_tag(), "v2.0.0");
    }
}
