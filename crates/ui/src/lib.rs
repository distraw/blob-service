use console::style;

const BORDER_LENGTH: usize = 20;

pub fn header(text: &str) -> String {
    format!(
        "\n{}\n{}\n{}",
        "═".repeat(BORDER_LENGTH),
        style(text).bold().cyan(),
        "═".repeat(BORDER_LENGTH)
    )
}

pub fn section_header(text: &str) -> String {
    style(format!("┌─ {}", text)).bold().blue().to_string()
}

pub fn success_footer(text: &str) -> String {
    format!(
        "{}",
        style(text).bold().green()
    )
}