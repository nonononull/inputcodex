use std::sync::Mutex;
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use std::{fs, io, path::Path};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use inputcodex_application::{PlatformPathsPort, PlatformPathsRequest};
use inputcodex_application::{
    WatcherPreferenceMutationControl, WatcherPreferenceMutationPhase,
    WatcherPreferenceMutationPort, WatcherPreferenceMutationRequest,
};
#[cfg(any(target_os = "windows", target_os = "macos", test))]
use inputcodex_domain::WatcherPreference;
use inputcodex_domain::{
    DiagnosticCode, WatcherPreferenceFinalObservation, WatcherPreferenceMarkerCommit,
    WatcherPreferenceMutationOutcome, WatcherPreferenceMutationReceipt,
    WatcherPreferenceSetupCommit,
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::SystemPlatformPaths;

#[cfg(any(target_os = "windows", target_os = "macos", test))]
const WATCHER_DISABLED_MARKER: &str = "watcher.disabled";
#[cfg(any(target_os = "windows", target_os = "macos", test))]
const INPUTCODEX_STATE_ROOT_NAME: &str = "inputcodex";

static WATCHER_PREFERENCE_MUTATION_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemWatcherPreferenceMutation;

impl WatcherPreferenceMutationPort for SystemWatcherPreferenceMutation {
    fn mutate(
        &self,
        request: &WatcherPreferenceMutationRequest,
        control: &WatcherPreferenceMutationControl,
    ) -> WatcherPreferenceMutationReceipt {
        if control.phase() == WatcherPreferenceMutationPhase::Cancelled {
            return cancelled_receipt(
                request,
                WatcherPreferenceSetupCommit::NotRequired,
                WatcherPreferenceFinalObservation::Unknown,
            );
        }

        let _guard = match WATCHER_PREFERENCE_MUTATION_LOCK.lock() {
            Ok(guard) => guard,
            Err(_) => {
                return failed_receipt(
                    request,
                    WatcherPreferenceSetupCommit::NotRequired,
                    WatcherPreferenceMarkerCommit::NotAttempted,
                    "WATCHER_PREFERENCE_MUTATION_LOCK_UNAVAILABLE",
                );
            }
        };

        if control.phase() == WatcherPreferenceMutationPhase::Cancelled {
            return cancelled_receipt(
                request,
                WatcherPreferenceSetupCommit::NotRequired,
                WatcherPreferenceFinalObservation::Unknown,
            );
        }

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            let paths = match SystemPlatformPaths.resolve(&PlatformPathsRequest::default()) {
                Ok(paths) => paths,
                Err(_) => {
                    return failed_receipt(
                        request,
                        WatcherPreferenceSetupCommit::NotRequired,
                        WatcherPreferenceMarkerCommit::NotAttempted,
                        "WATCHER_PREFERENCE_MUTATION_PATH_UNAVAILABLE",
                    );
                }
            };
            mutate_watcher_preference_at_state_root(
                paths.inputcodex_state_root().as_path(),
                request,
                control,
                &SystemWatcherPreferenceFileSystem,
            )
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            failed_receipt(
                request,
                WatcherPreferenceSetupCommit::NotRequired,
                WatcherPreferenceMarkerCommit::NotAttempted,
                "WATCHER_PREFERENCE_MUTATION_UNSUPPORTED",
            )
        }
    }
}

fn receipt(
    request: &WatcherPreferenceMutationRequest,
    setup_commit: WatcherPreferenceSetupCommit,
    marker_commit: WatcherPreferenceMarkerCommit,
    final_observation: WatcherPreferenceFinalObservation,
    outcome: WatcherPreferenceMutationOutcome,
    diagnostic_code: &'static str,
) -> WatcherPreferenceMutationReceipt {
    WatcherPreferenceMutationReceipt::new(
        request.request_id(),
        request.desired(),
        setup_commit,
        marker_commit,
        final_observation,
        outcome,
        DiagnosticCode::new(diagnostic_code),
    )
}

fn failed_receipt(
    request: &WatcherPreferenceMutationRequest,
    setup_commit: WatcherPreferenceSetupCommit,
    marker_commit: WatcherPreferenceMarkerCommit,
    diagnostic_code: &'static str,
) -> WatcherPreferenceMutationReceipt {
    receipt(
        request,
        setup_commit,
        marker_commit,
        WatcherPreferenceFinalObservation::Unknown,
        WatcherPreferenceMutationOutcome::Failed,
        diagnostic_code,
    )
}

fn cancelled_receipt(
    request: &WatcherPreferenceMutationRequest,
    setup_commit: WatcherPreferenceSetupCommit,
    final_observation: WatcherPreferenceFinalObservation,
) -> WatcherPreferenceMutationReceipt {
    receipt(
        request,
        setup_commit,
        WatcherPreferenceMarkerCommit::NotAttempted,
        final_observation,
        WatcherPreferenceMutationOutcome::Cancelled,
        "WATCHER_PREFERENCE_MUTATION_CANCELLED",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum WatcherPreferenceEntryKind {
    File,
    Directory,
    LinkOrReparse,
    Other,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
trait WatcherPreferenceFileSystem {
    fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceEntryKind>;
    fn create_dir(&self, path: &Path) -> io::Result<()>;
    fn create_marker_new(&self, path: &Path) -> io::Result<()>;
    fn remove_marker(&self, path: &Path) -> io::Result<()>;
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg(any(target_os = "windows", target_os = "macos"))]
struct SystemWatcherPreferenceFileSystem;

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl WatcherPreferenceFileSystem for SystemWatcherPreferenceFileSystem {
    fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceEntryKind> {
        let metadata = fs::symlink_metadata(path)?;

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;

            const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
            if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Ok(WatcherPreferenceEntryKind::LinkOrReparse);
            }
        }

        let file_type = metadata.file_type();
        Ok(if file_type.is_symlink() {
            WatcherPreferenceEntryKind::LinkOrReparse
        } else if metadata.is_file() {
            WatcherPreferenceEntryKind::File
        } else if metadata.is_dir() {
            WatcherPreferenceEntryKind::Directory
        } else {
            WatcherPreferenceEntryKind::Other
        })
    }

    fn create_dir(&self, path: &Path) -> io::Result<()> {
        fs::create_dir(path)
    }

    fn create_marker_new(&self, path: &Path) -> io::Result<()> {
        drop(
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)?,
        );
        Ok(())
    }

    fn remove_marker(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
enum ContextObservation {
    RootMissing,
    Preference(WatcherPreference),
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
impl ContextObservation {
    const fn preference(self) -> WatcherPreference {
        match self {
            Self::RootMissing => WatcherPreference::EnabledByDefault,
            Self::Preference(preference) => preference,
        }
    }

    const fn final_observation(self) -> WatcherPreferenceFinalObservation {
        WatcherPreferenceFinalObservation::Known(self.preference())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(any(target_os = "windows", target_os = "macos", test))]
struct ContextObservationError {
    code: &'static str,
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn mutate_watcher_preference_at_state_root(
    state_root: &Path,
    request: &WatcherPreferenceMutationRequest,
    control: &WatcherPreferenceMutationControl,
    file_system: &impl WatcherPreferenceFileSystem,
) -> WatcherPreferenceMutationReceipt {
    if control.phase() == WatcherPreferenceMutationPhase::Cancelled {
        return cancelled_receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            WatcherPreferenceFinalObservation::Unknown,
        );
    }

    if !state_root.is_absolute()
        || state_root.file_name().and_then(|name| name.to_str()) != Some(INPUTCODEX_STATE_ROOT_NAME)
    {
        return failed_receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            WatcherPreferenceMarkerCommit::NotAttempted,
            "WATCHER_PREFERENCE_MUTATION_PATH_INVALID",
        );
    }
    let Some(parent) = state_root.parent() else {
        return failed_receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            WatcherPreferenceMarkerCommit::NotAttempted,
            "WATCHER_PREFERENCE_MUTATION_PATH_INVALID",
        );
    };
    let marker = state_root.join(WATCHER_DISABLED_MARKER);

    let initial = match observe_context(parent, state_root, &marker, file_system) {
        Ok(observation) => observation,
        Err(error) => {
            return failed_receipt(
                request,
                WatcherPreferenceSetupCommit::NotRequired,
                WatcherPreferenceMarkerCommit::NotAttempted,
                error.code,
            );
        }
    };
    let initial_final = initial.final_observation();

    if control.phase() == WatcherPreferenceMutationPhase::Cancelled {
        return cancelled_receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            initial_final,
        );
    }

    let actual = initial.preference();
    if actual == request.desired() {
        return receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            WatcherPreferenceMarkerCommit::NotAttempted,
            initial_final,
            WatcherPreferenceMutationOutcome::AlreadySatisfied,
            "WATCHER_PREFERENCE_MUTATION_ALREADY_SATISFIED",
        );
    }
    if actual != request.expected() {
        return receipt(
            request,
            WatcherPreferenceSetupCommit::NotRequired,
            WatcherPreferenceMarkerCommit::NotAttempted,
            initial_final,
            WatcherPreferenceMutationOutcome::Conflict,
            "WATCHER_PREFERENCE_MUTATION_CONFLICT",
        );
    }

    let mut setup_commit = WatcherPreferenceSetupCommit::NotRequired;
    if initial == ContextObservation::RootMissing {
        if !control.try_reach_commit() {
            return cancelled_receipt(request, setup_commit, initial_final);
        }

        setup_commit = match file_system.create_dir(state_root) {
            Ok(()) => WatcherPreferenceSetupCommit::RootCreated,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                WatcherPreferenceSetupCommit::NotRequired
            }
            Err(_) => {
                let final_observation = observe_context(parent, state_root, &marker, file_system);
                return finish_setup_failure(request, final_observation);
            }
        };

        let after_setup = match observe_context(parent, state_root, &marker, file_system) {
            Ok(observation) => observation,
            Err(_) => {
                return receipt(
                    request,
                    setup_commit,
                    WatcherPreferenceMarkerCommit::NotAttempted,
                    WatcherPreferenceFinalObservation::Unknown,
                    WatcherPreferenceMutationOutcome::Indeterminate,
                    "WATCHER_PREFERENCE_MUTATION_INDETERMINATE",
                );
            }
        };
        if after_setup == ContextObservation::RootMissing
            || after_setup.preference() != request.expected()
        {
            return receipt(
                request,
                setup_commit,
                WatcherPreferenceMarkerCommit::NotAttempted,
                after_setup.final_observation(),
                WatcherPreferenceMutationOutcome::Conflict,
                "WATCHER_PREFERENCE_MUTATION_CONFLICT",
            );
        }
    }

    if !control.try_reach_commit() {
        return cancelled_receipt(request, setup_commit, initial_final);
    }

    let operation = match request.desired() {
        WatcherPreference::ExplicitlyDisabled => file_system.create_marker_new(&marker),
        WatcherPreference::EnabledByDefault => file_system.remove_marker(&marker),
    };
    let (marker_commit, operation_succeeded) = match operation {
        Ok(()) => (
            match request.desired() {
                WatcherPreference::ExplicitlyDisabled => WatcherPreferenceMarkerCommit::Created,
                WatcherPreference::EnabledByDefault => WatcherPreferenceMarkerCommit::Removed,
            },
            true,
        ),
        Err(_) => (WatcherPreferenceMarkerCommit::Failed, false),
    };

    finish_marker_operation(
        request,
        setup_commit,
        marker_commit,
        operation_succeeded,
        observe_context(parent, state_root, &marker, file_system),
    )
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn observe_context(
    parent: &Path,
    state_root: &Path,
    marker: &Path,
    file_system: &impl WatcherPreferenceFileSystem,
) -> Result<ContextObservation, ContextObservationError> {
    match file_system.symlink_metadata(parent) {
        Ok(WatcherPreferenceEntryKind::Directory) => {}
        Ok(
            WatcherPreferenceEntryKind::File
            | WatcherPreferenceEntryKind::LinkOrReparse
            | WatcherPreferenceEntryKind::Other,
        ) => {
            return Err(ContextObservationError {
                code: "WATCHER_PREFERENCE_MUTATION_INVALID_PARENT",
            });
        }
        Err(_) => {
            return Err(ContextObservationError {
                code: "WATCHER_PREFERENCE_MUTATION_PARENT_UNAVAILABLE",
            });
        }
    }

    match file_system.symlink_metadata(state_root) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(ContextObservation::RootMissing);
        }
        Ok(WatcherPreferenceEntryKind::Directory) => {}
        Ok(
            WatcherPreferenceEntryKind::File
            | WatcherPreferenceEntryKind::LinkOrReparse
            | WatcherPreferenceEntryKind::Other,
        ) => {
            return Err(ContextObservationError {
                code: "WATCHER_PREFERENCE_MUTATION_INVALID_STATE_ROOT",
            });
        }
        Err(_) => {
            return Err(ContextObservationError {
                code: "WATCHER_PREFERENCE_MUTATION_STATE_ROOT_UNAVAILABLE",
            });
        }
    }

    match file_system.symlink_metadata(marker) {
        Ok(WatcherPreferenceEntryKind::File) => Ok(ContextObservation::Preference(
            WatcherPreference::ExplicitlyDisabled,
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(
            ContextObservation::Preference(WatcherPreference::EnabledByDefault),
        ),
        Ok(
            WatcherPreferenceEntryKind::Directory
            | WatcherPreferenceEntryKind::LinkOrReparse
            | WatcherPreferenceEntryKind::Other,
        ) => Err(ContextObservationError {
            code: "WATCHER_PREFERENCE_MUTATION_INVALID_MARKER",
        }),
        Err(_) => Err(ContextObservationError {
            code: "WATCHER_PREFERENCE_MUTATION_MARKER_UNAVAILABLE",
        }),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn finish_setup_failure(
    request: &WatcherPreferenceMutationRequest,
    final_observation: Result<ContextObservation, ContextObservationError>,
) -> WatcherPreferenceMutationReceipt {
    match final_observation {
        Ok(observation) if observation.preference() == request.desired() => receipt(
            request,
            WatcherPreferenceSetupCommit::Failed,
            WatcherPreferenceMarkerCommit::NotAttempted,
            observation.final_observation(),
            WatcherPreferenceMutationOutcome::Conflict,
            "WATCHER_PREFERENCE_MUTATION_CONFLICT",
        ),
        Ok(observation) => receipt(
            request,
            WatcherPreferenceSetupCommit::Failed,
            WatcherPreferenceMarkerCommit::NotAttempted,
            observation.final_observation(),
            WatcherPreferenceMutationOutcome::Failed,
            "WATCHER_PREFERENCE_MUTATION_SETUP_FAILED",
        ),
        Err(_) => receipt(
            request,
            WatcherPreferenceSetupCommit::Failed,
            WatcherPreferenceMarkerCommit::NotAttempted,
            WatcherPreferenceFinalObservation::Unknown,
            WatcherPreferenceMutationOutcome::Indeterminate,
            "WATCHER_PREFERENCE_MUTATION_INDETERMINATE",
        ),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", test))]
fn finish_marker_operation(
    request: &WatcherPreferenceMutationRequest,
    setup_commit: WatcherPreferenceSetupCommit,
    marker_commit: WatcherPreferenceMarkerCommit,
    operation_succeeded: bool,
    final_observation: Result<ContextObservation, ContextObservationError>,
) -> WatcherPreferenceMutationReceipt {
    let Ok(observation) = final_observation else {
        return receipt(
            request,
            setup_commit,
            marker_commit,
            WatcherPreferenceFinalObservation::Unknown,
            WatcherPreferenceMutationOutcome::Indeterminate,
            "WATCHER_PREFERENCE_MUTATION_INDETERMINATE",
        );
    };
    let final_observation = observation.final_observation();
    let final_is_desired = observation.preference() == request.desired();

    match (operation_succeeded, final_is_desired) {
        (true, true) => receipt(
            request,
            setup_commit,
            marker_commit,
            final_observation,
            WatcherPreferenceMutationOutcome::Applied,
            "WATCHER_PREFERENCE_MUTATION_APPLIED",
        ),
        (true, false) | (false, true) => receipt(
            request,
            setup_commit,
            marker_commit,
            final_observation,
            WatcherPreferenceMutationOutcome::Conflict,
            "WATCHER_PREFERENCE_MUTATION_CONFLICT",
        ),
        (false, false) => receipt(
            request,
            setup_commit,
            marker_commit,
            final_observation,
            WatcherPreferenceMutationOutcome::Failed,
            "WATCHER_PREFERENCE_MUTATION_MARKER_FAILED",
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
        io,
        path::{Path, PathBuf},
    };

    use inputcodex_application::{
        WatcherPreferenceMutationControl, WatcherPreferenceMutationRequest,
    };
    use inputcodex_domain::{
        WatcherPreference, WatcherPreferenceFinalObservation, WatcherPreferenceMarkerCommit,
        WatcherPreferenceMutationId, WatcherPreferenceMutationOutcome,
        WatcherPreferenceSetupCommit,
    };

    use super::{
        WatcherPreferenceEntryKind, WatcherPreferenceFileSystem,
        mutate_watcher_preference_at_state_root,
    };

    #[derive(Debug, Clone, Copy)]
    enum MetadataResult {
        Kind(WatcherPreferenceEntryKind),
        Error(io::ErrorKind),
    }

    #[derive(Debug)]
    enum Step {
        Metadata(PathBuf, MetadataResult),
        CreateDir(PathBuf, Option<io::ErrorKind>),
        CreateMarker(PathBuf, Option<io::ErrorKind>),
        RemoveMarker(PathBuf, Option<io::ErrorKind>),
    }

    struct ScriptedFileSystem {
        steps: RefCell<VecDeque<Step>>,
        metadata_calls: Cell<usize>,
        cancel_on_metadata_call: Option<(usize, WatcherPreferenceMutationControl)>,
    }

    impl ScriptedFileSystem {
        fn new(steps: Vec<Step>) -> Self {
            Self {
                steps: RefCell::new(steps.into()),
                metadata_calls: Cell::new(0),
                cancel_on_metadata_call: None,
            }
        }

        fn cancelling_on_metadata_call(
            mut self,
            call: usize,
            control: WatcherPreferenceMutationControl,
        ) -> Self {
            self.cancel_on_metadata_call = Some((call, control));
            self
        }

        fn assert_done(&self) {
            assert!(
                self.steps.borrow().is_empty(),
                "仍有未消费文件系统步骤: {:?}",
                self.steps.borrow()
            );
        }

        fn pop(&self) -> Step {
            self.steps
                .borrow_mut()
                .pop_front()
                .expect("文件系统调用超出脚本")
        }
    }

    impl WatcherPreferenceFileSystem for ScriptedFileSystem {
        fn symlink_metadata(&self, path: &Path) -> io::Result<WatcherPreferenceEntryKind> {
            let Step::Metadata(expected, result) = self.pop() else {
                panic!("预期 metadata 调用");
            };
            assert_eq!(path, expected);
            let call = self.metadata_calls.get() + 1;
            self.metadata_calls.set(call);
            if let Some((cancel_at, control)) = &self.cancel_on_metadata_call
                && call == *cancel_at
            {
                let _ = control.cancel();
            }
            match result {
                MetadataResult::Kind(kind) => Ok(kind),
                MetadataResult::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn create_dir(&self, path: &Path) -> io::Result<()> {
            let Step::CreateDir(expected, error) = self.pop() else {
                panic!("预期 create_dir 调用");
            };
            assert_eq!(path, expected);
            error.map_or(Ok(()), |kind| Err(io::Error::from(kind)))
        }

        fn create_marker_new(&self, path: &Path) -> io::Result<()> {
            let Step::CreateMarker(expected, error) = self.pop() else {
                panic!("预期 create_marker_new 调用");
            };
            assert_eq!(path, expected);
            error.map_or(Ok(()), |kind| Err(io::Error::from(kind)))
        }

        fn remove_marker(&self, path: &Path) -> io::Result<()> {
            let Step::RemoveMarker(expected, error) = self.pop() else {
                panic!("预期 remove_marker 调用");
            };
            assert_eq!(path, expected);
            error.map_or(Ok(()), |kind| Err(io::Error::from(kind)))
        }
    }

    fn paths() -> (PathBuf, PathBuf, PathBuf) {
        let parent = std::env::temp_dir().join("inputcodex-approved-parent");
        let root = parent.join("inputcodex");
        let marker = root.join("watcher.disabled");
        (parent, root, marker)
    }

    fn request(
        expected: WatcherPreference,
        desired: WatcherPreference,
    ) -> WatcherPreferenceMutationRequest {
        WatcherPreferenceMutationRequest::new(
            WatcherPreferenceMutationId::new(143),
            expected,
            desired,
        )
    }

    fn metadata(path: &Path, result: MetadataResult) -> Step {
        Step::Metadata(path.to_path_buf(), result)
    }

    #[test]
    fn 预先取消时不触碰文件系统() {
        let (_, root, _) = paths();
        let fs = ScriptedFileSystem::new(vec![]);
        let control = WatcherPreferenceMutationControl::new();
        let _ = control.cancel();

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &control,
            &fs,
        );

        assert_eq!(
            receipt.outcome(),
            WatcherPreferenceMutationOutcome::Cancelled
        );
        fs.assert_done();
    }

    #[test]
    fn 父目录缺失或链接均在提交前失败() {
        let (parent, root, _) = paths();
        for (result, code) in [
            (
                MetadataResult::Error(io::ErrorKind::NotFound),
                "WATCHER_PREFERENCE_MUTATION_PARENT_UNAVAILABLE",
            ),
            (
                MetadataResult::Kind(WatcherPreferenceEntryKind::LinkOrReparse),
                "WATCHER_PREFERENCE_MUTATION_INVALID_PARENT",
            ),
        ] {
            let fs = ScriptedFileSystem::new(vec![metadata(&parent, result)]);
            let receipt = mutate_watcher_preference_at_state_root(
                &root,
                &request(
                    WatcherPreference::EnabledByDefault,
                    WatcherPreference::ExplicitlyDisabled,
                ),
                &WatcherPreferenceMutationControl::new(),
                &fs,
            );

            assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
            assert_eq!(receipt.diagnostic_code().as_str(), code);
            fs.assert_done();
        }
    }

    #[test]
    fn 状态根缺失时启用已经满足且不创建目录() {
        let (parent, root, _) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::ExplicitlyDisabled,
                WatcherPreference::EnabledByDefault,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(
            receipt.outcome(),
            WatcherPreferenceMutationOutcome::AlreadySatisfied
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault)
        );
        fs.assert_done();
    }

    #[test]
    fn 首次禁用只单层创建根并以_create_new_创建固定标记() {
        let (parent, root, marker) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateDir(root.clone(), None),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateMarker(marker.clone(), None),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &marker,
                MetadataResult::Kind(WatcherPreferenceEntryKind::File),
            ),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Applied);
        assert_eq!(
            receipt.setup_commit(),
            WatcherPreferenceSetupCommit::RootCreated
        );
        assert_eq!(
            receipt.marker_commit(),
            WatcherPreferenceMarkerCommit::Created
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Known(WatcherPreference::ExplicitlyDisabled)
        );
        fs.assert_done();
    }

    #[test]
    fn 根已创建但标记失败保留空根并交付完整收据() {
        let (parent, root, marker) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateDir(root.clone(), None),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateMarker(marker.clone(), Some(io::ErrorKind::PermissionDenied)),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
        assert_eq!(
            receipt.setup_commit(),
            WatcherPreferenceSetupCommit::RootCreated
        );
        assert_eq!(
            receipt.marker_commit(),
            WatcherPreferenceMarkerCommit::Failed
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault)
        );
        fs.assert_done();
    }

    #[test]
    fn 状态根创建失败后复观测并交付_setup_failed() {
        let (parent, root, _) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateDir(root.clone(), Some(io::ErrorKind::PermissionDenied)),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
        assert_eq!(receipt.setup_commit(), WatcherPreferenceSetupCommit::Failed);
        assert_eq!(
            receipt.marker_commit(),
            WatcherPreferenceMarkerCommit::NotAttempted
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault)
        );
        assert_eq!(
            receipt.diagnostic_code().as_str(),
            "WATCHER_PREFERENCE_MUTATION_SETUP_FAILED"
        );
        fs.assert_done();
    }

    #[test]
    fn 已存在根上的禁用和启用分别创建与删除标记() {
        let (parent, root, marker) = paths();
        for (initial, desired, operation, final_result, marker_commit) in [
            (
                MetadataResult::Error(io::ErrorKind::NotFound),
                WatcherPreference::ExplicitlyDisabled,
                Step::CreateMarker(marker.clone(), None),
                MetadataResult::Kind(WatcherPreferenceEntryKind::File),
                WatcherPreferenceMarkerCommit::Created,
            ),
            (
                MetadataResult::Kind(WatcherPreferenceEntryKind::File),
                WatcherPreference::EnabledByDefault,
                Step::RemoveMarker(marker.clone(), None),
                MetadataResult::Error(io::ErrorKind::NotFound),
                WatcherPreferenceMarkerCommit::Removed,
            ),
        ] {
            let expected = if desired == WatcherPreference::ExplicitlyDisabled {
                WatcherPreference::EnabledByDefault
            } else {
                WatcherPreference::ExplicitlyDisabled
            };
            let fs = ScriptedFileSystem::new(vec![
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(&marker, initial),
                operation,
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(&marker, final_result),
            ]);

            let receipt = mutate_watcher_preference_at_state_root(
                &root,
                &request(expected, desired),
                &WatcherPreferenceMutationControl::new(),
                &fs,
            );

            assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Applied);
            assert_eq!(receipt.marker_commit(), marker_commit);
            assert_eq!(
                receipt.final_observation(),
                WatcherPreferenceFinalObservation::Known(desired)
            );
            fs.assert_done();
        }
    }

    #[test]
    fn 已满足和预期冲突均不写入() {
        let (parent, root, marker) = paths();
        for (actual, request, outcome) in [
            (
                MetadataResult::Kind(WatcherPreferenceEntryKind::File),
                request(
                    WatcherPreference::EnabledByDefault,
                    WatcherPreference::ExplicitlyDisabled,
                ),
                WatcherPreferenceMutationOutcome::AlreadySatisfied,
            ),
            (
                MetadataResult::Error(io::ErrorKind::NotFound),
                request(
                    WatcherPreference::ExplicitlyDisabled,
                    WatcherPreference::ExplicitlyDisabled,
                ),
                WatcherPreferenceMutationOutcome::Conflict,
            ),
        ] {
            let fs = ScriptedFileSystem::new(vec![
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(&marker, actual),
            ]);

            let receipt = mutate_watcher_preference_at_state_root(
                &root,
                &request,
                &WatcherPreferenceMutationControl::new(),
                &fs,
            );

            assert_eq!(receipt.outcome(), outcome);
            assert_eq!(
                receipt.marker_commit(),
                WatcherPreferenceMarkerCommit::NotAttempted
            );
            fs.assert_done();
        }
    }

    #[test]
    fn 根或标记为链接时_fail_closed() {
        let (parent, root, marker) = paths();
        for steps in [
            vec![
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::LinkOrReparse),
                ),
            ],
            vec![
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &marker,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::LinkOrReparse),
                ),
            ],
        ] {
            let fs = ScriptedFileSystem::new(steps);
            let receipt = mutate_watcher_preference_at_state_root(
                &root,
                &request(
                    WatcherPreference::EnabledByDefault,
                    WatcherPreference::ExplicitlyDisabled,
                ),
                &WatcherPreferenceMutationControl::new(),
                &fs,
            );

            assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Failed);
            assert_eq!(
                receipt.final_observation(),
                WatcherPreferenceFinalObservation::Unknown
            );
            fs.assert_done();
        }
    }

    #[test]
    fn 最后一次元数据观察触发取消时不进入提交() {
        let (parent, root, marker) = paths();
        let control = WatcherPreferenceMutationControl::new();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
        ])
        .cancelling_on_metadata_call(3, control.clone());

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &control,
            &fs,
        );

        assert_eq!(
            receipt.outcome(),
            WatcherPreferenceMutationOutcome::Cancelled
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Known(WatcherPreference::EnabledByDefault)
        );
        fs.assert_done();
    }

    #[test]
    fn 提交后相反合法状态与不可读状态分别为_conflict_和_indeterminate() {
        let (parent, root, marker) = paths();
        for (final_result, outcome) in [
            (
                MetadataResult::Error(io::ErrorKind::NotFound),
                WatcherPreferenceMutationOutcome::Conflict,
            ),
            (
                MetadataResult::Error(io::ErrorKind::PermissionDenied),
                WatcherPreferenceMutationOutcome::Indeterminate,
            ),
        ] {
            let fs = ScriptedFileSystem::new(vec![
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
                Step::CreateMarker(marker.clone(), None),
                metadata(
                    &parent,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(
                    &root,
                    MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
                ),
                metadata(&marker, final_result),
            ]);

            let receipt = mutate_watcher_preference_at_state_root(
                &root,
                &request(
                    WatcherPreference::EnabledByDefault,
                    WatcherPreference::ExplicitlyDisabled,
                ),
                &WatcherPreferenceMutationControl::new(),
                &fs,
            );

            assert_eq!(receipt.outcome(), outcome);
            fs.assert_done();
        }
    }

    #[test]
    fn 并发创建状态根的_already_exists_必须复观测后继续() {
        let (parent, root, marker) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateDir(root.clone(), Some(io::ErrorKind::AlreadyExists)),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&marker, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateMarker(marker.clone(), None),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &marker,
                MetadataResult::Kind(WatcherPreferenceEntryKind::File),
            ),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(receipt.outcome(), WatcherPreferenceMutationOutcome::Applied);
        assert_eq!(
            receipt.setup_commit(),
            WatcherPreferenceSetupCommit::NotRequired
        );
        fs.assert_done();
    }

    #[test]
    fn already_exists_后状态根为_reparse_必须停止() {
        let (parent, root, _) = paths();
        let fs = ScriptedFileSystem::new(vec![
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(&root, MetadataResult::Error(io::ErrorKind::NotFound)),
            Step::CreateDir(root.clone(), Some(io::ErrorKind::AlreadyExists)),
            metadata(
                &parent,
                MetadataResult::Kind(WatcherPreferenceEntryKind::Directory),
            ),
            metadata(
                &root,
                MetadataResult::Kind(WatcherPreferenceEntryKind::LinkOrReparse),
            ),
        ]);

        let receipt = mutate_watcher_preference_at_state_root(
            &root,
            &request(
                WatcherPreference::EnabledByDefault,
                WatcherPreference::ExplicitlyDisabled,
            ),
            &WatcherPreferenceMutationControl::new(),
            &fs,
        );

        assert_eq!(
            receipt.outcome(),
            WatcherPreferenceMutationOutcome::Indeterminate
        );
        assert_eq!(
            receipt.setup_commit(),
            WatcherPreferenceSetupCommit::NotRequired
        );
        assert_eq!(
            receipt.marker_commit(),
            WatcherPreferenceMarkerCommit::NotAttempted
        );
        assert_eq!(
            receipt.final_observation(),
            WatcherPreferenceFinalObservation::Unknown
        );
        fs.assert_done();
    }
}
