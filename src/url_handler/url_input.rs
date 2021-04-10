use super::url_macro_handler;

pub enum UrlInputType {
    Regular(String),
    Blank,
    Special(url_macro_handler::SpecialUrl),
}

pub fn validate_and_fix_url_string(url_str: &str) -> String {
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
