use std::path::{Path, PathBuf};

use pathdiff::diff_paths;

pub trait PathUtils<T>
where
    T: AsRef<Path>,
{
    fn is_ancestor_of(&self, maybe_child: &T) -> bool;
    fn relative_path_from_ancsestor(&self, maybe_ancsestor: T) -> Option<PathBuf>;
}

impl<T, U> PathUtils<T> for U
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    fn is_ancestor_of(&self, maybe_child: &T) -> bool {
        let self_path_buf = self.as_ref();
        maybe_child
            .as_ref()
            .to_path_buf()
            .ancestors()
            .into_iter()
            .any(|path| self_path_buf.eq(path))
    }

    fn relative_path_from_ancsestor(&self, maybe_ancsestor: T) -> Option<PathBuf> {
        if maybe_ancsestor.is_ancestor_of(self) {
            diff_paths(self, maybe_ancsestor)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::PathUtils;

    #[test]
    fn another() {
        let a = Path::new("/home/yuki");
        let b: PathBuf = Path::new("/home/yuki/program").into();
        assert!(a.is_ancestor_of(&b));
        let c: PathBuf = Path::new("/home/yuki").into();
        let d = Path::new("/home/hogem");
        assert!(!c.is_ancestor_of(&d));
    }

    #[test]
    fn test_relative_path_from_ancsestor() {
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/yuki/program/hoge/fuga").into();
        assert_eq!(
            b.relative_path_from_ancsestor(a),
            Some(Path::new("program/hoge/fuga").to_path_buf())
        );
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/yuki-nagato/program/hoge/fuga").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/home/poyo").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
        let a: PathBuf = Path::new("/home/yuki").into();
        let b: PathBuf = Path::new("/monyo").into();
        assert_eq!(b.relative_path_from_ancsestor(a), None);
    }
}
