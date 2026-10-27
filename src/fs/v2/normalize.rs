use std::path::{Component, Path, PathBuf};

pub fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => match result.components().next_back() {
                Some(Component::Normal(_)) => {
                    result.pop();
                }
                Some(Component::RootDir) | Some(Component::Prefix(_)) => {}
                _ => result.push(Component::ParentDir),
            },
            c => result.push(c),
        }
    }
    result
}

pub fn normalize_path(path: &Path) -> PathBuf {
    normalize(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_dots() {
        assert_eq!(normalize(Path::new("a/./b/../c")), PathBuf::from("a/c"));
    }

    #[test]
    fn normalize_root_parent() {
        assert_eq!(normalize(Path::new("/..")), PathBuf::from("/"));
    }

    #[test]
    fn normalize_relative_parent() {
        assert_eq!(normalize(Path::new("a/../../b")), PathBuf::from("../b"));
    }
}
