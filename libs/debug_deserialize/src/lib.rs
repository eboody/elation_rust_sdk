use colored::*;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::any::type_name;

#[derive(Debug)]
pub struct DebugDeserialize<T>(pub T);

impl<'de, T> Deserialize<'de> for DebugDeserialize<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        print_usage_warning::<T>();
        let json = Value::deserialize(deserializer)?;
        let json_string = serde_json::to_string(&json).map_err(D::Error::custom)?;
        let first_pass_result = serde_json::from_str::<T>(&json_string);
        if let Err(first_pass_error) = first_pass_result {
            println!(
                "{} {}",
                "Deserialization error:".red().bold(),
                first_pass_error
            );
            if let Some(column) = extract_column(&first_pass_error.to_string()) {
                if let Some((field, context)) = find_problematic_field(&json_string, column) {
                    println!(
                        "\n{} {}\n",
                        "Problematic field:".green().bold(),
                        field.white().on_red().bold()
                    );
                    println!("{}", "Context:".cyan().bold());
                    println!("{}", context);
                } else {
                    println!(
                        "{}",
                        "Couldn't identify the specific problematic field."
                            .yellow()
                            .bold()
                    );
                    println!("{}", "JSON snippet around error:".cyan().bold());
                    println!("{}", create_context(&json_string, column));
                }
            }
            print_usage_warning::<T>();
            return Err(D::Error::custom(first_pass_error.to_string()));
        }
        print_usage_warning::<T>();
        match serde_json::from_value::<T>(json) {
            Ok(value) => Ok(DebugDeserialize(value)),
            Err(err) => Err(D::Error::custom(err.to_string())),
        }
    }
}

fn print_usage_warning<T>() {
    println!(
        "\n{}\n",
        format!(
            "WARNING: You're using DebugDeserialize on your struct: {}! Remove this in production!",
            type_name::<T>()
        )
        .yellow()
        .bold()
    );
}

fn extract_column(error_message: &str) -> Option<usize> {
    let column_start = error_message.find("column ")? + 7;
    error_message[column_start..]
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

fn find_problematic_field(json: &str, target_column: usize) -> Option<(String, String)> {
    let mut field_start = 0;
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    for (i, ch) in json.chars().enumerate() {
        if i + 1 == target_column {
            return Some((
                current_field
                    .trim_matches(|c: char| c == '"' || c.is_whitespace())
                    .to_string(),
                create_context(json, i),
            ));
        }
        if !escaped && ch == '"' {
            in_quotes = !in_quotes;
        }
        if in_quotes {
            escaped = !escaped && ch == '\\';
        } else {
            if ch == ':' {
                current_field = json[field_start..i]
                    .trim_matches(|c: char| c == '"' || c.is_whitespace())
                    .to_string();
            } else if ch == ',' {
                field_start = i + 1;
            }
        }
    }
    None
}

fn create_context(json: &str, error_position: usize) -> String {
    let start = error_position.saturating_sub(50);
    let end = (error_position + 50).min(json.len());
    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < json.len() { "..." } else { "" };
    let context = format!("{}{}{}", prefix, &json[start..end], suffix);
    let error_index = error_position - start + prefix.len();
    let mut result = String::new();
    result.push_str(&context);
    result.push('\n');
    result.push_str(&" ".repeat(error_index));
    result.push_str(&"^".red().bold().to_string());
    result
}
