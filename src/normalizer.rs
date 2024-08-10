use crate::{KeywordConfig, Style};
use std::collections::HashMap;
use std::hash::Hash;

fn normalize_keyword_configs(configs: Vec<KeywordConfig>) -> Vec<KeywordConfig> {
    let mut grouped_configs: HashMap<Style, Vec<String>> = HashMap::new();

    for config in configs {
        grouped_configs
            .entry(config.style)
            .or_insert_with(Vec::new)
            .extend(config.words);
    }

    grouped_configs
        .into_iter()
        .map(|(style, words)| KeywordConfig { words, style })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color::*;

    #[test]
    fn test_normalize_keyword_configs() {
        let configs = vec![
            KeywordConfig {
                words: vec!["hello".to_string(), "world".to_string()],
                style: Style {
                    fg: Some(Red),
                    bg: None,
                    bold: true,
                    faint: false,
                    italic: false,
                    underline: false,
                },
            },
            KeywordConfig {
                words: vec!["foo".to_string(), "bar".to_string()],
                style: Style {
                    fg: Some(Red),
                    bg: None,
                    bold: true,
                    faint: false,
                    italic: false,
                    underline: false,
                },
            },
            KeywordConfig {
                words: vec!["baz".to_string()],
                style: Style {
                    fg: Some(Green),
                    bg: None,
                    bold: false,
                    faint: false,
                    italic: false,
                    underline: true,
                },
            },
        ];

        let expected = vec![
            KeywordConfig {
                words: vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "foo".to_string(),
                    "bar".to_string(),
                ],
                style: Style {
                    fg: Some(Red),
                    bg: None,
                    bold: true,
                    faint: false,
                    italic: false,
                    underline: false,
                },
            },
            KeywordConfig {
                words: vec!["baz".to_string()],
                style: Style {
                    fg: Some(Green),
                    bg: None,
                    bold: false,
                    faint: false,
                    italic: false,
                    underline: true,
                },
            },
        ];

        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }

    #[test]
    fn test_normalize_keyword_configs_empty() {
        let configs: Vec<KeywordConfig> = vec![];
        let expected: Vec<KeywordConfig> = vec![];
        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }

    #[test]
    fn test_normalize_keyword_configs_no_duplicates() {
        let configs = vec![KeywordConfig {
            words: vec!["unique".to_string()],
            style: Style {
                fg: Some(Blue),
                bg: None,
                bold: false,
                faint: false,
                italic: true,
                underline: false,
            },
        }];

        let expected = vec![KeywordConfig {
            words: vec!["unique".to_string()],
            style: Style {
                fg: Some(Blue),
                bg: None,
                bold: false,
                faint: false,
                italic: true,
                underline: false,
            },
        }];

        let normalized_configs = normalize_keyword_configs(configs);
        assert_eq!(normalized_configs, expected);
    }
}
