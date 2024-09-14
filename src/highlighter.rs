use crate::config::*;
use crate::error::Error;
use crate::highlighters::date_dash::DateDashHighlighter;
use crate::highlighters::date_time::TimeHighlighter;
use crate::highlighters::ip_v4::IpV4Highlighter;
use crate::highlighters::ip_v6::IpV6Highlighter;
use crate::highlighters::json::JsonHighlighter;
use crate::highlighters::key_value::KeyValueHighlighter;
use crate::highlighters::keyword::KeywordHighlighter;
use crate::highlighters::number::NumberHighlighter;
use crate::highlighters::pointer::PointerHighlighter;
use crate::highlighters::quote::QuoteHighlighter;
use crate::highlighters::regex::RegexpHighlighter;
use crate::highlighters::unix_path::UnixPathHighlighter;
use crate::highlighters::unix_process::UnixProcessHighlighter;
use crate::highlighters::url::UrlHighlighter;
use crate::highlighters::uuid::UuidHighlighter;
use crate::normalizer::normalize_keyword_configs;
use crate::split_and_apply::apply_only_to_unhighlighted;
use crate::Style;
use std::sync::Arc;

pub trait Highlight: Sync + Send {
    fn apply(&self, input: &str) -> String;
}

pub struct Highlighter {
    highlighters: Vec<Arc<dyn Highlight>>,
}

impl Highlighter {
    const fn new() -> Self {
        Highlighter {
            highlighters: Vec::new(),
        }
    }

    pub fn builder() -> HighlightBuilder {
        HighlightBuilder {
            highlighters: Vec::new(),
            regex_errors: Vec::new(),
        }
    }

    fn with_highlighters(mut self, highlighters: Vec<Arc<dyn Highlight>>) -> Self {
        self.highlighters = highlighters;

        self
    }

    pub fn apply(self, text: String) -> String {
        self.highlighters
            .into_iter()
            .fold(text, |acc, highlighter| apply_only_to_unhighlighted(&acc, highlighter))
    }
}

impl Default for Highlighter {
    /// Compiles a default highlighter with reasonable defaults.
    ///
    /// Since we are compiling regexes under the hood, this is an expensive operation and should be done once and then
    /// be reused.
    fn default() -> Self {
        Highlighter::builder()
            .with_json_highlighter(JsonConfig::default())
            .with_date_time_highlighters(DateTimeConfig::default())
            .with_url_highlighter(UrlConfig::default())
            .with_ip_v4_highlighter(IpV4Config::default())
            .with_ip_v6_highlighter(IpV6Config::default())
            .with_uuid_highlighter(UuidConfig::default())
            .with_pointer_highlighter(PointerConfig::default())
            .with_unix_path_highlighter(UnixPathConfig::default())
            .with_unix_process_highlighter(UnixProcessConfig::default())
            .with_key_value_highlighter(KeyValueConfig::default())
            .with_number_highlighter(NumberConfig::default())
            .with_quote_highlighter(QuoteConfig::default())
            .build()
            .expect("Default constructor should never fail")
    }
}

pub struct HighlightBuilder {
    highlighters: Vec<Arc<dyn Highlight>>,
    regex_errors: Vec<regex::Error>,
}

impl HighlightBuilder {
    pub fn with_number_highlighter(self, config: NumberConfig) -> Self {
        self.try_add_highlighter(NumberHighlighter::new(config))
    }

    pub fn with_uuid_highlighter(self, config: UuidConfig) -> Self {
        self.try_add_highlighter(UuidHighlighter::new(config))
    }

    pub fn with_unix_path_highlighter(self, config: UnixPathConfig) -> Self {
        self.try_add_highlighter(UnixPathHighlighter::new(config))
    }

    pub fn with_unix_process_highlighter(self, config: UnixProcessConfig) -> Self {
        self.try_add_highlighter(UnixProcessHighlighter::new(config))
    }

    pub fn with_key_value_highlighter(self, config: KeyValueConfig) -> Self {
        self.try_add_highlighter(KeyValueHighlighter::new(config))
    }

    pub fn with_date_time_highlighters(self, config: DateTimeConfig) -> Self {
        self.try_add_highlighter(TimeHighlighter::new(config))
            .try_add_highlighter(DateDashHighlighter::new(config))
    }

    pub fn with_ip_v4_highlighter(self, config: IpV4Config) -> Self {
        self.try_add_highlighter(IpV4Highlighter::new(config))
    }

    pub fn with_ip_v6_highlighter(self, config: IpV6Config) -> Self {
        self.try_add_highlighter(IpV6Highlighter::new(config))
    }

    pub fn with_url_highlighter(self, config: UrlConfig) -> Self {
        self.try_add_highlighter(UrlHighlighter::new(config))
    }

    pub fn with_pointer_highlighter(self, config: PointerConfig) -> Self {
        self.try_add_highlighter(PointerHighlighter::new(config))
    }

    pub fn with_regex_highlighter(self, regexp: String, style: Style) -> Self {
        self.try_add_highlighter(RegexpHighlighter::new(regexp, style))
    }

    pub fn with_quote_highlighter(self, config: QuoteConfig) -> Self {
        self.try_add_highlighter(Ok(QuoteHighlighter::new(config)))
    }

    pub fn with_json_highlighter(self, config: JsonConfig) -> Self {
        self.try_add_highlighter(Ok(JsonHighlighter::new(config)))
    }

    pub fn with_keyword_highlighter(mut self, keyword_configs: Vec<KeywordConfig>) -> Self {
        let normalized_keyword_configs = normalize_keyword_configs(keyword_configs);

        for keyword_config in normalized_keyword_configs {
            let highlighter = KeywordHighlighter::new(keyword_config);

            match highlighter {
                Ok(h) => self.highlighters.push(Arc::new(h)),
                Err(e) => self.regex_errors.push(e),
            }
        }

        self
    }

    fn try_add_highlighter<T: Highlight + 'static>(mut self, highlighter: Result<T, regex::Error>) -> Self {
        match highlighter {
            Ok(h) => self.highlighters.push(Arc::new(h)),
            Err(e) => self.regex_errors.push(e),
        }

        self
    }

    pub fn build(self) -> Result<Highlighter, Error> {
        match self.regex_errors.is_empty() {
            true => Ok(Highlighter::new().with_highlighters(self.highlighters)),
            false => Err(Error::RegexErrors(self.regex_errors)),
        }
    }
}
