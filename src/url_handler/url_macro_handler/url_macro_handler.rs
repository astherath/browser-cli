use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

pub enum SpecialUrl {
    Github,
    Logs,
    Gmail,
    DevServer,
    Ng,
    Yt,
    Default,
}

impl SpecialUrl {
    pub fn to_url_string(self) -> String {
        match self {
            Self::Github => "https://github.com/astherath",
            Self::Gmail => "https://gmail.com",
            Self::Logs => "https://dashboard.heroku.com/apps/sparkdev-underline/logs",
            Self::DevServer => "localhost:8000/docs",
            Self::Ng => "localhost:4200",
            Self::Yt => "https://youtube.com",
            Self::Default => "https://google.com",
        }
        .to_string()
    }

    pub fn try_to_match_from_str(url_str: &str) -> Option<Self> {
        let matcher = SkimMatcherV2::default();
        let possible_enum_strings = Self::get_all_possible_value_strs();

        for string_to_compare in &possible_enum_strings {
            if matcher.fuzzy_match(string_to_compare, url_str).is_some() {
                return Some(Self::from_str(string_to_compare));
            }
        }
        None
    }

    fn from_str(str_val: &str) -> Self {
        match str_val {
            "github" => Self::Github,
            "mail" | "gmail" => Self::Gmail,
            "logs" | "log" => Self::Logs,
            "dev" => Self::DevServer,
            "ng" => Self::Ng,
            "yt" => Self::Yt,
            "new" => Self::Default,
            _ => Self::Default,
        }
    }

    fn get_all_possible_value_strs() -> Vec<String> {
        vec!["github", "mail", "gmail", "logs", "log", "dev", "ng", "yt", "new"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
}
