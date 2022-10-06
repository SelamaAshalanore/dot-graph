/// each node is an index in a vector in the graph.
// pub type Node = usize;
use crate::{style::Style, utils::quote_string};

/// `Graph`'s node
#[derive(Clone)]
pub struct Node {
    pub name: String,
    label: String,
    style: Style,
    color: Option<String>,
    shape: Option<String>,
    url: String,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: new_name(name),
            label: String::from(name),
            style: Style::None,
            color: None,
            shape: None,
            url: Default::default(),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = String::from(label);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn shape(mut self, shape: Option<&str>) -> Self {
        match shape {
            Some(s) => self.shape = Some(String::from(s)),
            None => self.shape = None,
        }
        self
    }

    pub fn color(mut self, color: Option<&str>) -> Self {
        self.color = match color {
            Some(c) => Some(String::from(c)),
            None => None,
        };
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn to_dot_string(&self) -> String {
        let colorstring: String;

        let escaped_label: String = quote_string(self.label.clone());
        let escaped_url: String = quote_string(self.url.clone());
        let shape: String;

        let mut text = vec!["\"", self.name.as_str(), "\""];

        text.push("[label=");
        text.push(escaped_label.as_str());
        text.push("]");

        if !self.url.is_empty() {
            text.push("[URL=");
            text.push(escaped_url.as_str());
            text.push("]");
        }

        if self.style != Style::None {
            text.push("[style=\"");
            text.push(self.style.as_slice());
            text.push("\"]");
        }

        if let Some(c) = &self.color {
            colorstring = quote_string(c.to_string());
            text.push("[color=");
            text.push(&colorstring);
            text.push("]");
        }

        if let Some(s) = self.shape.clone() {
            shape = s;
            text.push("[shape=\"");
            text.push(&shape);
            text.push("\"]");
        }

        text.push(";");
        return text.into_iter().collect();
    }
}

/// Check if the node's name is illegal.
///
/// The caller must ensure that the input conforms to an
/// identifier format: it must be a non-empty string made up of
/// alphanumeric or underscore characters, not beginning with a
/// digit (i.e. the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`).
///
/// (Note: this format is a strict subset of the `ID` format
/// defined by the DOT language.  This function may change in the
/// future to accept a broader subset, or the entirety, of DOT's
/// `ID` format.)
///
/// Passing an invalid string (containing spaces, brackets,
/// quotes, ...) will cause panic.
fn new_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if is_letter_or_underscore_or_dot(c) => {}
        _ => panic!("The name of the node should start with a letter or underscore or dot"),
    }
    if !chars.all(is_constituent) {
        panic!("The name of the node should only contain letter/number/underscore/dot")
    }
    return String::from(name);

    fn is_letter_or_underscore_or_dot(c: char) -> bool {
        in_range('a', c, 'z') || in_range('A', c, 'Z') || c == '_' || c == '.'
    }
    fn is_constituent(c: char) -> bool {
        is_letter_or_underscore_or_dot(c) || in_range('0', c, '9')
    }
    fn in_range(low: char, c: char, high: char) -> bool {
        low as usize <= c as usize && c as usize <= high as usize
    }
}
