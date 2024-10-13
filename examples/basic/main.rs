use anyhow::Result;
use file_split_to_directory::FileSplitToDirectoryBuilder;
use std::num::NonZeroUsize;

fn main() -> Result<()> {
    FileSplitToDirectoryBuilder::default()
        .with_path("/tmp/test".into())
        .with_chunk(NonZeroUsize::new(100).unwrap())
        .build()?
        .execute()?;
    Ok(())
}
