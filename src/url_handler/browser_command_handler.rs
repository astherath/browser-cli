use std::ffi::OsStr;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn open_browser_to_url_string(url_str: &str) -> Result<()> {
    let browser_exec_path_str = get_path_of_browser_executable();
    let args = &["-a", browser_exec_path_str.to_str().unwrap(), &url_str];
    run_open_browser_command_with_args(args)?;

    Ok(())
}

pub fn open_browser_to_blank_page() -> Result<()> {
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

#[cfg(test)]
mod test_browser_command {
    use super::*;
    mod test_browser_path {
        use super::*;

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

        #[test]
        fn test_path_from_env_ok() {
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
            let old_env_var = get_browser_bin_path_env_value();

            let invalid_var_value = "NONEXISTENT_PATH";
            set_browser_bin_path_env_value(invalid_var_value);

            let nonexistent_path = get_path_of_browser_executable();
            let file_doesnt_exist = !nonexistent_path.exists();

            assert!(file_doesnt_exist);

            set_browser_bin_path_env_value(&old_env_var);
        }
    }
}
