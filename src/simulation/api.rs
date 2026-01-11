//! Simulation API operations

use super::types::*;
use crate::client::{encode_path_segment, Client};
use crate::error::Result;

/// Simulation API client
pub struct SimulationApi<'a> {
    client: &'a Client,
}

impl<'a> SimulationApi<'a> {
    /// Create a new simulation API client
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Simulate a single transaction
    ///
    /// # Example
    ///
    /// ```ignore
    /// let request = SimulationRequest::new(
    ///     "0x0000000000000000000000000000000000000000",
    ///     "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    ///     "0x70a08231000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa96045"
    /// );
    /// let result = client.simulation().simulate(&request).await?;
    /// ```
    pub async fn simulate(&self, request: &SimulationRequest) -> Result<SimulationResponse> {
        self.client.post("/simulate", request).await
    }

    /// Simulate a bundle of transactions in sequence
    ///
    /// Each transaction is simulated on top of the state changes from previous ones.
    pub async fn simulate_bundle(
        &self,
        request: &BundleSimulationRequest,
    ) -> Result<BundleSimulationResponse> {
        self.client.post("/simulate-bundle", request).await
    }

    /// List saved simulations
    ///
    /// # Arguments
    ///
    /// * `page` - Page number (0-indexed)
    /// * `per_page` - Number of results per page (max 100)
    pub async fn list(&self, page: u32, per_page: u32) -> Result<SimulationListResponse> {
        let query = SimulationListQuery { page, per_page };
        self.client.get_with_query("/simulations", &query).await
    }

    /// Get a saved simulation by ID
    pub async fn get(&self, id: &str) -> Result<SimulationResponse> {
        self.client
            .get(&format!("/simulations/{}", encode_path_segment(id)))
            .await
    }

    /// Get simulation info/metadata by ID
    pub async fn info(&self, id: &str) -> Result<serde_json::Value> {
        self.client
            .get(&format!("/simulations/{}/info", encode_path_segment(id)))
            .await
    }

    /// Share a simulation publicly
    ///
    /// Returns the public URL for the shared simulation.
    pub async fn share(&self, id: &str) -> Result<String> {
        let empty: serde_json::Value = serde_json::json!({});
        self.client
            .post_no_response(
                &format!("/simulations/{}/share", encode_path_segment(id)),
                &empty,
            )
            .await?;

        Ok(format!(
            "https://dashboard.tenderly.co/shared/simulation/{}",
            encode_path_segment(id)
        ))
    }

    /// Unshare a simulation (make it private)
    pub async fn unshare(&self, id: &str) -> Result<()> {
        let empty: serde_json::Value = serde_json::json!({});
        self.client
            .post_no_response(
                &format!("/simulations/{}/unshare", encode_path_segment(id)),
                &empty,
            )
            .await
    }

    /// Trace an existing transaction
    pub async fn trace(&self, hash: &str) -> Result<serde_json::Value> {
        self.client
            .get(&format!("/trace/{}", encode_path_segment(hash)))
            .await
    }
}

#[derive(serde::Serialize)]
struct SimulationListQuery {
    page: u32,
    #[serde(rename = "perPage")]
    per_page: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_request_builder() {
        let request = SimulationRequest::new("0x1234", "0x5678", "0xabcd")
            .network_id("137")
            .value_wei(1_000_000_000_000_000_000u128)
            .gas(100_000)
            .block_number(12_345_678)
            .save(true);

        assert_eq!(request.network_id, "137");
        assert_eq!(request.from, "0x1234");
        assert_eq!(request.to, "0x5678");
        assert_eq!(request.input, "0xabcd");
        assert_eq!(request.value, Some("0xde0b6b3a7640000".to_string()));
        assert_eq!(request.gas, Some(100_000));
        assert_eq!(request.block_number, Some(12_345_678));
        assert!(request.save);
    }

    #[test]
    fn test_simulation_request_state_overrides() {
        let request = SimulationRequest::new("0x1234", "0x5678", "0xabcd")
            .override_balance("0xaaaa", "1000000000000000000")
            .override_storage("0xbbbb", "0x0", "0x1")
            .override_code("0xcccc", "0x6080");

        let overrides = request.state_objects.unwrap();
        assert!(overrides.contains_key("0xaaaa"));
        assert!(overrides.contains_key("0xbbbb"));
        assert!(overrides.contains_key("0xcccc"));
    }
}
