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
