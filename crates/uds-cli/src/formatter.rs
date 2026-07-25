use serde::Serialize;

pub trait Formatter {
    fn format<T: Serialize>(&self, value: &T) -> String;
}

pub struct HumanFormatter;

impl Formatter for HumanFormatter {
    fn format<T: Serialize>(&self, _value: &T) -> String {
        String::new()
    }
}

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format<T: Serialize>(&self, value: &T) -> String {
        serde_json::to_string_pretty(value).unwrap_or_default()
    }
}
