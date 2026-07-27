use std::io;

use inputcodex_domain::{
    ApplicationVersion, CodexInstallation, InstalledVersion, InstalledVersionUnknownReason,
};

use super::{LimitedRead, MetadataReader};

pub(super) const PLIST_FILE_LIMIT: usize = 65_536;

pub(super) fn resolve_installed_version(
    installation: &CodexInstallation,
    reader: &impl MetadataReader,
) -> InstalledVersion {
    let path = installation
        .application_root()
        .as_path()
        .join("Contents/Info.plist");
    let bytes = match reader.read_limited(&path, PLIST_FILE_LIMIT) {
        Ok(LimitedRead::Bytes(bytes)) => bytes,
        Ok(LimitedRead::TooLarge) => {
            return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid);
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataMissing);
        }
        Err(_) => {
            return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable);
        }
    };
    if bytes.starts_with(b"bplist") {
        return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable);
    }
    let Ok(plist) = std::str::from_utf8(&bytes) else {
        return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataUnreadable);
    };

    match plist_string_value(plist, "CFBundleShortVersionString") {
        PlistValue::Value(value) => validated_version(value),
        PlistValue::Invalid => {
            InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid)
        }
        PlistValue::Missing => match plist_string_value(plist, "CFBundleVersion") {
            PlistValue::Value(value) => validated_version(value),
            PlistValue::Invalid => {
                InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid)
            }
            PlistValue::Missing => {
                InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataMissing)
            }
        },
    }
}

fn validated_version(value: &str) -> InstalledVersion {
    if value.contains(['<', '&']) {
        return InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid);
    }
    match ApplicationVersion::new(value.to_owned()) {
        Ok(version) => InstalledVersion::Known(version),
        Err(_) => InstalledVersion::Unknown(InstalledVersionUnknownReason::MetadataInvalid),
    }
}

enum PlistValue<'a> {
    Value(&'a str),
    Missing,
    Invalid,
}

fn plist_string_value<'a>(plist: &'a str, key: &str) -> PlistValue<'a> {
    let marker = format!("<key>{key}</key>");
    let Some((_, after_key)) = plist.split_once(&marker) else {
        return PlistValue::Missing;
    };
    let after_key = after_key.trim_start();
    let Some(after_open) = after_key.strip_prefix("<string>") else {
        return PlistValue::Invalid;
    };
    let Some((value, _)) = after_open.split_once("</string>") else {
        return PlistValue::Invalid;
    };
    PlistValue::Value(value.trim())
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

    use super::{PLIST_FILE_LIMIT, resolve_installed_version};
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

    fn installation(root: PathBuf) -> CodexInstallation {
        CodexInstallation::new(
            PrivatePath::new(root.clone()).expect("测试应用包必须是绝对路径"),
            PrivatePath::new(root.join("Contents/MacOS/Codex"))
                .expect("测试可执行文件必须是绝对路径"),
            ApplicationInstallSource::MacosSystemApplications,
        )
    }

    #[test]
    fn 短版本优先于构建版本且_plist_只读取一次() {
        let root = std::env::temp_dir().join("inputcodex-macos-short-version.app");
        let plist = br#"
            <plist><dict>
              <key>CFBundleShortVersionString</key><string>1.2.43</string>
              <key>CFBundleVersion</key><string>999</string>
            </dict></plist>
        "#;
        let reader = MemoryReader::new(ReadResult::Bytes(plist.to_vec()));

        let version = resolve_installed_version(&installation(root.clone()), &reader);

        assert!(matches!(
            version,
            InstalledVersion::Known(value) if value.as_str() == "1.2.43"
        ));
        assert_eq!(reader.reads.get(), 1);
        assert_eq!(reader.last_limit.get(), PLIST_FILE_LIMIT);
        assert_eq!(
            reader.last_path.borrow().as_deref(),
            Some(root.join("Contents/Info.plist").as_path())
        );
    }

    #[test]
    fn 缺少短版本时回退构建版本() {
        let root = std::env::temp_dir().join("inputcodex-macos-build-version.app");
        let plist = br#"
            <plist><dict>
              <key>CFBundleVersion</key><string>2430</string>
            </dict></plist>
        "#;
        let reader = MemoryReader::new(ReadResult::Bytes(plist.to_vec()));

        assert!(matches!(
            resolve_installed_version(&installation(root), &reader),
            InstalledVersion::Known(value) if value.as_str() == "2430"
        ));
    }

    #[test]
    fn plist_问题不会否定已确认安装() {
        let root = std::env::temp_dir().join("inputcodex-macos-version-unknown.app");
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
                ReadResult::Bytes(b"bplist00\x00\x01".to_vec()),
                InstalledVersionUnknownReason::MetadataUnreadable,
            ),
            (
                ReadResult::Bytes(
                    b"<plist><dict><key>CFBundleShortVersionString</key><integer>1</integer></dict></plist>"
                        .to_vec(),
                ),
                InstalledVersionUnknownReason::MetadataInvalid,
            ),
            (
                ReadResult::Bytes(b"<plist><dict></dict></plist>".to_vec()),
                InstalledVersionUnknownReason::MetadataMissing,
            ),
        ];

        for (result, expected) in cases {
            let reader = MemoryReader::new(result);
            assert_eq!(
                resolve_installed_version(&installation(root.clone()), &reader),
                InstalledVersion::Unknown(expected)
            );
            assert_eq!(reader.reads.get(), 1);
        }
    }
}
