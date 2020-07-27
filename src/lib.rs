mod app;
mod error;
mod input;
pub(crate) mod utils;

pub(crate) use self::input::{Replacer, Source};
pub(crate) use error::{Error, Result};

use std::path::PathBuf;

pub fn replacer(files: Vec<PathBuf>, find: String, replace_with: String) -> Result<()> {
    let source = Source::infer(files);
    let in_place = true;
    let flags = None;
    let literal_mode = false;

    let replacer = Replacer::new(
        find,
        replace_with,
        literal_mode,
        flags,
    )?;
    replacer.run(&source, in_place)?;
    Ok(())
}
