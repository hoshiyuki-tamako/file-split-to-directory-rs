# File Split To Directory

split files to folders

## Install

```bash
cargo install file-split-to-directory
```

## Usage

### Command Line

```bash
file-split-to-directory .
```

```bash
file-split-to-directory "/mnt/d/download"
```

```bash
# each folder 4400 files (default is 4400)
file-split-to-directory "/mnt/d/download" --chunk 4400
file-split-to-directory "/mnt/d/download" -c 4400
```

### Result

from

```text
- Download
-- 1.png
-- 2.png
-- 3.png
-- ....
-- 4401.png
-- ....
-- 8801.png
```

to

```text
- Download
|- 0
||- 1.png
||- 2.png
||- 3.png
||- ...
|- 1
||- 4401.png
||- ...
|- 2
||- 8801.png
```

## Use as library

### Basic

```rs
use anyhow::Result;
use file_split_to_directory::FileSplitToDirectoryBuilder;
use std::num::NonZeroUsize;

fn main() -> Result<()> {
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_chunk(NonZeroUsize::new(100).unwrap())
        .build()? // std::io::Error if path = None
        .execute()?;
    Ok(())
}
```

### Custom directory name

```rs
use anyhow::Result;
use file_split_to_directory::FileSplitToDirectoryBuilder;

fn main() -> Result<()> {
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_directory_name(|i: usize| i.to_string())
        .build()?
        .execute()?;
    Ok(())
}
```

### Custom file order

```rs
use anyhow::Result;
use file_split_to_directory::FileSplitToDirectoryBuilder;
use std::fs::DirEntry;

fn main() -> Result<()> {
    // default order
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_sort_cmp(|a: &DirEntry, b: &DirEntry| {
            natord::compare( // cargo install natord
                &a.file_name().to_string_lossy(),
                &b.file_name().to_string_lossy(),
            )
        })
        .build()?
        .execute()?;

    // order file by size
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_sort_cmp(|a: &DirEntry, b: &DirEntry| {
            b.metadata()
                .map(|m| m.len())
                .unwrap_or_default()
                .cmp(&a.metadata().map(|m| m.len()).unwrap_or_default())
        })
        .build()?
        .execute()?;

    Ok(())
}
```
