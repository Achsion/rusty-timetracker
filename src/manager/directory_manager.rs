use directories::ProjectDirs;
use std::env::current_dir;
use std::fs::{create_dir_all, metadata};
use std::io;
use std::path::PathBuf;

pub enum DirectoryType {
    Config,
    Data,
}

impl DirectoryType {
    pub fn setup_directory(
        self,
        qualifier: &str,
        organization: &str,
        application: &str,
    ) -> Result<PathBuf, io::Error> {
        let dir_path = if cfg!(debug_assertions) {
            let mut working_dir = current_dir()?;
            working_dir.push("timetracker-tmp");
            working_dir
        } else {
            get_specific_directory(&self, qualifier, organization, application)
        };

        if metadata(&dir_path).is_err() {
            create_dir_all(&dir_path)?;
        }

        Ok(dir_path)
    }
}

fn get_specific_directory(
    dir_type: &DirectoryType,
    qualifier: &str,
    organization: &str,
    application: &str,
) -> PathBuf {
    let project_dirs = ProjectDirs::from(qualifier, organization, application).unwrap();

    let path = match dir_type {
        DirectoryType::Config => project_dirs.config_dir(),
        DirectoryType::Data => project_dirs.data_dir(),
    };

    path.to_path_buf()
}
