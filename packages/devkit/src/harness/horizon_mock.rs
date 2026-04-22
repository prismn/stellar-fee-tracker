/// Mock implementation of the Horizon API server for use in tests.
pub struct HorizonMock {
    /// Path to the scenario JSON file to serve.
    pub scenario_path: std::path::PathBuf,
}

impl HorizonMock {
    pub fn new(scenario_path: impl Into<std::path::PathBuf>) -> Self {
        Self { scenario_path: scenario_path.into() }
    }

    /// Loads and returns the scenario JSON to be served at `GET /fee_stats`.
    pub fn fee_stats_payload(&self) -> std::io::Result<String> {
        crate::harness::scenarios::load_from_file(&self.scenario_path)
    }
}
