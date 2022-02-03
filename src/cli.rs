use crate::url_handler;
use clap::{App, Arg, SubCommand};
use std::io::Result;
use url_handler::url_macro_handler::SpecialUrl;

pub fn get_matches_and_run_command() -> Result<()> {
    let matches = App::new("Browser Utility")
        .version("1.0")
        .author("astherath <me@felipearce.dev>")
        .about("Opens up a browser tab to the given URL + some more niceties")
        .arg(
            Arg::with_name("url")
                .multiple(false)
                .default_value("new")
                .value_name("url")
                .help("URL string, search term(s) or shortcut to open the browser instance to")
                .long_help(
                    "Can accept a URL string, a quoted sentence to google, a youtube query signified by \"yt +\", or one of the shortcuts",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bin-name")
                .multiple(false)
                .default_value("qute")
                .value_name("bin-name")
                .help("sets the browser bin to use")
                .possible_values(&["qute", "ffox", "brave"])
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about("Lists the possible shorthand URL bookmarks and where they take you"),
        )
        .get_matches();

    if matches.subcommand_matches("view").is_some() {
        let opts = SpecialUrl::get_shorthand_url_pairs();
        println!("Shorthand | url");
        println!("{}", "-".repeat(50));
        opts.iter()
            .for_each(|x| println!("{:<8} | {:<30}", x.0, x.1));
    } else {
        let raw_url_match = matches.value_of("url");
        let bin_to_use = matches.value_of("bin-name");
        let url = url_handler::get_url_from_raw_match(raw_url_match);
        url_handler::open_browser_to_url(url, bin_to_use)?;
    }

    Ok(())
}
