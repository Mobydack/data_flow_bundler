use std::fmt::Display;
use std::fs;
use std::io::{ Error as IOError };
use super::resource::Resource;

pub enum FileSystemError {
    FileTypeError(String),
    PermissionsError(String),
}

/// Strategies for event when size is excited.
pub enum SizeLimitStrategy {
    /// Prevent write data to file and all.
    Quite,
}


/// Size limit settings for file
pub struct SizeLimit {
    size: u16,
    strategy: SizeLimitStrategy,
}

/// File system resource
pub struct FileSystem {
    /// Maximum file size limit
    max_size: u16,
    /// Path to directory
    path_to_storage_dir: String,
}

impl Resource<IOError> for FileSystem {
    fn is_resource_allowed(&self) -> Result<bool> {
        let dir_metadata = fs::metadata(&self.path_to_storage_dir);

        if
    }
}

