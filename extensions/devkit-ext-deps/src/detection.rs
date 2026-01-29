//! Package detection logic

use std::path::{Path, PathBuf};

/// Package manager types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    // Rust
    Cargo,

    // JavaScript/TypeScript
    Npm,
    Yarn,
    Pnpm,
    Bun,

    // Python
    Pip,
    Poetry,
    Pipenv,
    Uv,

    // Ruby
    Bundler,

    // Go
    GoMod,

    // Java
    Maven,
    Gradle,

    // PHP
    Composer,

    // .NET
    Dotnet,

    // Elixir
    Mix,
}

impl PackageManager {
    /// Get the name of the package manager
    pub fn name(&self) -> &'static str {
        match self {
            PackageManager::Cargo => "cargo",
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
            PackageManager::Pip => "pip",
            PackageManager::Poetry => "poetry",
            PackageManager::Pipenv => "pipenv",
            PackageManager::Uv => "uv",
            PackageManager::Bundler => "bundle",
            PackageManager::GoMod => "go",
            PackageManager::Maven => "mvn",
            PackageManager::Gradle => "gradle",
            PackageManager::Composer => "composer",
            PackageManager::Dotnet => "dotnet",
            PackageManager::Mix => "mix",
        }
    }

    /// Get the install command
    pub fn install_cmd(&self) -> Vec<&'static str> {
        match self {
            PackageManager::Cargo => vec!["cargo", "fetch"],
            PackageManager::Npm => vec!["npm", "install"],
            PackageManager::Yarn => vec!["yarn", "install"],
            PackageManager::Pnpm => vec!["pnpm", "install"],
            PackageManager::Bun => vec!["bun", "install"],
            PackageManager::Pip => vec!["pip", "install", "-r", "requirements.txt"],
            PackageManager::Poetry => vec!["poetry", "install"],
            PackageManager::Pipenv => vec!["pipenv", "install"],
            PackageManager::Uv => vec!["uv", "pip", "install", "-r", "requirements.txt"],
            PackageManager::Bundler => vec!["bundle", "install"],
            PackageManager::GoMod => vec!["go", "mod", "download"],
            PackageManager::Maven => vec!["mvn", "dependency:resolve"],
            PackageManager::Gradle => vec!["gradle", "dependencies"],
            PackageManager::Composer => vec!["composer", "install"],
            PackageManager::Dotnet => vec!["dotnet", "restore"],
            PackageManager::Mix => vec!["mix", "deps.get"],
        }
    }

    /// Check if this package manager is installed
    pub fn is_available(&self) -> bool {
        devkit_core::cmd_exists(self.name())
    }
}

/// Language type detected for a package
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Ruby,
    Go,
    Java,
    PHP,
    CSharp,
    Elixir,
}

impl Language {
    pub fn name(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Ruby => "Ruby",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::PHP => "PHP",
            Language::CSharp => "C#",
            Language::Elixir => "Elixir",
        }
    }
}

/// Information about a discovered package
#[derive(Debug)]
pub struct PackageInfo {
    /// Package directory path
    pub path: PathBuf,
    /// Package name (from Cargo.toml or package.json)
    pub name: String,
    /// Detected language
    pub language: Language,
    /// Detected package manager
    pub package_manager: PackageManager,
    /// Whether dependencies need to be installed
    pub needs_install: bool,
}

impl PackageInfo {
    /// Detect package information from a directory
    pub fn detect(path: &Path) -> Option<Self> {
        // Try each language detector
        None.or_else(|| Self::detect_rust(path))
            .or_else(|| Self::detect_node(path))
            .or_else(|| Self::detect_python(path))
            .or_else(|| Self::detect_ruby(path))
            .or_else(|| Self::detect_go(path))
            .or_else(|| Self::detect_java(path))
            .or_else(|| Self::detect_php(path))
            .or_else(|| Self::detect_dotnet(path))
            .or_else(|| Self::detect_elixir(path))
    }

    /// Detect Rust package
    fn detect_rust(path: &Path) -> Option<Self> {
        let cargo_toml = path.join("Cargo.toml");
        if !cargo_toml.exists() {
            return None;
        }

        // Parse package name from Cargo.toml
        let content = std::fs::read_to_string(&cargo_toml).ok()?;
        let parsed: toml::Value = toml::from_str(&content).ok()?;
        let name = parsed
            .get("package")?
            .get("name")?
            .as_str()?
            .to_string();

        // Check if dependencies need installing
        let needs_install = Self::rust_needs_install(path);

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Rust,
            package_manager: PackageManager::Cargo,
            needs_install,
        })
    }

    /// Detect Node/TypeScript package
    fn detect_node(path: &Path) -> Option<Self> {
        let package_json = path.join("package.json");
        if !package_json.exists() {
            return None;
        }

        // Parse package name from package.json
        let content = std::fs::read_to_string(&package_json).ok()?;
        let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
        let name_value = parsed.get("name")?;
        let name = name_value.as_str()?.to_string();

        // Detect if TypeScript
        let has_tsconfig = path.join("tsconfig.json").exists();
        let language = if has_tsconfig {
            Language::TypeScript
        } else {
            Language::JavaScript
        };

        // Detect package manager
        let package_manager = Self::detect_node_package_manager(path);

        // Check if dependencies need installing
        let needs_install = Self::node_needs_install(path, package_manager);

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language,
            package_manager,
            needs_install,
        })
    }

    /// Detect which Node package manager to use
    fn detect_node_package_manager(path: &Path) -> PackageManager {
        // Check for lock files to determine package manager
        if path.join("bun.lockb").exists() {
            PackageManager::Bun
        } else if path.join("pnpm-lock.yaml").exists() {
            PackageManager::Pnpm
        } else if path.join("yarn.lock").exists() {
            PackageManager::Yarn
        } else {
            // Default to npm (package-lock.json or no lock file)
            PackageManager::Npm
        }
    }

    /// Detect Python package
    fn detect_python(path: &Path) -> Option<Self> {
        // Check for various Python project files
        let has_pyproject = path.join("pyproject.toml").exists();
        let has_requirements = path.join("requirements.txt").exists();
        let has_setup_py = path.join("setup.py").exists();
        let has_pipfile = path.join("Pipfile").exists();

        if !has_pyproject && !has_requirements && !has_setup_py && !has_pipfile {
            return None;
        }

        // Detect package manager
        let package_manager = if has_pyproject {
            // Check if it's a Poetry or UV project
            if let Ok(content) = std::fs::read_to_string(path.join("pyproject.toml")) {
                if content.contains("[tool.poetry]") {
                    PackageManager::Poetry
                } else if content.contains("[tool.uv]") {
                    PackageManager::Uv
                } else {
                    PackageManager::Pip
                }
            } else {
                PackageManager::Pip
            }
        } else if has_pipfile {
            PackageManager::Pipenv
        } else {
            PackageManager::Pip
        };

        // Try to get package name from pyproject.toml or setup.py
        let name = path
            .file_name()?
            .to_str()?
            .to_string();

        let needs_install = Self::python_needs_install(path, package_manager);

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Python,
            package_manager,
            needs_install,
        })
    }

    /// Detect Ruby package
    fn detect_ruby(path: &Path) -> Option<Self> {
        let gemfile = path.join("Gemfile");
        if !gemfile.exists() {
            return None;
        }

        let name = path.file_name()?.to_str()?.to_string();
        let needs_install = !path.join("Gemfile.lock").exists()
            || Self::file_newer_than(&gemfile, &path.join("vendor/bundle"));

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Ruby,
            package_manager: PackageManager::Bundler,
            needs_install,
        })
    }

    /// Detect Go package
    fn detect_go(path: &Path) -> Option<Self> {
        let go_mod = path.join("go.mod");
        if !go_mod.exists() {
            return None;
        }

        // Parse module name from go.mod
        let content = std::fs::read_to_string(&go_mod).ok()?;
        let name = content
            .lines()
            .find(|line| line.starts_with("module "))
            .and_then(|line| line.strip_prefix("module "))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });

        let needs_install = !path.join("go.sum").exists()
            || Self::file_newer_than(&go_mod, &path.join("go.sum"));

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Go,
            package_manager: PackageManager::GoMod,
            needs_install,
        })
    }

    /// Detect Java package (Maven or Gradle)
    fn detect_java(path: &Path) -> Option<Self> {
        let has_pom = path.join("pom.xml").exists();
        let has_gradle = path.join("build.gradle").exists() || path.join("build.gradle.kts").exists();

        if !has_pom && !has_gradle {
            return None;
        }

        let package_manager = if has_gradle {
            PackageManager::Gradle
        } else {
            PackageManager::Maven
        };

        let name = path.file_name()?.to_str()?.to_string();
        let needs_install = true; // Always check for Java projects

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Java,
            package_manager,
            needs_install,
        })
    }

    /// Detect PHP package
    fn detect_php(path: &Path) -> Option<Self> {
        let composer_json = path.join("composer.json");
        if !composer_json.exists() {
            return None;
        }

        // Parse package name from composer.json
        let content = std::fs::read_to_string(&composer_json).ok()?;
        let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
        let name = parsed
            .get("name")?
            .as_str()?
            .to_string();

        let needs_install = !path.join("vendor").exists()
            || Self::file_newer_than(&composer_json, &path.join("vendor"));

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::PHP,
            package_manager: PackageManager::Composer,
            needs_install,
        })
    }

    /// Detect .NET package
    fn detect_dotnet(path: &Path) -> Option<Self> {
        // Look for .csproj, .fsproj, or .vbproj files
        let entries = std::fs::read_dir(path).ok()?;
        let has_project = entries
            .filter_map(|e| e.ok())
            .any(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| matches!(ext, "csproj" | "fsproj" | "vbproj"))
                    .unwrap_or(false)
            });

        if !has_project {
            return None;
        }

        let name = path.file_name()?.to_str()?.to_string();
        let needs_install = true; // Always check for .NET projects

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::CSharp,
            package_manager: PackageManager::Dotnet,
            needs_install,
        })
    }

    /// Detect Elixir package
    fn detect_elixir(path: &Path) -> Option<Self> {
        let mix_exs = path.join("mix.exs");
        if !mix_exs.exists() {
            return None;
        }

        let name = path.file_name()?.to_str()?.to_string();
        let needs_install = !path.join("deps").exists()
            || Self::file_newer_than(&mix_exs, &path.join("mix.lock"));

        Some(PackageInfo {
            path: path.to_path_buf(),
            name,
            language: Language::Elixir,
            package_manager: PackageManager::Mix,
            needs_install,
        })
    }

    /// Helper: Check if file A is newer than file/dir B
    fn file_newer_than(a: &Path, b: &Path) -> bool {
        if !b.exists() {
            return true;
        }

        if let (Ok(a_meta), Ok(b_meta)) = (std::fs::metadata(a), std::fs::metadata(b)) {
            if let (Ok(a_time), Ok(b_time)) = (a_meta.modified(), b_meta.modified()) {
                return a_time > b_time;
            }
        }

        false
    }

    /// Check if Python dependencies need installing
    fn python_needs_install(path: &Path, package_manager: PackageManager) -> bool {
        match package_manager {
            PackageManager::Poetry => {
                let poetry_lock = path.join("poetry.lock");
                !poetry_lock.exists() || Self::file_newer_than(&path.join("pyproject.toml"), &poetry_lock)
            }
            PackageManager::Pipenv => {
                let pipfile_lock = path.join("Pipfile.lock");
                !pipfile_lock.exists() || Self::file_newer_than(&path.join("Pipfile"), &pipfile_lock)
            }
            _ => {
                // For pip and uv, check if requirements.txt is newer than venv
                let venv_dir = if path.join("venv").exists() {
                    path.join("venv")
                } else {
                    path.join(".venv")
                };
                !venv_dir.exists() || Self::file_newer_than(&path.join("requirements.txt"), &venv_dir)
            }
        }
    }

    /// Check if Rust dependencies need installing
    fn rust_needs_install(path: &Path) -> bool {
        let cargo_toml = path.join("Cargo.toml");
        let cargo_lock = path.join("Cargo.lock");

        // If no Cargo.lock, definitely needs install
        if !cargo_lock.exists() {
            return true;
        }

        // Check if Cargo.toml is newer than Cargo.lock
        if let (Ok(toml_meta), Ok(lock_meta)) = (
            std::fs::metadata(&cargo_toml),
            std::fs::metadata(&cargo_lock),
        ) {
            if let (Ok(toml_time), Ok(lock_time)) =
                (toml_meta.modified(), lock_meta.modified())
            {
                if toml_time > lock_time {
                    return true;
                }
            }
        }

        false
    }

    /// Check if Node dependencies need installing
    fn node_needs_install(path: &Path, package_manager: PackageManager) -> bool {
        let package_json = path.join("package.json");
        let node_modules = path.join("node_modules");

        // If no node_modules, definitely needs install
        if !node_modules.exists() {
            return true;
        }

        // Check lock file vs node_modules timestamp
        let lock_file = match package_manager {
            PackageManager::Pnpm => path.join("pnpm-lock.yaml"),
            PackageManager::Yarn => path.join("yarn.lock"),
            PackageManager::Npm => path.join("package-lock.json"),
            _ => return false,
        };

        // If lock file exists and is newer than node_modules, needs install
        if lock_file.exists() {
            if let (Ok(lock_meta), Ok(modules_meta)) = (
                std::fs::metadata(&lock_file),
                std::fs::metadata(&node_modules),
            ) {
                if let (Ok(lock_time), Ok(modules_time)) =
                    (lock_meta.modified(), modules_meta.modified())
                {
                    if lock_time > modules_time {
                        return true;
                    }
                }
            }
        }

        // Check if package.json is newer than node_modules
        if let (Ok(json_meta), Ok(modules_meta)) = (
            std::fs::metadata(&package_json),
            std::fs::metadata(&node_modules),
        ) {
            if let (Ok(json_time), Ok(modules_time)) =
                (json_meta.modified(), modules_meta.modified())
            {
                if json_time > modules_time {
                    return true;
                }
            }
        }

        false
    }
}
