use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    Request(reqwest::Error),
    Status(u16),
    Io(std::io::Error),
    HashMismatch { expected: String, actual: String },
    RetriesExceeded(u32),
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Request(e) => write!(f, "http request failed: {e}"),
            HttpError::Status(code) => write!(f, "http returned non-success status: {code}"),
            HttpError::Io(e) => write!(f, "io error: {e}"),
            HttpError::HashMismatch { expected, actual } => {
                write!(f, "sha1 mismatch: expected {expected}, got {actual}")
            }
            HttpError::RetriesExceeded(n) => write!(f, "retries exceeded ({n})"),
        }
    }
}

impl HttpError {
    pub fn is_permanent(&self) -> bool {
        match self {
            HttpError::Status(code) => *code >= 400 && *code < 500 && *code != 408 && *code != 429,
            _ => false,
        }
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HttpError::Request(e) => Some(e),
            HttpError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        HttpError::Request(e)
    }
}

impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        HttpError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_display() {
        let e = HttpError::Status(404);
        assert!(e.to_string().contains("404"));
    }
}
