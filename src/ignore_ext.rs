//! This module extends the API of some types provided by the ignore trait.

use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

pub trait DirEntryExt {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
}

impl DirEntryExt for DirEntry {
    fn is_dir(&self) -> bool {
        self.file_type()
            .map(|file_type| file_type.is_dir())
            .unwrap_or_default()
    }

    fn is_file(&self) -> bool {
        self.file_type()
            .map(|file_type| file_type.is_file())
            .unwrap_or_default()
    }
}

pub trait WalkBuilderExt: Sized {
    fn try_from_iter<I, P>(paths: I) -> Option<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>;
}

impl WalkBuilderExt for WalkBuilder {
    fn try_from_iter<I, P>(paths: I) -> Option<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut paths = paths.into_iter();
        let first_path = paths.next()?;
        Some(
            paths.fold(WalkBuilder::new(first_path), |mut builder, path| {
                builder.add(path);
                builder
            }),
        )
    }
}
