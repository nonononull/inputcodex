use inputcodex_domain::{
    EnvironmentConflictSource, EnvironmentObservationStatus, EnvironmentSourceCoverage,
    EnvironmentValuePresence, EnvironmentVariableName, EnvironmentVariableNameError,
    RuntimeEnvironmentConflict, RuntimeEnvironmentConflictObservation,
};

fn conflict(name: &str, presence: EnvironmentValuePresence) -> RuntimeEnvironmentConflict {
    RuntimeEnvironmentConflict::runtime_process(
        EnvironmentVariableName::new(name.to_owned()).expect("测试变量名必须合法"),
        presence,
    )
}

#[test]
fn 观察结果排序去重且非空值优先() {
    let observation = RuntimeEnvironmentConflictObservation::new(
        7,
        vec![
            conflict("OPENAI_ZETA", EnvironmentValuePresence::Empty),
            conflict("OPENAI_API_KEY", EnvironmentValuePresence::Empty),
            conflict("OPENAI_API_KEY", EnvironmentValuePresence::NonEmpty),
        ],
    );

    assert_eq!(observation.scanned_entry_count(), 7);
    assert_eq!(observation.conflict_count(), 2);
    assert_eq!(observation.conflicts().len(), 2);
    assert_eq!(observation.conflicts()[0].name().as_str(), "OPENAI_API_KEY");
    assert_eq!(
        observation.conflicts()[0].value_presence(),
        EnvironmentValuePresence::NonEmpty
    );
    assert_eq!(observation.conflicts()[1].name().as_str(), "OPENAI_ZETA");
}

#[test]
fn 覆盖状态明确只有当前进程已观察() {
    let observation = RuntimeEnvironmentConflictObservation::new(
        1,
        vec![conflict(
            "OPENAI_BASE_URL",
            EnvironmentValuePresence::NonEmpty,
        )],
    );

    assert_eq!(
        observation.coverage(),
        &EnvironmentSourceCoverage::runtime_only()
    );
    assert_eq!(
        observation.coverage().runtime_process(),
        EnvironmentObservationStatus::Observed
    );
    assert_eq!(
        observation.coverage().persistent_user(),
        EnvironmentObservationStatus::NotObserved
    );
    assert_eq!(
        observation.coverage().persistent_system(),
        EnvironmentObservationStatus::NotObserved
    );
    assert_eq!(
        observation.conflicts()[0].source(),
        EnvironmentConflictSource::RuntimeProcess
    );
}

#[test]
fn 零冲突仍是完整观察结果() {
    let observation = RuntimeEnvironmentConflictObservation::new(3, Vec::new());

    assert_eq!(observation.scanned_entry_count(), 3);
    assert_eq!(observation.conflict_count(), 0);
    assert!(observation.conflicts().is_empty());
    assert_eq!(
        observation.coverage(),
        &EnvironmentSourceCoverage::runtime_only()
    );
}

#[test]
fn 领域变量名只接受真实_openai_前缀且不修剪() {
    assert_eq!(
        EnvironmentVariableName::new("CUSTOM_OPENAI_API_KEY".to_owned()),
        Err(EnvironmentVariableNameError::InvalidPrefix)
    );
    assert_eq!(
        EnvironmentVariableName::new(" OPENAI_API_KEY".to_owned()),
        Err(EnvironmentVariableNameError::InvalidPrefix)
    );
    assert_eq!(
        EnvironmentVariableName::new("openai_api_key".to_owned()),
        Err(EnvironmentVariableNameError::InvalidPrefix)
    );

    let name =
        EnvironmentVariableName::new("OPENAI_API_KEY".to_owned()).expect("真实前缀应通过领域校验");
    assert_eq!(name.as_str(), "OPENAI_API_KEY");
}
