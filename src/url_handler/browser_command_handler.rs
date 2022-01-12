use std::env::{consts, var};
use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

type UrlString<'a> = Option<&'a str>;
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
        Ok(())
    }
}

pub fn open_browser_to_url_string(url_str: &str, bin_to_use: Option<&str>) -> Result<()> {
    let args = match bin_to_use {
        Some(bin_name_str) => unimplemented!(),
        None => get_args_list_for_opening_page_to_url(url_str),
    };
    run_open_browser_command_with_args(args)?;

    Ok(())
}

pub fn open_browser_to_blank_page(bin_to_use: Option<&str>) -> Result<()> {
    let args = match bin_to_use {
        Some(bin_name_str) => unimplemented!(),
        None => get_args_list_for_blank_page(),
    };
    run_open_browser_command_with_args(args)?;

    Ok(())
}

fn get_args_list_for_opening_page_to_url(url_str: &str) -> Vec<String> {
    let browser_exec_path_str = get_path_of_browser_executable()
        .into_os_string()
        .into_string()
        .unwrap();

    vec!["-a".to_string(), browser_exec_path_str, url_str.to_string()]
}

fn get_args_list_for_blank_page() -> Vec<PathBuf> {
    let browser_exec_path = get_path_of_browser_executable();
    vec![browser_exec_path]
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

fn get_path_of_browser_executable() -> PathBuf {
    let browser_path_key = "BROWSER_BIN_PATH";
    let browser_path_str_from_env = var(browser_path_key).expect(
        r#"Environment variable "BROWSER_BIN_PATH" not set.
            Should point to the executable browser binary"#,
    );
    PathBuf::from(browser_path_str_from_env)
}

#[cfg(test)]
mod test_browser_command {
    use super::*;

    #[test]
    fn test_args_list_from_url_ok() {
        pre_test_setup();
        let url = "https://google.com";
        let browser_bin_path_string = get_path_of_browser_executable()
            .into_os_string()
            .into_string()
            .unwrap();

        let args_list = get_args_list_for_opening_page_to_url(url);

        let args_vec_to_compare = vec!["-a".to_string(), browser_bin_path_string, url.to_string()];

        assert_eq!(args_vec_to_compare, args_list);
    }

    #[test]
    fn test_blank_page_arg_list_ok() {
        pre_test_setup();
        let arg_list = get_args_list_for_blank_page();
        let browser_bin_path_in_vec = vec![get_path_of_browser_executable()];
        assert_eq!(browser_bin_path_in_vec, arg_list);
    }

    #[test]
    fn test_path_from_env_ok() {
        pre_test_setup();

        let old_env_var = get_browser_bin_path_env_value();

        // what are the odds of someone running this and NOT having rustup?
        let temp_var = "~/.rustup/settings.toml";
        set_browser_bin_path_env_value(temp_var);

        let bin_path = get_path_of_browser_executable();
        let file_exists = !bin_path.exists();

        assert!(file_exists);

        set_browser_bin_path_env_value(&old_env_var);
    }

    #[test]
    fn test_bad_env_path_() {
        pre_test_setup();

        let old_env_var = get_browser_bin_path_env_value();

        let invalid_var_value = "NONEXISTENT_PATH";
        set_browser_bin_path_env_value(invalid_var_value);

        let nonexistent_path = get_path_of_browser_executable();
        let file_doesnt_exist = !nonexistent_path.exists();

        assert!(file_doesnt_exist);

        set_browser_bin_path_env_value(&old_env_var);
    }

    fn pre_test_setup() {
        let browser_path = get_browser_path_key();
        set_browser_bin_path_env_value(browser_path);
    }

    fn get_browser_path_key<'a>() -> &'a str {
        "BROWSER_BIN_PATH"
    }

    fn set_browser_bin_path_env_value(value: &str) {
        let env_var_key = get_browser_path_key();
        std::env::set_var(env_var_key, value);
    }

    fn get_browser_bin_path_env_value() -> String {
        let env_var_key = get_browser_path_key();
        std::env::var(env_var_key).expect("Can't read env var")
    }
}
