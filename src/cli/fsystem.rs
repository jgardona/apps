use askama::Template;

use super::error::Result;

use std::{
    env,
    fs::{self},
    ops::Add,
};

const HOME: &str = "HOME";
const APPLICATION_FOLDER: &str = "/.local/share/applications/";
const FILE_EXTENSION: &str = ".desktop";

#[derive(Template)]
#[template(path = "template.txt")]
pub struct DesktopTemplate<'a> {
    pub appname: &'a str,
    appicon: &'a str,
    executable: &'a str,
    comment: &'a str,
}

impl<'a> DesktopTemplate<'a> {
    pub fn new(appname: &'a str, appicon: &'a str, executable: &'a str, comment: &'a str) -> Self {
        Self {
            appname,
            appicon,
            executable,
            comment,
        }
    }
}

pub fn get_dir_iter() -> Result<Vec<String>> {
    let path = get_app_fs()?;
    let dir = fs::read_dir(path)?;
    let buffer: Vec<String> = dir
        .filter_map(|s| {
            let path = s.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.into())
            } else {
                None
            }
        })
        .collect();
    Ok(buffer)
}

fn get_app_fs() -> Result<String> {
    let home = env::var(HOME)?;
    let path = home.add(APPLICATION_FOLDER);
    Ok(path)
}

pub fn write_file(name: &str, contents: &str) -> Result<()> {
    let name = name.to_owned().add(FILE_EXTENSION);
    let path = get_app_fs()?;
    let path = path.add(&name);
    fs::write(path, contents.as_bytes())?;
    Ok(())
}

pub fn remove_file(name: &str) -> Result<()> {
    let name = name.to_owned().add(FILE_EXTENSION);
    let path = get_app_fs()?.add(&name);
    fs::remove_file(path)?;
    Ok(())
}

#[cfg(test)]
mod fsystem_tests {
    use super::get_dir_iter;
    use crate::cli::error::Result;

    #[test]
    fn test_fystem_path() -> Result<()> {
        let _iter = get_dir_iter()?;
        Ok(())
    }
}
