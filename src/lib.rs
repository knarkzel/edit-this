use std::{
    env::var,
    path::Path,
    process::{Command, Stdio},
};

/// Anyhow like Result<T> type.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Edits file in "EDITOR".
pub fn edit_file<P: AsRef<Path>>(file: P) -> Result<()> {
    let editor = var("EDITOR")?;
    Command::new(&editor)
        .arg(file.as_ref())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}
