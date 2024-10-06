use crate::{Color, Style};

pub struct NumberConfig {
    pub style: Style,
}

impl Default for NumberConfig {
    fn default() -> Self {
        NumberConfig {
            style: Style::new().fg(Color::Cyan),
        }
    }
}

pub struct UuidConfig {
    pub number: Style,
    pub letter: Style,
    pub dash: Style,
}

impl Default for UuidConfig {
    fn default() -> Self {
        UuidConfig {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            dash: Style::new().fg(Color::Red),
        }
    }
}

pub struct KeyValueConfig {
    pub key: Style,
    pub separator: Style,
}

impl Default for KeyValueConfig {
    fn default() -> Self {
        KeyValueConfig {
            key: Style::new().faint(),
            separator: Style::new().fg(Color::White),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DateTimeConfig {
    pub date: Style,
    pub time: Style,
    pub zone: Style,
    pub separator: Style,
}

impl Default for DateTimeConfig {
    fn default() -> Self {
        DateTimeConfig {
            date: Style::new().fg(Color::Magenta),
            time: Style::new().fg(Color::Blue),
            zone: Style::new().fg(Color::Red),
            separator: Style::new().faint(),
        }
    }
}

pub struct IpV4Config {
    pub number: Style,
    pub separator: Style,
}

impl Default for IpV4Config {
    fn default() -> Self {
        IpV4Config {
            number: Style::new().fg(Color::Blue).italic(),
            separator: Style::new().fg(Color::Red),
        }
    }
}

pub struct IpV6Config {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
}

impl Default for IpV6Config {
    fn default() -> Self {
        IpV6Config {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().fg(Color::Red),
        }
    }
}

pub struct UrlConfig {
    pub http: Style,
    pub https: Style,
    pub host: Style,
    pub path: Style,
    pub query_params_key: Style,
    pub query_params_value: Style,
    pub symbols: Style,
}

impl Default for UrlConfig {
    fn default() -> Self {
        UrlConfig {
            http: Style::new().fg(Color::Red).faint(),
            https: Style::new().fg(Color::Green).faint(),
            host: Style::new().fg(Color::Blue).faint(),
            path: Style::new().fg(Color::Blue),
            query_params_key: Style::new().fg(Color::Magenta),
            query_params_value: Style::new().fg(Color::Cyan),
            symbols: Style::new().fg(Color::Red),
        }
    }
}

pub struct UnixPathConfig {
    pub segment: Style,
    pub separator: Style,
}

impl Default for UnixPathConfig {
    fn default() -> Self {
        UnixPathConfig {
            segment: Style::new().fg(Color::Green),
            separator: Style::new().fg(Color::Yellow),
        }
    }
}

pub struct PointerConfig {
    pub number: Style,
    pub letter: Style,
    pub separator: Style,
    pub separator_token: char,
    pub x: Style,
}

impl Default for PointerConfig {
    fn default() -> Self {
        PointerConfig {
            number: Style::new().fg(Color::Blue).italic(),
            letter: Style::new().fg(Color::Magenta).italic(),
            separator: Style::new().faint(),
            separator_token: '•',
            x: Style::new().fg(Color::Red),
        }
    }
}

pub struct UnixProcessConfig {
    pub name: Style,
    pub id: Style,
    pub bracket: Style,
}

impl Default for UnixProcessConfig {
    fn default() -> Self {
        UnixProcessConfig {
            name: Style::new().fg(Color::Green),
            id: Style::new().fg(Color::Yellow),
            bracket: Style::new().fg(Color::Red),
        }
    }
}

pub struct JsonConfig {
    pub key: Style,
    pub quote_token: Style,
    pub curly_bracket: Style,
    pub square_bracket: Style,
    pub comma: Style,
    pub colon: Style,
}

impl Default for JsonConfig {
    fn default() -> Self {
        JsonConfig {
            key: Style::new().faint(),
            quote_token: Style::new().fg(Color::Default),
            curly_bracket: Style::new().fg(Color::Blue),
            square_bracket: Style::new().fg(Color::Red),
            comma: Style::new().fg(Color::Red),
            colon: Style::new().fg(Color::Red),
        }
    }
}

pub struct QuotesConfig {
    pub quotes_token: char,
    pub style: Style,
}

impl Default for QuotesConfig {
    fn default() -> Self {
        QuotesConfig {
            quotes_token: '"',
            style: Style::new().fg(Color::Yellow),
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct KeywordConfig {
    pub words: Vec<String>,
    pub style: Style,
}
