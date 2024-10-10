use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    path: String,

    #[arg(
        short,
        long = "chunk",
        default_value_t = 4400,
        help = "each folder with [chunk] files"
    )]
    chunk: usize,
}

impl Args {
    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(self.path.as_str())
    }
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    split_files(&args.path_buf(), args.chunk)?;
    Ok(())
}

fn split_files(root: &PathBuf, chunk: usize) -> Result<()> {
    let chunks = fs::read_dir(root)?
        .enumerate()
        .filter_map(|(_, f)| f.ok())
        .filter(|f| {
            let Ok(t) = f.file_type() else {
                return false;
            };
            t.is_file()
        })
        .sorted_by(|a, b| {
            natord::compare(
                &a.file_name().to_string_lossy(),
                &b.file_name().to_string_lossy(),
            )
        })
        .chunks(chunk);

    for (i, chunk) in chunks.into_iter().enumerate() {
        let target_root = root.join(i.to_string());
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
