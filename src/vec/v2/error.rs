use std::fmt;

#[derive(Debug)]
pub enum PageError {
    ZeroPerPage,
}

impl fmt::Display for PageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PageError::ZeroPerPage => write!(f, "per_page must be greater than zero"),
        }
    }
}

impl std::error::Error for PageError {}

#[derive(Debug)]
pub enum TableError {
    Empty,
    NoFields,
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableError::Empty => write!(f, "no data to display"),
            TableError::NoFields => write!(f, "no fields to display"),
        }
    }
}

impl std::error::Error for TableError {}
