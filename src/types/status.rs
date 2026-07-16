use crate::color::{Colors,color_str};

pub enum Status {
    Ok,
    Err,
    Inf,
    Warn,
    Task,
    Note,
    Quote,
    No,
    Yes,
    Question,
    Option,
    Select,
    Status
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Ok => "!Ok",
            Status::Err => "!Err",
            Status::Inf => ":Info",
            Status::Warn => "!Warn",
            Status::Task => ":Task",
            Status::Note => ":!Note",
            Status::Quote => ":>Quote",
            Status::No => "!No",
            Status::Yes => "!Yes",
            Status::Question => "?Question",
            Status::Option => ":?Option",
            Status::Select => ":?Select",
            Status::Status => ":Status"
        }
    }

    pub fn as_colored_str(&self) -> String {
        let text = self.as_str();
        match self {
            Status::Ok | Status::Yes => color_str(text, Colors::Lime),
            Status::Err | Status::No => color_str(text, Colors::Rose),
            Status::Warn => color_str(text, Colors::Orange),
            Status::Inf | Status::Status | Status::Task => color_str(text, Colors::Gold),
            Status::Note | Status::Select | Status::Option => color_str(text, Colors::Yellow),
            Status::Quote => color_str(text, Colors::Gray),
            Status::Question => color_str(text, Colors::Purple),
        }
    }
}
