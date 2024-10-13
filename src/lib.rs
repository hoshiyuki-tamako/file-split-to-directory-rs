use itertools::Itertools;
use std::fs::{self, DirEntry};
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileSplitToDirectory {
    path: PathBuf,
    chunk: NonZeroUsize,
    sort_cmp: fn(&DirEntry, &DirEntry) -> std::cmp::Ordering,
    directory_name: fn(usize) -> String,
}

impl FileSplitToDirectory {
    pub fn execute(&self) -> Result<(), std::io::Error> {
        let chunks = fs::read_dir(&self.path)?
            .enumerate()
            .filter_map(|(_, f)| f.ok())
            .filter(|f| {
                let Ok(t) = f.file_type() else {
                    return false;
                };
                t.is_file()
            })
            .sorted_by(self.sort_cmp)
            .chunks(self.chunk.get());

        for (i, chunk) in chunks.into_iter().enumerate() {
            let target_root = self.path.join((self.directory_name)(i));
            if !target_root.exists() {
                fs::create_dir(&target_root)?;
            }

            for f in chunk {
                let to = target_root.join(f.file_name());
                fs::rename(f.path(), to)?;
            }
        }
        Ok(())
    }
}

type SortCmpFn = fn(&DirEntry, &DirEntry) -> std::cmp::Ordering;
type DirectoryNameGeneratorFn = fn(usize) -> String;

pub struct FileSplitToDirectoryBuilder {
    pub path: Option<PathBuf>,
    pub chunk: NonZeroUsize,
    pub sort_cmp: SortCmpFn,
    pub directory_name: DirectoryNameGeneratorFn,
}

impl FileSplitToDirectoryBuilder {
    pub fn default_sort_cmp(a: &DirEntry, b: &DirEntry) -> std::cmp::Ordering {
        natord::compare(
            &a.file_name().to_string_lossy(),
            &b.file_name().to_string_lossy(),
        )
    }

    pub fn default_directory_name(i: usize) -> String {
        i.to_string()
    }

    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_chunk(mut self, chunk: NonZeroUsize) -> Self {
        self.chunk = chunk;
        self
    }

    pub fn with_sort_cmp(mut self, sort_cmp: SortCmpFn) -> Self {
        self.sort_cmp = sort_cmp;
        self
    }

    pub fn with_directory_name(mut self, directory_name: DirectoryNameGeneratorFn) -> Self {
        self.directory_name = directory_name;
        self
    }

    pub fn build(&self) -> Result<FileSplitToDirectory, std::io::Error> {
        if let Some(path) = &self.path {
            Ok(FileSplitToDirectory {
                path: path.clone(),
                chunk: self.chunk,
                sort_cmp: self.sort_cmp.clone(),
                directory_name: self.directory_name.clone(),
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "path is not set",
            ))
        }
    }
}

impl Default for FileSplitToDirectoryBuilder {
    fn default() -> Self {
        Self {
            path: None,
            chunk: NonZeroUsize::new(4400).unwrap(),
            sort_cmp: Self::default_sort_cmp,
            directory_name: Self::default_directory_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn test_default() {
        assert!(FileSplitToDirectoryBuilder::default().build().is_err());
    }

    #[test]
    fn test_default_directory_name() {
        for _ in 0..100 {
            let v = Faker.fake::<usize>();
            assert_eq!(
                FileSplitToDirectoryBuilder::default_directory_name(v),
                v.to_string()
            );
        }
    }

    #[test]
    fn test_with_path() {
        let path = Faker.fake::<PathBuf>();
        let builder = FileSplitToDirectoryBuilder::default().with_path(path.clone());
        assert_eq!(builder.path, Some(path));
    }

    #[test]
    fn test_with_chunk() {
        let chunk = Faker.fake::<NonZeroUsize>();
        let builder = FileSplitToDirectoryBuilder::default().with_chunk(chunk);
        assert_eq!(builder.chunk, chunk);
    }

    // #[test]
    // fn test_with_sort_cmp() {
    //     todo!()
    // }

    // #[test]
    // fn test_with_directory_name() {
    //     todo!()
    // }

    // #[test]
    // fn test_execute() {
    //     todo!()
    // }
}
