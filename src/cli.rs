use clap::{App, Arg};
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_matches_and_run_command() -> Result<()> {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Felipe A. <farceriv@gmail.com>")
        .about("Opens up a browser tab to the given URL + some more niceties")
        .arg(
            Arg::with_name("url")
                .value_name("url")
                .help("URL value to open to")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("url") {
        Some(url) => open_browser_to_url(url)?,
        None => open_blank_browser_page(url)?,
    }
    Ok(())
}

enum SavedUrlStrings {
    Github,
    Gmail,
}

enum UrlInputTypes {
    Regular(String),
    Blank,
    Special(SavedUrlStrings),
}

fn open_browser_to_url(url: &str) -> Result<()> {
    run_open_browser_command(url)?;
    Ok(())
}

fn open_blank_browser_page() -> Result<()> {
    run_open_browser_command(url)?;
    Ok(())
}

fn run_open_browser_command(url: &str) -> Result<()> {
    let browser_exec_path = get_path_of_browser_executable();

    Command::new("open")
        .arg("-a")
        .arg(browser_exec_path)
        .arg(url)
        .spawn()
        .expect("Failed to execute");

    Ok(())
}

fn get_path_of_browser_executable() -> PathBuf {
    PathBuf::from("/Applications/Brave Browser.app/")
}
