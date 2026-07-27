use std::{io, path::Path};

use inputcodex_domain::{
    ApplicationVersion, CodexInstallation, InstalledVersion, InstalledVersionUnknownReason,
};

use super::{LimitedRead, MetadataReader};

pub(super) const VERSION_FILE_LIMIT: usize = 256;
const PACKAGE_PREFIXES: [&str; 3] = [
    "OpenAI.Codex_",
    "OpenAI.CodexBeta_",
    "OpenAI.ChatGPT-Desktop_",
];

pub(super) fn resolve_installed_version(
    installation: &CodexInstallation,
    reader: &impl MetadataReader,
) -> InstalledVersion {
    let metadata_root = metadata_root(installation.application_root().as_path());
    if let Some(version) =
        package_version(metadata_root).or_else(|| directory_version(metadata_root))
    {
        return InstalledVersion::Known(version);
    }

    match reader.read_limited(&metadata_root.join("version"), VERSION_FILE_LIMIT) {
        Ok(LimitedRead::Bytes(bytes)) => version_from_bytes(&bytes),
        Ok(LimitedRead::TooLarge) => {
            InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataMissing)
        }
        Err(_) => InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable),
    }
}

fn metadata_root(application_root: &Path) -> &Path {
    let is_nested = application_root
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("app") || name.eq_ignore_ascii_case("bin"));
    if is_nested {
        application_root.parent().unwrap_or(application_root)
    } else {
        application_root
    }
}

fn package_version(path: &Path) -> Option<ApplicationVersion> {
    let name = path.file_name()?.to_str()?;
    PACKAGE_PREFIXES.into_iter().find_map(|prefix| {
        let rest = strip_prefix_ignore_ascii_case(name, prefix)?;
        let version = rest.split('_').next()?;
        is_version_like(version)
            .then(|| ApplicationVersion::new(version.to_owned()).ok())
            .flatten()
    })
}

fn directory_version(path: &Path) -> Option<ApplicationVersion> {
    let version = path.file_name()?.to_str()?;
    is_version_like(version)
        .then(|| ApplicationVersion::new(version.to_owned()).ok())
        .flatten()
}

fn version_from_bytes(bytes: &[u8]) -> InstalledVersion {
    let Ok(value) = std::str::from_utf8(bytes) else {
        return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable);
    };
    match ApplicationVersion::new(value.to_owned()) {
        Ok(version) => InstalledVersion::Known(version),
        Err(_) => InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid),
    }
}

fn is_version_like(version: &str) -> bool {
    let parts = version.split('.').collect::<Vec<_>>();
    parts.len() >= 2
        && parts.iter().all(|part| {
            !part.is_empty() && part.chars().all(|character| character.is_ascii_digit())
        })
}

fn strip_prefix_ignore_ascii_case<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    if value.len() < prefix.len() {
        return None;
    }
    let (head, rest) = value.split_at(prefix.len());
    head.eq_ignore_ascii_case(prefix).then_some(rest)
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_domain::{
        ApplicationInstallSource, CodexInstallation, InstalledVersion,
        InstalledVersionUnknownReason, PrivatePath,
    };

    use super::{VERSION_FILE_LIMIT, resolve_installed_version};
    use crate::application_overview::{LimitedRead, MetadataReader};

    enum ReadResult {
        Bytes(Vec<u8>),
        TooLarge,
        Error(io::ErrorKind),
    }

    struct MemoryReader {
        result: ReadResult,
        reads: Cell<usize>,
        last_path: RefCell<Option<PathBuf>>,
        last_limit: Cell<usize>,
    }

    impl MemoryReader {
        fn new(result: ReadResult) -> Self {
            Self {
                result,
                reads: Cell::new(0),
                last_path: RefCell::new(None),
                last_limit: Cell::new(0),
            }
        }
    }

    impl MetadataReader for MemoryReader {
        fn read_limited(&self, path: &Path, limit: usize) -> io::Result<LimitedRead> {
            self.reads.set(self.reads.get() + 1);
            self.last_path.replace(Some(path.to_path_buf()));
            self.last_limit.set(limit);
            match &self.result {
                ReadResult::Bytes(bytes) => Ok(LimitedRead::Bytes(bytes.clone())),
                ReadResult::TooLarge => Ok(LimitedRead::TooLarge),
                ReadResult::Error(kind) => Err(io::Error::from(*kind)),
            }
        }
    }

    fn installation(root: PathBuf, source: ApplicationInstallSource) -> CodexInstallation {
        CodexInstallation::new(
            PrivatePath::new(root.clone()).expect("测试安装根必须是绝对路径"),
            PrivatePath::new(root.join("Codex.exe")).expect("测试可执行文件必须是绝对路径"),
            source,
        )
    }

    #[test]
    fn 包目录版本优先且不读取版本文件() {
        let package = std::env::temp_dir()
            .join("OpenAI.Codex_1.2.43.0_x64__2p2nqsd0c76g0")
            .join("app");
        let reader = MemoryReader::new(ReadResult::Error(io::ErrorKind::PermissionDenied));

        let version = resolve_installed_version(
            &installation(package, ApplicationInstallSource::WindowsPackage),
            &reader,
        );

        assert!(matches!(
            version,
            InstalledVersion::Known(value) if value.as_str() == "1.2.43.0"
        ));
        assert_eq!(reader.reads.get(), 0);
    }

    #[test]
    fn 固定_version_文件只读取一次且限制二百五十六字节() {
        let root = std::env::temp_dir().join("inputcodex-windows-version-file");
        let reader = MemoryReader::new(ReadResult::Bytes(b"2.4.6\n".to_vec()));

        let version = resolve_installed_version(
            &installation(root.clone(), ApplicationInstallSource::WindowsStandalone),
            &reader,
        );

        assert!(matches!(
            version,
            InstalledVersion::Known(value) if value.as_str() == "2.4.6"
        ));
        assert_eq!(reader.reads.get(), 1);
        assert_eq!(reader.last_limit.get(), VERSION_FILE_LIMIT);
        assert_eq!(
            reader.last_path.borrow().as_deref(),
            Some(root.join("version").as_path())
        );
    }

    #[test]
    fn 版本文件问题不会否定已确认安装() {
        let root = std::env::temp_dir().join("inputcodex-windows-version-unknown");
        let cases = [
            (
                ReadResult::Error(io::ErrorKind::NotFound),
                InstalledVersionUnknownReason::MetadataMissing,
            ),
            (
                ReadResult::Error(io::ErrorKind::PermissionDenied),
                InstalledVersionUnknownReason::MetadataUnreadable,
            ),
            (
                ReadResult::TooLarge,
                InstalledVersionUnknownReason::MetadataInvalid,
            ),
            (
                ReadResult::Bytes(vec![0xff, 0xfe]),
                InstalledVersionUnknownReason::MetadataUnreadable,
            ),
            (
                ReadResult::Bytes(b"   ".to_vec()),
                InstalledVersionUnknownReason::MetadataInvalid,
            ),
        ];

        for (result, expected) in cases {
            let reader = MemoryReader::new(result);
            assert_eq!(
                resolve_installed_version(
                    &installation(root.clone(), ApplicationInstallSource::WindowsStandalone),
                    &reader,
                ),
                InstalledVersion::Unknown(expected)
            );
            assert_eq!(reader.reads.get(), 1);
        }
    }
}
