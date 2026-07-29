use std::mem::size_of;

use inputcodex_application::ContextEntryObservationPort;
use inputcodex_platform::SystemContextEntryObservation;

fn assert_port<T: ContextEntryObservationPort>() {}

#[test]
fn 系统上下文条目观察保持零字段平台端口() {
    assert_eq!(size_of::<SystemContextEntryObservation>(), 0);
    assert_port::<SystemContextEntryObservation>();
    assert_eq!(
        format!("{:?}", SystemContextEntryObservation),
        "SystemContextEntryObservation"
    );
}
