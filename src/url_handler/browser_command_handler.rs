use std::env::{consts, var};
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

type UrlString<'a> = &'a str;
type BinName<'a> = Option<&'a str>;

pub struct ArgUtil<'a> {
    url: UrlString<'a>,
    bin: BinName<'a>,
}

impl<'a> ArgUtil<'a> {
    pub fn new(url: UrlString<'a>, bin: BinName<'a>) -> Self {
        Self { url, bin }
    }

    pub fn open_blank_page(bin_to_use: Option<&'a str>) -> Result<()> {
        let empty_str = "";
        let util = Self::new(empty_str, bin_to_use);
        let args = util.get_args_list_for_blank_page();
        run_open_browser_command_with_args(args)?;
        Ok(())
    }

    pub fn open_browser_to_url(&self) -> Result<()> {
        let args = self.get_args_list_for_opening_page_to_url();
        run_open_browser_command_with_args(args)?;

        Ok(())
    }

    fn get_args_list_for_opening_page_to_url(&self) -> Vec<String> {
        let browser_exec_path_str = self
            .get_path_of_browser_executable()
            .into_os_string()
            .into_string()
            .unwrap();

        vec![
            "-a".to_string(),
            browser_exec_path_str,
            self.url.to_string(),
        ]
    }

    fn get_args_list_for_blank_page(&self) -> Vec<PathBuf> {
        let browser_exec_path = self.get_path_of_browser_executable();
        vec![browser_exec_path]
    }

    fn get_path_of_browser_executable(&self) -> PathBuf {
        match self.bin {
            None => get_browser_executable_path_from_os(),
            Some(bin_name) => get_path_of_browser_bin_by_name(bin_name),
        }
    }
}

fn get_path_of_browser_bin_by_name(bin_name: &str) -> PathBuf {
    PathBuf::from(match bin_name {
        "qute" => "/Applications/qutebrowser.app",
        "ffox" => "/Applications/Firefox Developer Edition.app",
        "brave" => "/Applications/Brave Browser.app",
        _ => panic!("should not be reachable!"),
    })
}

fn get_browser_executable_path_from_os() -> PathBuf {
    let browser_path_key = "BROWSER_BIN_PATH";
    let browser_path_str_from_env = var(browser_path_key).expect(
        r#"Environment variable "BROWSER_BIN_PATH" not set.
            Should point to the executable browser binary"#,
    );
    PathBuf::from(browser_path_str_from_env)
}

fn run_open_browser_command_with_args<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let open_command = match consts::OS {
        "linux" => "xdg-open",
        "macos" => "open",
        "_" | &_ => "open",
    };

    Command::new(open_command)
        .args(args)
        .spawn()
        .expect("Failed to execute");
    Ok(())
}
