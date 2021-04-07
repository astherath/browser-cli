use clap::{App, Arg};
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_matches_and_run_command() -> Result<()> {
    let special_url_possible_values = SpecialUrl::get_str_array_of_possible_values().into_iter();
    let matches = App::new("Browser Utility")
        .version("1.0")
        .author("Felipe A. <farceriv@gmail.com>")
        .about("Opens up a browser tab to the given URL + some more niceties")
        .arg(
            Arg::with_name("special")
                .value_name("special_url")
                .help("Provides shorthand for a couple of popular/common URLs")
                .takes_value(true)
                .multiple(false)
                .max_values(1)
                .min_values(1)
                .possible_values(special_url_possible_values)
                .conflicts_with("url"),
        )
        .arg(
            Arg::with_name("url")
                .value_name("url")
                .help("URL value to open to")
                .takes_value(true)
                .conflicts_with("special_url"),
        )
        .get_matches();

    let url = match matches.value_of("url") {
        Some(url) => UrlInputType::Regular(url.to_string()),
        None => UrlInputType::Blank,
    };

    open_browser_to_url(url)?;

    Ok(())
}

enum SpecialUrl {
    Github,
    Gmail,
}

impl SpecialUrl {
    fn get_str_array_of_possible_values() -> Vec<String> {
        vec![Self::Github, Self::Gmail]
            .into_iter()
            .map(|x| x.to_url_string())
            .collect()
    }

    fn to_url_string(self) -> String {
        match self {
            Self::Github => "https://github.com/astherath",
            Self::Gmail => "https://gmail.com",
        }
        .to_string()
    }
}

enum UrlInputType {
    Regular(String),
    Blank,
    Special(SpecialUrl),
}

fn open_browser_to_url(url: UrlInputType) -> Result<()> {
    match url {
        UrlInputType::Regular(url_string) => open_browser_to_url_string(&url_string)?,
        UrlInputType::Special(special_url) => {
            let url_string = special_url.to_url_string();
            open_browser_to_url_string(&url_string)?
        }
        UrlInputType::Blank => open_browser_to_blank_page()?,
    }
    Ok(())
}

fn open_browser_to_url_string(url_str: &str) -> Result<()> {
    let browser_exec_path_str = get_path_of_browser_executable();
    let args = &["-a", browser_exec_path_str.to_str().unwrap(), url_str];
    run_open_browser_command_with_args(args)?;

    Ok(())
}

fn open_browser_to_blank_page() -> Result<()> {
    let browser_exec_path = get_path_of_browser_executable();
    let args = &[browser_exec_path];
    run_open_browser_command_with_args(args)?;

    Ok(())
}

fn run_open_browser_command_with_args<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("open")
        .args(args)
        .spawn()
        .expect("Failed to execute");
    Ok(())
}

fn get_path_of_browser_executable() -> PathBuf {
    PathBuf::from("/Applications/Brave Browser.app/")
}
