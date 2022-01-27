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

pub fn turn_to_search_url(search_term: &str) -> String {
    let fmt_search_term = search_term.replace(" ", "+");
    format!("https://www.google.com/search?q={fmt_search_term}")
}

fn add_http_prefix_to_url(url_str: &str) -> String {
    ["http://", url_str].join("")
}

#[cfg(test)]
mod test_url_input_type_unit {
    use super::*;

    #[test]
    fn test_format_url_with_prefix_ok() {
        let url_with_prefix = "https://google";
        let formatted_url = validate_and_fix_url_string(url_with_prefix);

        assert_eq!(url_with_prefix, formatted_url)
    }

    #[test]
    fn test_format_url_str_no_http_prefix_ok() {
        let url_without_prefix = "google";
        let formatted_url = validate_and_fix_url_string(url_without_prefix);

        let edited_url = ["http://", url_without_prefix].join("");

        assert_eq!(formatted_url, edited_url)
    }
}
