use crate::structs::core::GetByFilter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Custom error types for PayloadFilterScenarios operations
#[derive(Error, Debug)]
pub enum PayloadFilterScenarioError {
    #[error("Scenario '{0}' not found")]
    ScenarioNotFound(String),
    #[error("Scenario '{0}' already exists")]
    ScenarioAlreadyExists(String),
    #[error("Invalid scenario name: '{0}'")]
    InvalidScenarioName(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Represents a scenario with its associated GetByFilter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Scenario {
    pub name: String,
    pub description: Option<String>,
    pub filter: GetByFilter,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Scenario {
    /// Creates a new scenario with the given name and filter
    pub fn new(name: String, filter: GetByFilter) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            description: None,
            filter,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a new scenario with description
    pub fn with_description(name: String, description: String, filter: GetByFilter) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            description: Some(description),
            filter,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the scenario's filter and timestamp
    pub fn update_filter(&mut self, filter: GetByFilter) {
        self.filter = filter;
        self.updated_at = chrono::Utc::now();
    }

    /// Updates the scenario's description and timestamp
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = chrono::Utc::now();
    }
}

/// Main struct for managing payload filters and scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadFilterScenarios {
    main_table: String,
    default_payload: GetByFilter,
    current_scenario: Option<String>,
    scenarios: HashMap<String, Scenario>,
}

#[allow(dead_code)]
impl PayloadFilterScenarios {
    /// Creates a new PayloadFilterScenarios instance
    pub fn new(main_table: String, default_payload: GetByFilter) -> Self {
        Self {
            main_table,
            default_payload,
            current_scenario: None,
            scenarios: HashMap::new(),
        }
    }

    /// Gets the main table name
    pub fn get_main_table(&self) -> &str {
        &self.main_table
    }

    /// Gets the default payload
    pub fn get_default_payload(&self) -> &GetByFilter {
        &self.default_payload
    }

    /// Gets the current active scenario name
    pub fn get_current_scenario(&self) -> Option<&String> {
        self.current_scenario.as_ref()
    }

    /// Creates a new scenario
    pub fn create_scenario(
        &mut self,
        name: String,
        filter: GetByFilter,
    ) -> Result<(), PayloadFilterScenarioError> {
        self.validate_scenario_name(&name)?;

        if self.scenarios.contains_key(&name) {
            return Err(PayloadFilterScenarioError::ScenarioAlreadyExists(name));
        }

        let scenario = Scenario::new(name.clone(), filter);
        self.scenarios.insert(name.clone(), scenario);

        // Automatically save the scenario to file
        self.save_scenario_to_file(&name)?;

        Ok(())
    }

    /// Creates a new scenario with description
    pub fn create_scenario_with_description(
        &mut self,
        name: String,
        description: String,
        filter: GetByFilter,
    ) -> Result<(), PayloadFilterScenarioError> {
        self.validate_scenario_name(&name)?;

        if self.scenarios.contains_key(&name) {
            return Err(PayloadFilterScenarioError::ScenarioAlreadyExists(name));
        }

        let scenario = Scenario::with_description(name.clone(), description, filter);
        self.scenarios.insert(name.clone(), scenario);

        // Automatically save the scenario to file
        self.save_scenario_to_file(&name)?;

        Ok(())
    }

    /// Updates an existing scenario's filter
    pub fn update_scenario(
        &mut self,
        name: &str,
        filter: GetByFilter,
    ) -> Result<(), PayloadFilterScenarioError> {
        match self.scenarios.get_mut(name) {
            Some(scenario) => {
                scenario.update_filter(filter);
                // Automatically save the updated scenario to file
                self.save_scenario_to_file(name)?;
                Ok(())
            }
            None => Err(PayloadFilterScenarioError::ScenarioNotFound(
                name.to_string(),
            )),
        }
    }

    /// Updates an existing scenario's description
    pub fn update_scenario_description(
        &mut self,
        name: &str,
        description: Option<String>,
    ) -> Result<(), PayloadFilterScenarioError> {
        match self.scenarios.get_mut(name) {
            Some(scenario) => {
                scenario.update_description(description);
                Ok(())
            }
            None => Err(PayloadFilterScenarioError::ScenarioNotFound(
                name.to_string(),
            )),
        }
    }

    /// Gets a scenario by name
    pub fn get_scenario(&self, name: &str) -> Result<&Scenario, PayloadFilterScenarioError> {
        self.scenarios
            .get(name)
            .ok_or_else(|| PayloadFilterScenarioError::ScenarioNotFound(name.to_string()))
    }

    /// Gets a mutable reference to a scenario by name
    pub fn get_scenario_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut Scenario, PayloadFilterScenarioError> {
        self.scenarios
            .get_mut(name)
            .ok_or_else(|| PayloadFilterScenarioError::ScenarioNotFound(name.to_string()))
    }

    /// Gets the filter for a specific scenario
    pub fn get_scenario_filter(
        &self,
        name: &str,
    ) -> Result<&GetByFilter, PayloadFilterScenarioError> {
        Ok(&self.get_scenario(name)?.filter)
    }

    /// Sets the current active scenario
    pub fn set_current_scenario(&mut self, name: &str) -> Result<(), PayloadFilterScenarioError> {
        if !self.scenarios.contains_key(name) {
            return Err(PayloadFilterScenarioError::ScenarioNotFound(
                name.to_string(),
            ));
        }
        self.current_scenario = Some(name.to_string());
        Ok(())
    }

    /// Clears the current active scenario
    pub fn clear_current_scenario(&mut self) {
        self.current_scenario = None;
    }

    /// Gets the current active filter (scenario or default)
    pub fn get_current_filter(&self) -> &GetByFilter {
        match &self.current_scenario {
            Some(scenario_name) => self
                .scenarios
                .get(scenario_name)
                .map(|s| &s.filter)
                .unwrap_or(&self.default_payload),
            None => &self.default_payload,
        }
    }

    /// Deletes a scenario
    pub fn delete_scenario(&mut self, name: &str) -> Result<Scenario, PayloadFilterScenarioError> {
        // Clear current scenario if it's the one being deleted
        if let Some(current) = &self.current_scenario {
            if current == name {
                self.current_scenario = None;
            }
        }

        let scenario = self
            .scenarios
            .remove(name)
            .ok_or_else(|| PayloadFilterScenarioError::ScenarioNotFound(name.to_string()))?;

        // Automatically delete the scenario file
        let file_path = Path::new("scenarios/filters").join(format!("{}.json", name));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }

        Ok(scenario)
    }

    /// Lists all scenario names
    pub fn list_scenarios(&self) -> Vec<&String> {
        self.scenarios.keys().collect()
    }

    /// Gets all scenarios
    pub fn get_all_scenarios(&self) -> &HashMap<String, Scenario> {
        &self.scenarios
    }

    /// Checks if a scenario exists
    pub fn has_scenario(&self, name: &str) -> bool {
        self.scenarios.contains_key(name)
    }

    /// Gets the number of scenarios
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    /// Clones a scenario with a new name
    pub fn clone_scenario(
        &mut self,
        source_name: &str,
        target_name: String,
    ) -> Result<(), PayloadFilterScenarioError> {
        self.validate_scenario_name(&target_name)?;

        if self.scenarios.contains_key(&target_name) {
            return Err(PayloadFilterScenarioError::ScenarioAlreadyExists(
                target_name,
            ));
        }

        let source_scenario = self.get_scenario(source_name)?.clone();
        let mut new_scenario = Scenario::new(target_name.clone(), source_scenario.filter);
        new_scenario.description = source_scenario.description;

        self.scenarios.insert(target_name, new_scenario);
        Ok(())
    }

    /// Saves a scenario to an individual JSON file in scenarios/filters directory
    pub fn save_scenario_to_file(
        &self,
        scenario_name: &str,
    ) -> Result<(), PayloadFilterScenarioError> {
        let scenario = self.get_scenario(scenario_name)?;
        let scenarios_dir = Path::new("scenarios/filters");

        // Create directory if it doesn't exist
        if !scenarios_dir.exists() {
            fs::create_dir_all(scenarios_dir)?;
        }

        let file_path = scenarios_dir.join(format!("{}.json", scenario_name));
        let json = serde_json::to_string_pretty(&scenario.filter)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    /// Loads a scenario from an individual JSON file
    pub fn load_scenario_from_file(
        &mut self,
        scenario_name: &str,
    ) -> Result<(), PayloadFilterScenarioError> {
        let file_path = Path::new("scenarios/filters").join(format!("{}.json", scenario_name));
        let content = fs::read_to_string(&file_path)?;
        let filter: GetByFilter = serde_json::from_str(&content)?;

        self.create_scenario(scenario_name.to_string(), filter)?;
        Ok(())
    }

    /// Saves all scenarios to individual JSON files
    pub fn save_all_scenarios_to_files(&self) -> Result<(), PayloadFilterScenarioError> {
        for scenario_name in self.list_scenarios() {
            self.save_scenario_to_file(scenario_name)?;
        }
        Ok(())
    }

    /// Loads all scenarios from JSON files in the scenarios/filters directory
    pub fn load_all_scenarios_from_files(&mut self) -> Result<usize, PayloadFilterScenarioError> {
        let scenarios_dir = Path::new("scenarios/filters");

        if !scenarios_dir.exists() {
            return Ok(0);
        }

        let mut loaded_count = 0;

        for entry in fs::read_dir(scenarios_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(()) = self.load_scenario_from_file(file_stem) {
                        loaded_count += 1;
                    }
                }
            }
        }

        Ok(loaded_count)
    }

    /// Saves scenarios to a JSON file (legacy method)
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), PayloadFilterScenarioError> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Loads scenarios from a JSON file (legacy method)
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, PayloadFilterScenarioError> {
        let content = fs::read_to_string(path)?;
        let payload_filters: PayloadFilterScenarios = serde_json::from_str(&content)?;
        Ok(payload_filters)
    }

    /// Validates scenario name
    fn validate_scenario_name(&self, name: &str) -> Result<(), PayloadFilterScenarioError> {
        if name.trim().is_empty() {
            return Err(PayloadFilterScenarioError::InvalidScenarioName(
                "Scenario name cannot be empty".to_string(),
            ));
        }

        if name.len() > 100 {
            return Err(PayloadFilterScenarioError::InvalidScenarioName(
                "Scenario name cannot exceed 100 characters".to_string(),
            ));
        }

        // Check for invalid characters
        if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
            return Err(PayloadFilterScenarioError::InvalidScenarioName(
                "Scenario name contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Exports scenarios to a specific format
    pub fn export_scenarios(&self) -> Result<String, PayloadFilterScenarioError> {
        serde_json::to_string_pretty(&self.scenarios).map_err(PayloadFilterScenarioError::from)
    }

    /// Imports scenarios from JSON string
    pub fn import_scenarios(&mut self, json: &str) -> Result<usize, PayloadFilterScenarioError> {
        let imported_scenarios: HashMap<String, Scenario> = serde_json::from_str(json)?;
        let count = imported_scenarios.len();

        for (name, scenario) in imported_scenarios {
            self.scenarios.insert(name, scenario);
        }

        Ok(count)
    }

    /// Merges scenarios from another PayloadFilterScenarios instance
    pub fn merge_scenarios(&mut self, other: &PayloadFilterScenarios) -> usize {
        let mut merged_count = 0;

        for (name, scenario) in &other.scenarios {
            if !self.scenarios.contains_key(name) {
                self.scenarios.insert(name.clone(), scenario.clone());
                merged_count += 1;
            }
        }

        merged_count
    }
}

impl Default for PayloadFilterScenarios {
    fn default() -> Self {
        Self::new("default_table".to_string(), GetByFilter::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_filter() -> GetByFilter {
        GetByFilter {
            pluck: vec!["id".to_string(), "name".to_string()],
            limit: 10,
            offset: 0,
            ..Default::default()
        }
    }

    #[test]
    fn should_create_new_payload_filters() {
        println!("Testing PayloadFilterScenarios creation");

        let filter = create_test_filter();
        let payload_filters = PayloadFilterScenarios::new("test_table".to_string(), filter.clone());

        assert_eq!(payload_filters.get_main_table(), "test_table");
        assert_eq!(payload_filters.get_default_payload().limit, 10);
        assert_eq!(payload_filters.scenario_count(), 0);
        assert!(payload_filters.get_current_scenario().is_none());

        println!("✓ PayloadFilterScenarios created successfully");
    }

    #[test]
    fn should_create_and_retrieve_scenario() {
        println!("Testing scenario creation and retrieval");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        let result = payload_filters.create_scenario("test_scenario".to_string(), scenario_filter);
        assert!(result.is_ok());

        let scenario = payload_filters.get_scenario("test_scenario");
        assert!(scenario.is_ok());
        assert_eq!(scenario.unwrap().name, "test_scenario");

        println!("✓ Scenario created and retrieved successfully");
    }

    #[test]
    fn should_handle_duplicate_scenario_creation() {
        println!("Testing duplicate scenario handling");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        payload_filters
            .create_scenario("duplicate".to_string(), scenario_filter.clone())
            .unwrap();
        let result = payload_filters.create_scenario("duplicate".to_string(), scenario_filter);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PayloadFilterScenarioError::ScenarioAlreadyExists(_)
        ));

        println!("✓ Duplicate scenario creation properly handled");
    }

    #[test]
    fn should_set_and_get_current_scenario() {
        println!("Testing current scenario management");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        payload_filters
            .create_scenario("current_test".to_string(), scenario_filter)
            .unwrap();
        payload_filters
            .set_current_scenario("current_test")
            .unwrap();

        assert_eq!(
            payload_filters.get_current_scenario(),
            Some(&"current_test".to_string())
        );

        payload_filters.clear_current_scenario();
        assert!(payload_filters.get_current_scenario().is_none());

        println!("✓ Current scenario management working correctly");
    }

    #[test]
    fn should_validate_scenario_names() {
        println!("Testing scenario name validation");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        // Test empty name
        let result = payload_filters.create_scenario("".to_string(), scenario_filter.clone());
        assert!(result.is_err());

        // Test invalid characters
        let result = payload_filters.create_scenario("test/scenario".to_string(), scenario_filter);
        assert!(result.is_err());

        println!("✓ Scenario name validation working correctly");
    }

    #[test]
    fn should_delete_scenario() {
        println!("Testing scenario deletion");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        payload_filters
            .create_scenario("to_delete".to_string(), scenario_filter)
            .unwrap();
        assert!(payload_filters.has_scenario("to_delete"));

        let deleted = payload_filters.delete_scenario("to_delete");
        assert!(deleted.is_ok());
        assert!(!payload_filters.has_scenario("to_delete"));

        println!("✓ Scenario deletion working correctly");
    }

    #[test]
    fn should_clone_scenario() {
        println!("Testing scenario cloning");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        payload_filters
            .create_scenario_with_description(
                "original".to_string(),
                "Original scenario".to_string(),
                scenario_filter,
            )
            .unwrap();

        let result = payload_filters.clone_scenario("original", "cloned".to_string());
        assert!(result.is_ok());

        let cloned = payload_filters.get_scenario("cloned").unwrap();
        assert_eq!(cloned.name, "cloned");
        assert_eq!(cloned.description, Some("Original scenario".to_string()));

        println!("✓ Scenario cloning working correctly");
    }

    #[test]
    fn should_save_and_load_scenario_from_file() {
        println!("Testing scenario file save and load");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        // Create scenario (should auto-save to file)
        payload_filters
            .create_scenario("file_test".to_string(), scenario_filter.clone())
            .unwrap();

        // Verify file exists
        let file_path = Path::new("scenarios/filters/file_test.json");
        assert!(file_path.exists());

        // Create new instance and load scenario
        let mut new_payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let result = new_payload_filters.load_scenario_from_file("file_test");
        assert!(result.is_ok());

        let loaded_scenario = new_payload_filters.get_scenario("file_test").unwrap();
        assert_eq!(loaded_scenario.filter.limit, scenario_filter.limit);
        assert_eq!(loaded_scenario.filter.pluck, scenario_filter.pluck);

        // Cleanup
        let _ = fs::remove_file(file_path);

        println!("✓ Scenario file save and load working correctly");
    }

    #[test]
    fn should_delete_scenario_and_file() {
        println!("Testing scenario deletion with file cleanup");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let scenario_filter = create_test_filter();

        // Create scenario
        payload_filters
            .create_scenario("delete_test".to_string(), scenario_filter)
            .unwrap();

        let file_path = Path::new("scenarios/filters/delete_test.json");
        assert!(file_path.exists());

        // Delete scenario
        let result = payload_filters.delete_scenario("delete_test");
        assert!(result.is_ok());

        // Verify file is deleted
        assert!(!file_path.exists());
        assert!(!payload_filters.has_scenario("delete_test"));

        println!("✓ Scenario deletion with file cleanup working correctly");
    }

    #[test]
    fn should_update_scenario_and_save_to_file() {
        println!("Testing scenario update with file save");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let initial_filter = create_test_filter();

        // Create scenario
        payload_filters
            .create_scenario("update_test".to_string(), initial_filter)
            .unwrap();

        // Update scenario
        let mut updated_filter = create_test_filter();
        updated_filter.limit = 50;
        updated_filter.pluck = vec!["updated_field".to_string()];

        let result = payload_filters.update_scenario("update_test", updated_filter.clone());
        assert!(result.is_ok());

        // Verify the file contains updated data
        let file_path = Path::new("scenarios/filters/update_test.json");
        let content = fs::read_to_string(&file_path).unwrap();
        let loaded_filter: GetByFilter = serde_json::from_str(&content).unwrap();

        assert_eq!(loaded_filter.limit, 50);
        assert_eq!(loaded_filter.pluck, vec!["updated_field".to_string()]);

        // Cleanup
        let _ = fs::remove_file(file_path);

        println!("✓ Scenario update with file save working correctly");
    }

    #[test]
    fn should_load_all_scenarios_from_files() {
        println!("Testing loading all scenarios from files");

        let mut payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());

        // Create multiple scenarios
        let filter1 = create_test_filter();
        let mut filter2 = create_test_filter();
        filter2.limit = 25;

        payload_filters
            .create_scenario("bulk_test_1".to_string(), filter1)
            .unwrap();
        payload_filters
            .create_scenario("bulk_test_2".to_string(), filter2)
            .unwrap();

        // Create new instance and load all scenarios
        let mut new_payload_filters =
            PayloadFilterScenarios::new("test_table".to_string(), create_test_filter());
        let loaded_count = new_payload_filters.load_all_scenarios_from_files().unwrap();

        assert!(loaded_count >= 2); // At least our 2 test scenarios
        assert!(new_payload_filters.has_scenario("bulk_test_1"));
        assert!(new_payload_filters.has_scenario("bulk_test_2"));

        let scenario2 = new_payload_filters.get_scenario("bulk_test_2").unwrap();
        assert_eq!(scenario2.filter.limit, 25);

        // Cleanup
        let _ = fs::remove_file("scenarios/filters/bulk_test_1.json");
        let _ = fs::remove_file("scenarios/filters/bulk_test_2.json");

        println!("✓ Loading all scenarios from files working correctly");
    }
}
