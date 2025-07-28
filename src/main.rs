use std::{ffi::OsStr, path::Path};
use anyhow::{ensure, Result};

pub mod dvpl;

#[cfg(test)]
mod tests;

fn process_file(path: &Path) -> Result<()> {
  let input = std::fs::read(path)?;

  if path.extension().and_then(OsStr::to_str) == Some("dvpl") {
    std::fs::write(path.with_extension(""), dvpl::decompress(&input)?)?;
  }
  else {
    std::fs::write(format!("{}.dvpl", path.display()), dvpl::compress(&input)?)?;
  };

  std::fs::remove_file(path)?;

  Ok(())
}

fn process_dir(path: &Path) -> Result<()> {
  for dir in std::fs::read_dir(path)? {
    let path = dir?.path();

    if path.is_dir() {
      process_dir(&path)?;
    }
    else if path.is_file() {
      let _ = process_file(&path).inspect_err(|e| {
        println!("failed to process {}: {e}", path.file_name().unwrap().display());
      });
    }
  }

  Ok(())
}

fn run() -> Result<()> {
  let args = std::env::args().collect::<Vec<String>>();

  ensure!(args.len() == 2, "invalid number of arguments");

  let path = Path::new(&args[1]);

  ensure!(std::fs::exists(path)?, "path doesn't exist");

  if path.is_file() {
    process_file(path)?;
  }
  else {
    process_dir(path)?;
  }

  Ok(())
}

fn main() {
  let _ = run().inspect_err(|e| {
    eprintln!("ERROR: {e}");
    let _ = std::io::stdin().read_line(&mut String::new());
  });
}