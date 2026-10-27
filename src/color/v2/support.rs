use std::io::IsTerminal;

pub fn color_supported() -> bool {
    if std::env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if let Some(v) = std::env::var_os("CLICOLOR")
        && v == "0"
    {
        return false;
    }
    if let Some(term) = std::env::var_os("TERM")
        && term == "dumb"
    {
        return false;
    }
    std::io::stdout().is_terminal()
}
