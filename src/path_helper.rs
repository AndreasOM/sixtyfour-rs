use std::path::Path;
use std::path::PathBuf;

pub struct PathHelper {}

impl PathHelper {
    pub fn strip_prefix<'a>(file: &'a Path, prefix: &Path) -> Option<&'a Path> {
        let file_os = file.as_os_str();
        let file_str = file_os.to_str().expect("file is utf-8");
        let prefix_os = prefix.as_os_str();
        let prefix_str = prefix_os.to_str().expect("prefix is utf-8");

        eprintln!("{file_str} {prefix_str}");

        file_str.strip_prefix(prefix_str).map(|s| {
            let s = s.strip_prefix("/").unwrap_or(s);
            Path::new(s)
        })
    }

    pub fn prefix_with(file: &Path, prefix: Option<&Path>) -> PathBuf {
        if let Some(prefix) = prefix {
            prefix.to_path_buf().join(file)
        } else {
            file.to_path_buf()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_prefix_works() {
        let folder = Path::new("/home/tester/abc");
        let file = Path::new("/home/tester/abc/file1.txt");
        let file2 = Path::new("/home/tester/def/file2.txt");

        assert_eq!(
            Some(Path::new("file1.txt")),
            PathHelper::strip_prefix(file, folder)
        );
        assert_eq!(None, PathHelper::strip_prefix(file2, folder));
        assert_eq!(
            Some(Path::new("file1.txt")),
            PathHelper::strip_prefix(Path::new("file1.txt"), Path::new(""))
        );
    }

    #[test]
    fn prefix_with_works() {
        assert_eq!(
            Path::new("file1.txt"),
            PathHelper::prefix_with(Path::new("file1.txt"), None)
        );
        assert_eq!(
            Path::new("/home/tester/file1.txt"),
            PathHelper::prefix_with(Path::new("file1.txt"), Some(Path::new("/home/tester")))
        );
    }
}
