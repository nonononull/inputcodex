use inputcodex_domain::DiagnosticLogObservation;

#[test]
fn 诊断日志观察只公开有界结构事实() {
    let observation = DiagnosticLogObservation::new(4096, 2, 3, true, true);

    assert_eq!(observation.file_size_bytes(), 4096);
    assert_eq!(observation.sampled_record_count(), 5);
    assert_eq!(observation.valid_object_record_count(), 2);
    assert_eq!(observation.malformed_record_count(), 3);
    assert!(observation.truncated());
    assert!(observation.partial_record_discarded());
    assert_eq!(
        format!("{observation:?}"),
        "DiagnosticLogObservation { file_size_bytes: 4096, sampled_record_count: 5, valid_object_record_count: 2, malformed_record_count: 3, truncated: true, partial_record_discarded: true }"
    );
}

#[test]
fn 空日志仍是零计数_ready_事实() {
    let observation = DiagnosticLogObservation::new(0, 0, 0, false, false);

    assert_eq!(observation.file_size_bytes(), 0);
    assert_eq!(observation.sampled_record_count(), 0);
    assert_eq!(observation.valid_object_record_count(), 0);
    assert_eq!(observation.malformed_record_count(), 0);
    assert!(!observation.truncated());
    assert!(!observation.partial_record_discarded());
}

#[test]
fn 采样记录数由合法和损坏记录数派生() {
    let observation = DiagnosticLogObservation::new(128, 7, 4, false, false);

    assert_eq!(observation.sampled_record_count(), 11);
    assert_eq!(
        observation.sampled_record_count(),
        observation.valid_object_record_count() + observation.malformed_record_count()
    );
}
