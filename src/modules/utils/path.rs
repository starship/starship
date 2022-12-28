use std::path::Path;

pub trait PathExt {
    /// Compare this path with another path, ignoring
    /// the differences between Verbatim and Non-Verbatim paths.
    fn normalised_equals(&self, other: &Path) -> bool;
    /// Determine if this path starts wit with another path fragment, ignoring
    /// the differences between Verbatim and Non-Verbatim paths.
    fn normalised_starts_with(&self, other: &Path) -> bool;
    /// Strips the path Prefix component from the Path, if there is one
    /// E.g. `\\?\path\foo` => `\foo`
    /// E.g. `\\?\C:\foo` => `\foo`
    /// E.g. `\\?\UNC\server\share\foo` => `\foo`
    /// E.g. `/foo/bar` => `/foo/bar`
    fn without_prefix(&self) -> &Path;
    /// Get device / volume info
    fn device_id(&self) -> u64;
}

#[cfg(windows)]
mod normalize {
    use std::ffi::OsStr;
    use std::path::{Component, Path, Prefix};

    // allow because this mirrors std
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum NormalizedPrefix<'a> {
        // No prefix, e.g. `\cat_pics` or `/cat_pics`
        None,
        /// Simple verbatim prefix, e.g. `\\?\cat_pics`.
        Verbatim(&'a OsStr),
        /// Device namespace prefix, e.g. `\\.\COM42`.
        DeviceNS(&'a OsStr),
        /// Prefix using Windows' _**U**niform **N**aming **C**onvention_, e.g. `\\server\share` or `\\?\UNC\server\share`
        UNC(&'a OsStr, &'a OsStr),
        /// Windows disk/drive prefix e.g. `C:` or `\\?\C:`
        Disk(u8),
    }

    /// Normalise Verbatim and Non-Verbatim path prefixes into a comparable structure.
    /// NOTE: "Verbatim" paths are the rust std library's name for Windows extended-path prefixed paths.
    #[cfg(windows)]
    fn normalize_prefix(prefix: Prefix) -> NormalizedPrefix {
        match prefix {
            Prefix::Verbatim(segment) => NormalizedPrefix::Verbatim(segment),
            Prefix::VerbatimUNC(server, share) => NormalizedPrefix::UNC(server, share),
            Prefix::VerbatimDisk(disk) => NormalizedPrefix::Disk(disk),
            Prefix::DeviceNS(device) => NormalizedPrefix::DeviceNS(device),
            Prefix::UNC(server, share) => NormalizedPrefix::UNC(server, share),
            Prefix::Disk(disk) => NormalizedPrefix::Disk(disk),
        }
    }

    #[cfg(windows)]
    pub fn normalize_path(path: &Path) -> (NormalizedPrefix, &Path) {
        let mut components = path.components();
        if let Some(Component::Prefix(prefix)) = components.next() {
            return (normalize_prefix(prefix.kind()), components.as_path());
        }
        (NormalizedPrefix::None, path)
    }
}

#[cfg(windows)]
impl PathExt for Path {
    fn normalised_starts_with(&self, other: &Path) -> bool {
        // Do a structured comparison of two paths (normalising differences between path prefixes)
        let (a_prefix, a_path) = normalize::normalize_path(self);
        let (b_prefix, b_path) = normalize::normalize_path(other);
        a_prefix == b_prefix && a_path.starts_with(b_path)
    }

    fn normalised_equals(&self, other: &Path) -> bool {
        // Do a structured comparison of two paths (normalising differences between path prefixes)
        let (a_prefix, a_path) = normalize::normalize_path(self);
        let (b_prefix, b_path) = normalize::normalize_path(other);
        a_prefix == b_prefix && a_path == b_path
    }

    fn without_prefix(&self) -> &Path {
        let (_, path) = normalize::normalize_path(self);
        path
    }
}

// NOTE: Windows path prefixes are only parsed on Windows.
// On other platforms, we can fall back to the non-normalized versions of these routines.
#[cfg(not(windows))]
impl PathExt for Path {
    #[inline]
    fn normalised_starts_with(&self, other: &Path) -> bool {
        self.starts_with(other)
    }

    #[inline]
    fn normalised_equals(&self, other: &Path) -> bool {
        self == other
    }

    #[inline]
    fn without_prefix(&self) -> &Path {
        self
    }

    #[cfg(target_os = "linux")]
    fn device_id(&self) -> u64 {
        use std::os::linux::fs::MetadataExt;
        let m = self.metadata().unwrap();
        m.st_dev()
    }

    #[cfg(all(unix, not(target_os = "linux")))]
    pub fn device_id(&self) -> u64 {
        use std::os::unix::fs::MetadataExt;
        let m = self.metadata().unwrap();
        m.dev()
    }
}

#[cfg(test)]
#[cfg(windows)]
mod windows {
    use super::*;

    #[test]
    fn normalised_equals() {
        fn test_equals(a: &Path, b: &Path) {
            assert!(a.normalised_equals(b));
            assert!(b.normalised_equals(a));
        }

        // UNC paths
        let verbatim_unc = Path::new(r"\\?\UNC\server\share\sub\path");
        let unc = Path::new(r"\\server\share\sub\path");
        test_equals(verbatim_unc, verbatim_unc);
        test_equals(verbatim_unc, unc);
        test_equals(unc, unc);
        test_equals(unc, verbatim_unc);

        // Disk paths
        let verbatim_disk = Path::new(r"\\?\C:\test\path");
        let disk = Path::new(r"C:\test\path");
        test_equals(verbatim_disk, verbatim_disk);
        test_equals(verbatim_disk, disk);
        test_equals(disk, disk);
        test_equals(disk, verbatim_disk);

        // Other paths
        let verbatim = Path::new(r"\\?\cat_pics");
        let no_prefix = Path::new(r"\cat_pics");
        let device_ns = Path::new(r"\\.\COM42");
        test_equals(verbatim, verbatim);
        test_equals(no_prefix, no_prefix);
        test_equals(device_ns, device_ns);
    }

    #[test]
    fn normalised_equals_differing_prefixes() {
        fn test_not_equals(a: &Path, b: &Path) {
            assert!(!a.normalised_equals(b));
            assert!(!b.normalised_equals(a));
        }

        let verbatim_unc = Path::new(r"\\?\UNC\server\share\sub\path");
        let unc = Path::new(r"\\server\share\sub\path");
        let verbatim_disk = Path::new(r"\\?\C:\test\path");
        let disk = Path::new(r"C:\test\path");
        let verbatim = Path::new(r"\\?\cat_pics");
        let no_prefix = Path::new(r"\cat_pics");
        let device_ns = Path::new(r"\\.\COM42");

        test_not_equals(verbatim_unc, verbatim_disk);
        test_not_equals(unc, disk);
        test_not_equals(disk, device_ns);
        test_not_equals(device_ns, verbatim_disk);
        test_not_equals(no_prefix, unc);
        test_not_equals(no_prefix, verbatim);
    }

    #[test]
    fn normalised_starts_with() {
        fn test_starts_with(a: &Path, b: &Path) {
            assert!(a.normalised_starts_with(b));
            assert!(!b.normalised_starts_with(a));
        }

        // UNC paths
        let verbatim_unc_a = Path::new(r"\\?\UNC\server\share\a\b\c\d");
        let verbatim_unc_b = Path::new(r"\\?\UNC\server\share\a\b");
        let unc_a = Path::new(r"\\server\share\a\b\c\d");
        let unc_b = Path::new(r"\\server\share\a\b");
        test_starts_with(verbatim_unc_a, verbatim_unc_b);
        test_starts_with(unc_a, unc_b);
        test_starts_with(verbatim_unc_a, unc_b);
        test_starts_with(unc_a, verbatim_unc_b);

        // Disk paths
        let verbatim_disk_a = Path::new(r"\\?\C:\a\b\c\d");
        let verbatim_disk_b = Path::new(r"\\?\C:\a\b");
        let disk_a = Path::new(r"C:\a\b\c\d");
        let disk_b = Path::new(r"C:\a\b");
        test_starts_with(verbatim_disk_a, verbatim_disk_b);
        test_starts_with(disk_a, disk_b);
        test_starts_with(disk_a, verbatim_disk_b);
        test_starts_with(verbatim_disk_a, disk_b);

        // Other paths
        let verbatim_a = Path::new(r"\\?\cat_pics\a\b\c\d");
        let verbatim_b = Path::new(r"\\?\cat_pics\a\b");
        let device_ns_a = Path::new(r"\\.\COM43\a\b\c\d");
        let device_ns_b = Path::new(r"\\.\COM43\a\b");
        let no_prefix_a = Path::new(r"\a\b\c\d");
        let no_prefix_b = Path::new(r"\a\b");
        test_starts_with(verbatim_a, verbatim_b);
        test_starts_with(device_ns_a, device_ns_b);
        test_starts_with(no_prefix_a, no_prefix_b);
    }

    #[test]
    fn normalised_starts_with_differing_prefixes() {
        fn test_not_starts_with(a: &Path, b: &Path) {
            assert!(!a.normalised_starts_with(b));
            assert!(!b.normalised_starts_with(a));
        }

        let verbatim_unc = Path::new(r"\\?\UNC\server\share\a\b\c\d");
        let unc = Path::new(r"\\server\share\a\b\c\d");
        let verbatim_disk = Path::new(r"\\?\C:\a\b\c\d");
        let disk = Path::new(r"C:\a\b\c\d");
        let verbatim = Path::new(r"\\?\cat_pics\a\b\c\d");
        let device_ns = Path::new(r"\\.\COM43\a\b\c\d");
        let no_prefix = Path::new(r"\a\b\c\d");

        test_not_starts_with(verbatim_unc, device_ns);
        test_not_starts_with(unc, device_ns);
        test_not_starts_with(verbatim_disk, verbatim);
        test_not_starts_with(disk, verbatim);
        test_not_starts_with(disk, unc);
        test_not_starts_with(verbatim_disk, no_prefix);
    }

    #[test]
    fn without_prefix() {
        // UNC paths
        assert_eq!(
            Path::new(r"\\?\UNC\server\share\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        assert_eq!(
            Path::new(r"\\server\share\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        // Disk paths
        assert_eq!(
            Path::new(r"\\?\C:\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        assert_eq!(
            Path::new(r"C:\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        // Other paths
        assert_eq!(
            Path::new(r"\\?\cat_pics\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        assert_eq!(
            Path::new(r"\\.\COM42\sub\path").without_prefix(),
            Path::new(r"\sub\path")
        );
        // No prefix
        assert_eq!(
            Path::new(r"\cat_pics\sub\path").without_prefix(),
            Path::new(r"\cat_pics\sub\path")
        );
    }
}

#[cfg(test)]
#[cfg(not(windows))]
mod nix {
    use super::*;

    #[test]
    fn normalised_equals() {
        let path_a = Path::new("/a/b/c/d");
        let path_b = Path::new("/a/b/c/d");
        assert!(path_a.normalised_equals(path_b));
        assert!(path_b.normalised_equals(path_a));

        let path_c = Path::new("/a/b");
        assert!(!path_a.normalised_equals(path_c));
    }

    #[test]
    fn normalised_equals_differing_prefixes() {
        // Windows path prefixes are not parsed on *nix
        let path_a = Path::new(r"\\?\UNC\server\share\a\b\c\d");
        let path_b = Path::new(r"\\server\share\a\b\c\d");
        assert!(!path_a.normalised_equals(path_b));
        assert!(!path_b.normalised_equals(path_a));

        assert!(path_a.normalised_equals(path_a));
    }

    #[test]
    fn normalised_starts_with() {
        let path_a = Path::new("/a/b/c/d");
        let path_b = Path::new("/a/b");
        assert!(path_a.normalised_starts_with(path_b));
        assert!(!path_b.normalised_starts_with(path_a));
    }

    #[test]
    fn normalised_starts_with_differing_prefixes() {
        // Windows path prefixes are not parsed on *nix
        let path_a = Path::new(r"\\?\UNC\server\share\a\b\c\d");
        let path_b = Path::new(r"\\server\share\a\b");
        assert!(!path_a.normalised_starts_with(path_b));
        assert!(!path_b.normalised_starts_with(path_a));

        assert!(path_a.normalised_starts_with(path_a));
    }

    #[test]
    fn without_prefix() {
        // Windows path prefixes are not parsed on *nix

        // UNC paths
        assert_eq!(
            Path::new(r"\\?\UNC\server\share\sub\path").without_prefix(),
            Path::new(r"\\?\UNC\server\share\sub\path")
        );
        assert_eq!(
            Path::new(r"\\server\share\sub\path").without_prefix(),
            Path::new(r"\\server\share\sub\path")
        );
        // Disk paths
        assert_eq!(
            Path::new(r"\\?\C:\sub\path").without_prefix(),
            Path::new(r"\\?\C:\sub\path")
        );
        assert_eq!(
            Path::new(r"C:\sub\path").without_prefix(),
            Path::new(r"C:\sub\path")
        );
        // Other paths
        assert_eq!(
            Path::new(r"\\?\cat_pics\sub\path").without_prefix(),
            Path::new(r"\\?\cat_pics\sub\path")
        );
        assert_eq!(
            Path::new(r"\\.\COM42\sub\path").without_prefix(),
            Path::new(r"\\.\COM42\sub\path")
        );
        // No prefix
        assert_eq!(
            Path::new(r"\cat_pics\sub\path").without_prefix(),
            Path::new(r"\cat_pics\sub\path")
        );
    }
}
