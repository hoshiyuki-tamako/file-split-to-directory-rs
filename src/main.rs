use anyhow::Result;
use clap::Parser;
use file_split_to_directory::FileSplitToDirectoryBuilder;
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    path: PathBuf,

    #[arg(
        short,
        long = "chunk",
        default_value_t = 4400.try_into().unwrap(),
        help = "each folder with [chunk] files"
    )]
    chunk: NonZeroUsize,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    FileSplitToDirectoryBuilder::default()
        .with_path(args.path)
        .with_chunk(args.chunk)
        .build()?
        .execute()?;
    Ok(())
}
