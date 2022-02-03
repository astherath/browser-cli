use super::url_macro_handler;

pub enum UrlInputType {
    Regular(String),
    Blank,
    Special(url_macro_handler::SpecialUrl),
}

pub fn validate_and_fix_url_string(url_str: &str) -> String {
    // check for special yt shorthand first
    if url_str.starts_with("yt +") {
        return turn_to_yt_search_url(url_str);
    }
    // check if searchable string
    if url_str.contains(" ") || !url_str.contains(".") {
        return turn_to_search_url(url_str);
    }

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

pub fn turn_to_search_url(search_term: &str) -> String {
    let fmt_search_term = search_term.replace(" ", "+");
    format!("https://www.google.com/search?q={fmt_search_term}")
}

fn turn_to_yt_search_url(search_term: &str) -> String {
    let fmt_search_term = search_term.chars().skip(4).collect::<String>().replace(" ", "+");
    format!("https://www.youtube.com/results?search_query={fmt_search_term}")
}


fn add_http_prefix_to_url(url_str: &str) -> String {
    ["http://", url_str].join("")
}
