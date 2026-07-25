#![forbid(unsafe_code)]

use inputcodex_application::{
    LoadCompletion, LoadCoordinator, LoadState, RequestId, TransitionOutcome,
};
use inputcodex_parity::validate_repository;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub const APPLICATION_LOAD_COMPLETE: &str = "application-load-complete";
pub const APPLICATION_CANCEL_STALE: &str = "application-cancel-stale";
pub const PARITY_REPOSITORY_VALIDATION: &str = "parity-repository-validation";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineError {
    UnknownScenario(String),
    ZeroIterations,
    InvalidRepositoryRoot(PathBuf),
    RepositoryValidation(String),
}

impl Display for BaselineError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownScenario(name) => write!(formatter, "未知性能场景：{name}"),
            Self::ZeroIterations => formatter.write_str("性能场景迭代数必须大于零"),
            Self::InvalidRepositoryRoot(path) => {
                write!(
                    formatter,
                    "无效的 inputcodex 仓库根目录：{}",
                    path.display()
                )
            }
            Self::RepositoryValidation(message) => {
                write!(formatter, "Parity 仓库验证失败：{message}")
            }
        }
    }
}

impl Error for BaselineError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioMeasurement {
    name: &'static str,
    iterations: u64,
    total_nanoseconds: u128,
    nanoseconds_per_operation: f64,
    checksum: u64,
}

impl ScenarioMeasurement {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn iterations(&self) -> u64 {
        self.iterations
    }

    #[must_use]
    pub const fn total_nanoseconds(&self) -> u128 {
        self.total_nanoseconds
    }

    #[must_use]
    pub const fn nanoseconds_per_operation(&self) -> f64 {
        self.nanoseconds_per_operation
    }

    #[must_use]
    pub const fn checksum(&self) -> u64 {
        self.checksum
    }

    #[must_use]
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{:.6},{}",
            self.name,
            self.iterations,
            self.total_nanoseconds,
            self.nanoseconds_per_operation,
            self.checksum
        )
    }
}

pub fn run_scenario(
    name: &str,
    repository_root: &Path,
    iterations: u64,
) -> Result<ScenarioMeasurement, BaselineError> {
    if iterations == 0 {
        return Err(BaselineError::ZeroIterations);
    }

    match name {
        APPLICATION_LOAD_COMPLETE => Ok(measure_application_load_complete(iterations)),
        APPLICATION_CANCEL_STALE => Ok(measure_application_cancel_stale(iterations)),
        PARITY_REPOSITORY_VALIDATION => {
            if !repository_root.is_dir() || !repository_root.join("parity").is_dir() {
                return Err(BaselineError::InvalidRepositoryRoot(
                    repository_root.to_path_buf(),
                ));
            }
            measure_parity_repository_validation(repository_root, iterations)
        }
        _ => Err(BaselineError::UnknownScenario(name.to_owned())),
    }
}

fn measure_application_load_complete(iterations: u64) -> ScenarioMeasurement {
    let mut coordinator = LoadCoordinator::<u64>::default();
    let mut checksum = checksum_seed();
    let started = Instant::now();

    for index in 0..iterations {
        let request_id = RequestId::new(index + 1);
        coordinator.begin(request_id);
        let outcome = coordinator.complete(request_id, LoadCompletion::Ready(index));
        let state_value = match coordinator.state() {
            LoadState::Ready { request_id, value } => request_id.value().wrapping_add(*value),
            _ => 0,
        };
        checksum = mix_checksum(checksum, state_value ^ outcome_value(outcome));
        black_box(coordinator.state());
    }

    finish_measurement(APPLICATION_LOAD_COMPLETE, iterations, started, checksum)
}

fn measure_application_cancel_stale(iterations: u64) -> ScenarioMeasurement {
    let mut coordinator = LoadCoordinator::<()>::default();
    let mut checksum = checksum_seed();
    let started = Instant::now();

    for index in 0..iterations {
        let request_id = RequestId::new(index.saturating_add(1));
        coordinator.begin(request_id);
        let cancellation_outcome = coordinator.cancel(request_id);
        let stale_completion_outcome = coordinator.complete(request_id, LoadCompletion::Ready(()));
        let state_value = match coordinator.state() {
            LoadState::Cancelling { request_id } => request_id.value(),
            _ => 0,
        };
        let sample = state_value
            ^ (outcome_value(cancellation_outcome) << 8)
            ^ (outcome_value(stale_completion_outcome) << 16);
        checksum = mix_checksum(checksum, sample);
        black_box(coordinator.state());
    }

    finish_measurement(APPLICATION_CANCEL_STALE, iterations, started, checksum)
}

fn measure_parity_repository_validation(
    repository_root: &Path,
    iterations: u64,
) -> Result<ScenarioMeasurement, BaselineError> {
    let mut checksum = checksum_seed();
    let started = Instant::now();

    for _ in 0..iterations {
        let summary = validate_repository(repository_root)
            .map_err(|error| BaselineError::RepositoryValidation(error.to_string()))?;
        let counts = [
            summary.source_entry_count(),
            summary.feature_count(),
            summary.contract_count(),
            summary.fixture_count(),
            summary.excluded_entry_count(),
            summary.exception_pending_count(),
            summary.coverage_gap_count(),
            usize::from(summary.requires_reaudit()),
        ];
        for count in counts {
            checksum = mix_checksum(checksum, count as u64);
        }
        black_box(summary);
    }

    Ok(finish_measurement(
        PARITY_REPOSITORY_VALIDATION,
        iterations,
        started,
        checksum,
    ))
}

fn finish_measurement(
    name: &'static str,
    iterations: u64,
    started: Instant,
    checksum: u64,
) -> ScenarioMeasurement {
    let total_nanoseconds = started.elapsed().as_nanos();
    ScenarioMeasurement {
        name,
        iterations,
        total_nanoseconds,
        nanoseconds_per_operation: calculate_nanoseconds_per_operation(
            total_nanoseconds,
            iterations,
        ),
        checksum,
    }
}

fn calculate_nanoseconds_per_operation(total_nanoseconds: u128, iterations: u64) -> f64 {
    total_nanoseconds as f64 / iterations as f64
}

const fn checksum_seed() -> u64 {
    0xcbf2_9ce4_8422_2325
}

const fn mix_checksum(current: u64, value: u64) -> u64 {
    current.rotate_left(7) ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15)
}

const fn outcome_value(outcome: TransitionOutcome) -> u64 {
    match outcome {
        TransitionOutcome::Applied => 1,
        TransitionOutcome::Stale => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::{ScenarioMeasurement, calculate_nanoseconds_per_operation};

    #[test]
    fn per_operation_time_preserves_fractional_nanoseconds() {
        assert_eq!(calculate_nanoseconds_per_operation(1, 2), 0.5);

        let measurement = ScenarioMeasurement {
            name: "fractional",
            iterations: 2,
            total_nanoseconds: 1,
            nanoseconds_per_operation: 0.5,
            checksum: 1,
        };
        assert_eq!(measurement.to_csv(), "fractional,2,1,0.500000,1");
    }
}
