use inputcodex_domain::{
    MAX_ZED_REMOTE_PROJECT_SOURCES, MAX_ZED_REMOTE_PROJECTS, ZedRemoteProjectEntry,
    ZedRemoteProjectId, ZedRemoteProjectIdError, ZedRemoteProjectObservation,
    ZedRemoteProjectObservationError, ZedRemoteProjectOrigin, ZedRemoteProjectSelectionHint,
    ZedRemoteProjectSourceCoverage, ZedRemoteProjectSourceSummary,
    ZedRemoteProjectSourceSummaryError,
};

const VALID_ID: &str =
    "zed-remote-project:v1:sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn project(suffix: char, origin: ZedRemoteProjectOrigin) -> ZedRemoteProjectEntry {
    let digest = suffix.to_string().repeat(64);
    let id = ZedRemoteProjectId::new(format!("zed-remote-project:v1:sha256:{digest}"))
        .expect("测试稳定假名应合法");
    ZedRemoteProjectEntry::new(id, origin, ZedRemoteProjectSelectionHint::NotObserved)
}

#[test]
fn 稳定假名只接受固定前缀与六十四位小写十六进制摘要() {
    let id = ZedRemoteProjectId::new(VALID_ID.to_string()).expect("固定格式应合法");
    assert_eq!(id.as_str(), VALID_ID);

    for invalid in [
        "",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "zed-remote-project:v2:sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "zed-remote-project:v1:sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde",
        "zed-remote-project:v1:sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeF",
        "zed-remote-project:v1:sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdeg",
    ] {
        assert_eq!(
            ZedRemoteProjectId::new(invalid.to_string()),
            Err(ZedRemoteProjectIdError::InvalidFormat),
            "非法稳定假名应被拒绝: {invalid}"
        );
    }
}

#[test]
fn 项目条目只保留三种来源与两种选择提示() {
    let origins = [
        ZedRemoteProjectOrigin::CodexRemoteProject,
        ZedRemoteProjectOrigin::ThreadWorkspaceHint,
        ZedRemoteProjectOrigin::SqliteThreadCwd,
    ];
    let hints = [
        ZedRemoteProjectSelectionHint::SelectedHostHint,
        ZedRemoteProjectSelectionHint::NotObserved,
    ];

    for (index, origin) in origins.into_iter().enumerate() {
        for hint in hints {
            let digest = format!("{index:x}").repeat(64);
            let id = ZedRemoteProjectId::new(format!("zed-remote-project:v1:sha256:{digest}"))
                .expect("测试稳定假名应合法");
            let entry = ZedRemoteProjectEntry::new(id, origin, hint);

            assert_eq!(entry.origin(), origin);
            assert_eq!(entry.selection_hint(), hint);
            assert!(
                entry
                    .id()
                    .as_str()
                    .starts_with("zed-remote-project:v1:sha256:")
            );
        }
    }
}

#[test]
fn 来源摘要固定计数边界并区分完整与部分覆盖() {
    let complete = ZedRemoteProjectSourceSummary::new(2, 2, 0).expect("完整来源应合法");
    assert_eq!(complete.discovered(), 2);
    assert_eq!(complete.readable(), 2);
    assert_eq!(complete.failed(), 0);
    assert_eq!(
        complete.coverage(),
        ZedRemoteProjectSourceCoverage::Complete
    );

    let partial = ZedRemoteProjectSourceSummary::new(3, 2, 1).expect("部分来源应合法");
    assert_eq!(partial.coverage(), ZedRemoteProjectSourceCoverage::Partial);

    assert_eq!(
        ZedRemoteProjectSourceSummary::new(0, 0, 0),
        Err(ZedRemoteProjectSourceSummaryError::NoSources)
    );
    assert_eq!(
        ZedRemoteProjectSourceSummary::new(2, 1, 0),
        Err(ZedRemoteProjectSourceSummaryError::CountMismatch)
    );
    assert_eq!(
        ZedRemoteProjectSourceSummary::new(2, 0, 2),
        Err(ZedRemoteProjectSourceSummaryError::NoReadableSources)
    );
    assert_eq!(
        ZedRemoteProjectSourceSummary::new(
            MAX_ZED_REMOTE_PROJECT_SOURCES + 1,
            1,
            MAX_ZED_REMOTE_PROJECT_SOURCES
        ),
        Err(ZedRemoteProjectSourceSummaryError::TooManySources)
    );
}

#[test]
fn 观察结果必须非空有界且不接受重复稳定假名() {
    let sources = ZedRemoteProjectSourceSummary::new(2, 2, 0).expect("来源应合法");
    let entry = project('a', ZedRemoteProjectOrigin::CodexRemoteProject);
    let observation =
        ZedRemoteProjectObservation::new(vec![entry.clone()], sources).expect("观察应合法");

    assert_eq!(observation.project_count(), 1);
    assert_eq!(observation.projects(), std::slice::from_ref(&entry));
    assert_eq!(observation.sources(), sources);

    assert_eq!(
        ZedRemoteProjectObservation::new(Vec::new(), sources),
        Err(ZedRemoteProjectObservationError::EmptyProjects)
    );
    assert_eq!(
        ZedRemoteProjectObservation::new(vec![entry.clone(), entry], sources),
        Err(ZedRemoteProjectObservationError::DuplicateProjectId)
    );

    let too_many = (0..=MAX_ZED_REMOTE_PROJECTS)
        .map(|index| {
            let digest = format!("{index:064x}");
            let id = ZedRemoteProjectId::new(format!("zed-remote-project:v1:sha256:{digest}"))
                .expect("测试稳定假名应合法");
            ZedRemoteProjectEntry::new(
                id,
                ZedRemoteProjectOrigin::SqliteThreadCwd,
                ZedRemoteProjectSelectionHint::NotObserved,
            )
        })
        .collect();
    assert_eq!(
        ZedRemoteProjectObservation::new(too_many, sources),
        Err(ZedRemoteProjectObservationError::TooManyProjects)
    );
}

#[test]
fn 调试输出不得记录稳定假名或任何远程身份材料() {
    let id = ZedRemoteProjectId::new(VALID_ID.to_string()).expect("固定格式应合法");
    let entry = ZedRemoteProjectEntry::new(
        id,
        ZedRemoteProjectOrigin::ThreadWorkspaceHint,
        ZedRemoteProjectSelectionHint::SelectedHostHint,
    );
    let sources = ZedRemoteProjectSourceSummary::new(1, 1, 0).expect("来源应合法");
    let observation = ZedRemoteProjectObservation::new(vec![entry], sources).expect("观察应合法");
    let debug = format!("{observation:?}");

    assert!(debug.contains("ThreadWorkspaceHint"));
    assert!(debug.contains("SelectedHostHint"));
    for forbidden in [
        VALID_ID,
        "0123456789abcdef",
        "alice",
        "example.internal",
        "/srv/private/repository",
        "ssh://",
        "hostId",
        "label",
        "timestamp",
    ] {
        assert!(
            !debug.contains(forbidden),
            "debug 泄漏禁止内容: {forbidden}"
        );
    }
}
