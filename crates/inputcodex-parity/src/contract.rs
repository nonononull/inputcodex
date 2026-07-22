use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{ValidationCode, ValidationIssue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum LoadingState {
    Idle,
    Loading,
    Ready,
    Empty,
    Failed,
    Cancelling,
}

#[derive(Debug, Deserialize)]
pub struct ContractCatalog {
    #[serde(rename = "schema_version")]
    _schema_version: String,
    #[serde(rename = "domain")]
    _domain: String,
    contracts: Vec<BehaviorContract>,
}

impl ContractCatalog {
    #[must_use]
    pub fn contracts(&self) -> &[BehaviorContract] {
        &self.contracts
    }
}

#[derive(Debug, Deserialize)]
pub struct BehaviorContract {
    id: String,
    feature_id: String,
    scenario: String,
    #[serde(rename = "preconditions")]
    _preconditions: Vec<String>,
    #[serde(rename = "inputs")]
    _inputs: Vec<ContractInput>,
    #[serde(rename = "outputs")]
    _outputs: Vec<ContractOutput>,
    #[serde(rename = "side_effects")]
    _side_effects: Vec<String>,
    #[serde(rename = "persistence")]
    _persistence: String,
    #[serde(rename = "errors")]
    _errors: Vec<ContractError>,
    loading: LoadingContract,
    #[serde(rename = "timeout")]
    _timeout: BehaviorEvidence,
    #[serde(rename = "cancellation")]
    _cancellation: BehaviorEvidence,
    #[serde(rename = "isolation")]
    _isolation: String,
    #[serde(rename = "observability")]
    _observability: Vec<String>,
    fixture_refs: Vec<String>,
    #[serde(rename = "platforms")]
    _platforms: ContractPlatforms,
}

#[derive(Debug, Deserialize)]
struct ContractInput {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "data_type")]
    _data_type: String,
    #[serde(rename = "required")]
    _required: bool,
    #[serde(rename = "description")]
    _description: String,
}

#[derive(Debug, Deserialize)]
struct ContractOutput {
    #[serde(rename = "name")]
    _name: String,
    #[serde(rename = "data_type")]
    _data_type: String,
    #[serde(rename = "description")]
    _description: String,
}

#[derive(Debug, Deserialize)]
struct ContractError {
    #[serde(rename = "code")]
    _code: String,
    #[serde(rename = "kind")]
    _kind: String,
    #[serde(rename = "semantics")]
    _semantics: String,
}

#[derive(Debug, Deserialize)]
struct BehaviorEvidence {
    #[serde(rename = "behavior")]
    _behavior: String,
    #[serde(rename = "evidence")]
    _evidence: String,
}

#[derive(Debug, Deserialize)]
struct ContractPlatforms {
    #[serde(rename = "shared_semantics")]
    _shared_semantics: String,
    #[serde(rename = "windows")]
    _windows: String,
    #[serde(rename = "macos")]
    _macos: String,
    #[serde(rename = "differences")]
    _differences: Vec<String>,
}

impl BehaviorContract {
    #[must_use]
    pub const fn loading(&self) -> &LoadingContract {
        &self.loading
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadingContract {
    states: Vec<LoadingState>,
    #[serde(rename = "request_identity")]
    _request_identity: String,
    #[serde(rename = "stale_result_rule")]
    _stale_result_rule: String,
}

impl LoadingContract {
    #[must_use]
    pub fn states(&self) -> &[LoadingState] {
        &self.states
    }
}

pub fn parse_contract_catalog(input: &str) -> yaml_serde::Result<ContractCatalog> {
    yaml_serde::from_str(input)
}

#[must_use]
pub fn validate_contract_catalog(
    catalog: &ContractCatalog,
    feature_ids: &BTreeSet<String>,
    fixture_ids: &BTreeSet<String>,
) -> Vec<ValidationIssue> {
    const REQUIRED_STATES: [LoadingState; 6] = [
        LoadingState::Idle,
        LoadingState::Loading,
        LoadingState::Ready,
        LoadingState::Empty,
        LoadingState::Failed,
        LoadingState::Cancelling,
    ];

    let mut issues = Vec::new();
    let mut contract_ids = BTreeSet::new();

    for contract in &catalog.contracts {
        if !contract_ids.insert(contract.id.as_str()) {
            issues.push(ValidationIssue::new(
                ValidationCode::DuplicateContractId,
                contract.id.clone(),
            ));
        }

        let expected_id = format!("contract.{}.{}", contract.feature_id, contract.scenario);
        if contract.id != expected_id {
            issues.push(ValidationIssue::new(
                ValidationCode::ContractIdentityMismatch,
                contract.id.clone(),
            ));
        }

        if !feature_ids.contains(&contract.feature_id) {
            issues.push(ValidationIssue::new(
                ValidationCode::DanglingFeatureReference,
                contract.feature_id.clone(),
            ));
        }

        for state in REQUIRED_STATES {
            if !contract.loading.states.contains(&state) {
                issues.push(ValidationIssue::new(
                    ValidationCode::MissingLoadingState,
                    format!("{}.loading.{state:?}", contract.feature_id),
                ));
            }
        }

        let expected_fixture_prefix = format!("fixture.{}.", contract.feature_id);
        for fixture_id in &contract.fixture_refs {
            if !fixture_ids.contains(fixture_id) {
                issues.push(ValidationIssue::new(
                    ValidationCode::DanglingFixtureReference,
                    fixture_id.clone(),
                ));
            } else if !fixture_id.starts_with(&expected_fixture_prefix) {
                issues.push(ValidationIssue::new(
                    ValidationCode::CrossFeatureFixtureReference,
                    fixture_id.clone(),
                ));
            }
        }
    }

    issues
}
