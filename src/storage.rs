use std::{
    cmp::Reverse,
    fs::{self, File},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::Deserialize;
use serde_json::to_string_pretty;
use std::io::{self, Write};

use crate::parser::{SortType, Task};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "kapj1coh"; // unique ID
const APPLICATION: &str = "prog-rs";

#[derive(Debug)]
pub enum StorageError {
    IoError(io::Error),
    SerdeError(String),
}

fn get_task_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
        // On Fedora: ~/.local/share/prog-rs/
        // On Mac:    ~/Library/Application Support/com.my_username.prog-rs/
        // On Win:    C:\Users\You\AppData\Roaming\my_username\prog-rs\
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Could not create data directory");
        }

        data_dir.join("tasks.json")
    } else {
        // Fallback: just use current directory
        PathBuf::from("tasks.json")
    }
}

pub struct TaskStorage {
    pub tasks: Vec<Task>,
    path: PathBuf,
}

impl TaskStorage {
    pub fn new() -> Self {
        let path = get_task_path();

        let tasks = Vec::new();
        Self { tasks, path }
    }

    pub fn load(&mut self) -> Result<(), StorageError> {
        if self.path.exists() {
            let data = match fs::read_to_string(&self.path) {
                Ok(data) => data,
                Err(e) => return Err(StorageError::IoError(e)),
            };

            let metadata = fs::metadata(&self.path).map_err(StorageError::IoError)?;
            // If file is empty, don't even try to parse
            if metadata.len() == 0 {
                self.tasks = Vec::new();
                return Ok(());
            }

            let tasks = {
                let this = serde_json::from_str(&data);
                match this {
                    Ok(t) => t,
                    Err(e) => return Err(StorageError::SerdeError(format!("{:?}", e))),
                }
            };
            self.tasks = tasks;
            Ok(())
        } else {
            self.tasks = Vec::new();
            Ok(())
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn clear(&mut self) -> io::Result<()> {
        File::create(&self.path)?;
        Ok(())
    }

    pub fn store(&mut self) -> io::Result<()> {
        let temp_file_path = self.path.with_extension("tmp");

        let mut file = File::create(&temp_file_path)?;
        let json = to_string_pretty(&self.tasks)?;

        file.write_all(json.as_bytes())
            .expect("Could not write tasks to file");

        fs::rename(temp_file_path, &self.path)?;

        Ok(())
    }

    pub fn sort(&mut self, sorting: SortType) {
        match sorting {
            SortType::Alphabetical => self.tasks.sort_by(|a, b| a.name.cmp(&b.name)),
            SortType::ClosestToDeadline => {
                self.tasks.sort_by_key(|p| p.due_date);
            }
            SortType::FurthestFromDeadline => {
                self.tasks.sort_by_key(|p| Reverse(p.due_date));
            }
        }
    }
}
