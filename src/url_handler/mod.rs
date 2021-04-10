mod browser_command_handler;
mod url_input;
mod url_macro_handler;

use std::io::Result;

use url_input::UrlInputType;
use url_macro_handler::SpecialUrl;

pub fn get_url_from_raw_match(raw_cli_match: Option<&str>) -> UrlInputType {
    if let Some(url_str) = raw_cli_match {
        match SpecialUrl::try_to_match_from_str(url_str) {
            Some(special_url) => UrlInputType::Special(special_url),
            None => UrlInputType::Regular(url_str.to_string()),
        }
    } else {
        UrlInputType::Blank
    }
}

pub fn open_browser_to_url(url: UrlInputType) -> Result<()> {
    match url {
        UrlInputType::Regular(raw_url_string) => {
            let url_string = url_input::validate_and_fix_url_string(&raw_url_string);
            browser_command_handler::open_browser_to_url_string(&url_string)?
        }
        UrlInputType::Special(special_url) => {
            let url_string = url_input::validate_and_fix_url_string(&special_url.to_url_string());
            browser_command_handler::open_browser_to_url_string(&url_string)?
        }
        UrlInputType::Blank => browser_command_handler::open_browser_to_blank_page()?,
    }
    Ok(())
}
