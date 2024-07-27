use nu_ansi_term::Style;
use once_cell::sync::Lazy;
use regex::Regex;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)       # Enable comments and whitespace insensitivity
            \b       # Word boundary, ensures we are at the start of a number
            \d+      # Matches one or more digits
            (\.      # Start a group to match a decimal part
            \d+      # Matches one or more digits after the dot
            )?       # The decimal part is optional
            \b       # Word boundary, ensures we are at the end of a number
            ",
    )
    .expect("Invalid regex pattern")
});

pub struct NumberHighlighter {
    style: Style,
}

impl NumberHighlighter {
    pub const fn new(style: Style) -> Self {
        Self { style }
    }
}

// impl Highlight for NumberHighlighter {
//     fn apply(&self, input: &str) -> String {
//         NUMBER_REGEX
//             .replace_all(input, |caps: &Captures<'_>| {
//                 format!("{}", self.style.paint(&caps[0]))
//             })
//             .to_string()
//     }
// }
