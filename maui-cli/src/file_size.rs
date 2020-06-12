use crate::prelude::*;
use std::path::Path;

pub fn file_size(path: &Path) -> crate::Result<u64> {
    path.metadata().map(|meta| meta.len()).wrap()
}

pub fn human_readable_file_size(path: &Path, format: &str) -> crate::Result<String> {
    let format = format.trim().to_lowercase();
    file_size(path).map(|size| {
        if format == "kb" {
            format!("{}", size / 1024)
        } else {
            format!("{}", size)
        }
    })
}
