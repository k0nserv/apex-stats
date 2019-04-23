use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

extern crate directories;
use directories::ProjectDirs;

pub trait Directories {
    fn data_dir(&self) -> &Path;
}

impl Directories for ProjectDirs {
    fn data_dir(&self) -> &Path {
        self.data_dir()
    }
}

pub struct Config {
    directories: Box<dyn Directories>,
}

impl Config {
    fn new(directories: Box<Directories>) -> Self {
        Self { directories }
    }

    pub fn data_dir(&self) -> &Path {
        self.directories.data_dir()
    }

    pub fn data_path(&self) -> PathBuf {
        self.directories.data_dir().join("log.csv")
    }

    pub fn ensure_data_path_exists(&self) -> Result<(), std::io::Error> {
        create_dir_all(self.directories.data_dir())
    }
}

pub fn make_config() -> Option<Config> {
    if let Some(project_dirs) = ProjectDirs::from("", "", "apex-stats") {
        Some(Config::new(Box::new(project_dirs)))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{Config, Directories};
    use std::path::{Path, PathBuf};

    struct MockDirectories {
        data_dir: PathBuf,
    }

    impl MockDirectories {
        fn new() -> Self {
            Self {
                data_dir: Path::new("/some/path").to_owned(),
            }
        }
    }

    impl Directories for MockDirectories {
        fn data_dir(&self) -> &Path {
            &self.data_dir
        }
    }

    fn make_test_config() -> Config {
        Config::new(Box::new(MockDirectories::new()))
    }

    #[test]
    fn test_data_path() {
        let config = make_test_config();

        assert_eq!(
            config.data_path(),
            Path::new("/some/path/log.csv").to_owned()
        );
    }
}
