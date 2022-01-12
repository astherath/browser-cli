use crate::url_handler;
use clap::{App, Arg};
use std::io::Result;

pub fn get_matches_and_run_command() -> Result<()> {
    let matches = App::new("Browser Utility")
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
        .arg(
            Arg::with_name("bin-name")
                .multiple(false)
                .default_value("qute")
                .value_name("bin_name")
                .help("sets the browser bin to use")
                .possible_values(&["qute", "ffox", "brave"])
                .takes_value(true),
        )
        .get_matches();

    let raw_url_match = matches.value_of("url");
    let bin_to_use = matches.value_of("bin_name");
    let url = url_handler::get_url_from_raw_match(raw_url_match);
    url_handler::open_browser_to_url(url, bin_to_use)?;

    Ok(())
}
