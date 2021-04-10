use clap::{App, Arg};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn get_matches_and_run_command() -> Result<()> {
    let matches = App::new("Browser Utility")
        .bin_name("browser")
        .version("1.0")
        .author("Felipe A. <farceriv@gmail.com>")
        .about("Opens up a browser tab to the given URL + some more niceties")
        .arg(
            Arg::with_name("url")
                .multiple(false)
                .value_name("url")
                .help("URL value to open the browser instance with")
                .takes_value(true),
        )
        .get_matches();

    let raw_url_match = matches.value_of("url");
    let url = get_url_from_raw_match(raw_url_match);
    open_browser_to_url(url)?;

    Ok(())
}

enum SpecialUrl {
    Github,
    Logs,
    Gmail,
    DevServer,
    Default,
}

impl SpecialUrl {
    fn get_all_possible_value_strs() -> Vec<String> {
        vec!["github", "mail", "gmail", "logs", "log", "dev"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    fn try_to_match_from_str(url_str: &str) -> Option<Self> {
        let matcher = SkimMatcherV2::default();
        let possible_enum_strings = Self::get_all_possible_value_strs();

        for string_to_compare in &possible_enum_strings {
            if matcher.fuzzy_match(string_to_compare, url_str).is_some() {
                return Some(Self::from_str(string_to_compare));
            }
        }
        None
    }

    fn from_str(str_val: &str) -> Self {
        match str_val {
            "github" => Self::Github,
            "mail" | "gmail" => Self::Gmail,
            "logs" | "log" => Self::Logs,
            "dev" => Self::DevServer,
            _ => Self::Default,
        }
    }

    fn to_url_string(self) -> String {
        match self {
            Self::Github => "https://github.com/astherath",
            Self::Gmail => "https://gmail.com",
            Self::Logs => "https://dashboard.heroku.com/apps/sparkdev-underline/logs",
            Self::DevServer => "localhost:8000/docs",
            Self::Default => "https://google.com",
        }
        .to_string()
    }
}

enum UrlInputType {
    Regular(String),
    Blank,
    Special(SpecialUrl),
}

fn get_url_from_raw_match(raw_cli_match: Option<&str>) -> UrlInputType {
    if let Some(url_str) = raw_cli_match {
        match SpecialUrl::try_to_match_from_str(url_str) {
            Some(special_url) => UrlInputType::Special(special_url),
            None => UrlInputType::Regular(url_str.to_string()),
        }
    } else {
        UrlInputType::Blank
    }
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
    let fixed_url_string = validate_and_fix_url_string(url_str);

    let args = &[
        "-a",
        browser_exec_path_str.to_str().unwrap(),
        &fixed_url_string,
    ];

    run_open_browser_command_with_args(args)?;

    Ok(())
}

fn validate_and_fix_url_string(url_str: &str) -> String {
    let min_http_prefix_size = 7;
    let url_too_small_to_have_http_prefix = url_str.len() <= min_http_prefix_size;

    if url_too_small_to_have_http_prefix {
        add_http_prefix_to_url(url_str)
    } else {
        let prefix_substring: String = url_str.chars().take(min_http_prefix_size).collect();
        match prefix_substring.as_str() {
            "http://" | "https:/" => String::from(url_str),
            _ => add_http_prefix_to_url(url_str),
        }
    }
}

fn add_http_prefix_to_url(url_str: &str) -> String {
    ["http://", url_str].join("")
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
    let browser_path_key = "BROWSER_BIN_PATH";
    let browser_path_str_from_env = std::env::var(browser_path_key).expect(
        r#"Environment variable "BROWSER_BIN_PATH" not set.
            Should point to the executable browser binary"#,
    );
    PathBuf::from(browser_path_str_from_env)
}
