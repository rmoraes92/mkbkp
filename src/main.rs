// #![feature(path_add_extension)]

use std::path::{Path, PathBuf};
use std::fs::rename;
use std::process;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// file path to backup
    file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    match create_bkp(&cli.file_path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("could not backup file: {}", e);
            process::exit(1);
        }
    };
}

fn append_extension(p: &Path, e: &str) -> PathBuf {
    PathBuf::from(format!("{}.{}", p.to_str().unwrap(), e))
}

fn create_bkp(src_path: &Path) -> Result<()> {
    let mut dst_path: PathBuf = append_extension(src_path, "bkp");
    while dst_path.is_file() {
        dst_path = append_extension(&dst_path, "bkp");
    }
    rename(src_path, dst_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cfo::{touch_file, remove_file};

    #[test]
    fn backups() {
        let p = Path::new("foo.txt");
        touch_file(p).unwrap();
        create_bkp(p).unwrap();
        assert!(Path::new("foo.txt.bkp").is_file());

        let p = Path::new("foo.txt");
        touch_file(p).unwrap();
        create_bkp(p).unwrap();
        assert!(Path::new("foo.txt.bkp.bkp").is_file());

        let p = Path::new("foo.txt");
        touch_file(p).unwrap();
        create_bkp(p).unwrap();
        assert!(Path::new("foo.txt.bkp.bkp.bkp").is_file());

        remove_file(Path::new("foo.txt.bkp")).unwrap();
        remove_file(Path::new("foo.txt.bkp.bkp")).unwrap();
        remove_file(Path::new("foo.txt.bkp.bkp.bkp")).unwrap();
    }
}

