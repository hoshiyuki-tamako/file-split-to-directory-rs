use anyhow::Result;
use file_split_to_directory::FileSplitToDirectoryBuilder;
use std::fs::DirEntry;

fn main() -> Result<()> {
    // default order
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_sort_cmp(|a: &DirEntry, b: &DirEntry| {
            natord::compare(
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
