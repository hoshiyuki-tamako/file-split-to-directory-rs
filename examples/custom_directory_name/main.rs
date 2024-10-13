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
