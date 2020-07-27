use anyhow::Result;
use std::io::prelude::*;
use std::fs::File;

use sd::replacer;

fn assert_file(path: &std::path::Path, content: &str) {
    assert_eq!(content, std::fs::read_to_string(path).unwrap());
}

#[test]
fn in_place() -> Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    file.write(b"abc123def")?;
    let path = file.into_temp_path();
    let pathbuf = vec![path.to_str().unwrap().into()];

    replacer(pathbuf, "abc\\d+".into(), "".into()).unwrap();    

    assert_file(&path.to_path_buf(), "def");

    Ok(())
}

#[test]
fn in_place_check() -> Result<()> {
    let mut file = File::create("./test.config").unwrap();
    file.write(b"abc123def")?;
    let pathbuf = vec!["./test.config".into()];

    replacer(pathbuf, "abc\\d+".into(), "".into()).unwrap();    

    Ok(())
}


#[test]
fn in_place_with_empty_result_file() -> Result<()> {
    let mut file = tempfile::NamedTempFile::new()?;
    file.write(b"a7c")?;
    let path = file.into_temp_path();
    let pathbuf = vec![path.to_str().unwrap().into()];


    replacer(pathbuf, "a\\dc".into(), "".into()).unwrap();
    assert_file(&path.to_path_buf(), "");

    Ok(())
}
