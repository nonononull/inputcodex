#![forbid(unsafe_code)]

use inputcodex_baseline::run_scenario;
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    match execute() {
        Ok(csv) => {
            println!("{csv}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn execute() -> Result<String, String> {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != 4 {
        return Err(
            "用法：inputcodex-baseline <scenario> <repository-root> <iterations>".to_owned(),
        );
    }

    let iterations = arguments[3]
        .parse::<u64>()
        .map_err(|error| format!("无效迭代数：{error}"))?;
    let measurement = run_scenario(&arguments[1], Path::new(&arguments[2]), iterations)
        .map_err(|error| error.to_string())?;

    Ok(measurement.to_csv())
}
