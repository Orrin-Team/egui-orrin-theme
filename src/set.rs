//! Every theme a session can switch between: the built-in first, then whatever
//! a folder of user `*.toml` files holds.

use std::path::{Path, PathBuf};

use crate::Theme;

pub struct ThemeSet {
    themes: Vec<Theme>,
    active: usize,
}

impl Default for ThemeSet {
    fn default() -> Self {
        Self {
            themes: vec![Theme::default()],
            active: 0,
        }
    }
}

/// A theme file that was skipped, and why.
#[derive(Debug)]
pub enum LoadError {
    Read(PathBuf, std::io::Error),
    Parse(PathBuf, toml::de::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(path, e) => write!(f, "cannot read {}: {e}", path.display()),
            Self::Parse(path, e) => write!(f, "{} is not a theme: {e}", path.display()),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read(_, e) => Some(e),
            Self::Parse(_, e) => Some(e),
        }
    }
}

impl ThemeSet {
    /// Read `*.toml` themes from `dir`, in filename order so two machines list
    /// them the same way. A theme that fails to read or parse is skipped and
    /// returned beside the set — one bad file must not cost the user the rest
    /// of their set. A missing `dir` is not an error: it gives the built-in.
    pub fn load(dir: &Path) -> (Self, Vec<LoadError>) {
        let mut set = Self::default();
        let mut errors = Vec::new();

        let Ok(entries) = std::fs::read_dir(dir) else {
            return (set, errors);
        };
        let mut paths: Vec<_> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
            .collect();
        paths.sort();

        for path in paths {
            match std::fs::read_to_string(&path) {
                Ok(text) => match toml::from_str::<Theme>(&text) {
                    Ok(theme) => set.themes.push(theme),
                    Err(e) => errors.push(LoadError::Parse(path, e)),
                },
                Err(e) => errors.push(LoadError::Read(path, e)),
            }
        }
        (set, errors)
    }

    pub fn push(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn active(&self) -> &Theme {
        &self.themes[self.active]
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.themes.iter().map(|theme| theme.name.as_str())
    }

    /// Selects by name, returning whether anything changed. A name that no
    /// longer exists (a theme file deleted since it was last chosen) leaves the
    /// current selection alone rather than resetting it.
    pub fn select(&mut self, name: &str) -> bool {
        match self.themes.iter().position(|theme| theme.name == name) {
            Some(index) if index != self.active => {
                self.active = index;
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BUILT_IN;

    #[test]
    fn selecting_an_absent_theme_keeps_the_current_one() {
        let mut set = ThemeSet::default();
        assert!(!set.select("Nothing By That Name"));
        assert_eq!(set.active().name, BUILT_IN);
    }

    #[test]
    fn a_missing_themes_directory_still_gives_the_built_in() {
        let (set, errors) = ThemeSet::load(Path::new("this/does/not/exist"));
        assert_eq!(set.names().collect::<Vec<_>>(), [BUILT_IN]);
        assert!(errors.is_empty());
    }

    #[test]
    fn a_bad_file_is_reported_and_the_rest_still_load() {
        let (set, errors) =
            ThemeSet::load(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/themes")));
        assert!(errors.is_empty(), "{errors:?}");
        assert!(set.names().any(|name| name == "Void"));

        let dir = std::env::temp_dir().join(format!("egui-orrin-theme-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.toml"), "name = \"half a theme\"").unwrap();
        std::fs::copy(
            concat!(env!("CARGO_MANIFEST_DIR"), "/themes/void.toml"),
            dir.join("b.toml"),
        )
        .unwrap();

        let (set, errors) = ThemeSet::load(&dir);
        std::fs::remove_dir_all(&dir).unwrap();

        assert_eq!(set.names().collect::<Vec<_>>(), [BUILT_IN, "Void"]);
        assert!(
            matches!(&errors[..], [LoadError::Parse(path, _)] if path.ends_with("a.toml")),
            "{errors:?}"
        );
    }
}
