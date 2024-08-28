use std::future::Future;

use super::models::{
    CreateNodeConfig, CreateNodeConfigError, GetNodeConfig, GetNodeConfigError, NodeConfig,
    UpdateNodeConfig, UpdateNodeConfigError,
};

/// NodeConfigRepository
pub trait NodeConfigRepository: Clone + Send + Sync + 'static {
    /// Creates a new NodeConfig in the repository.
    fn create_node_config(
        &self,
        req: &CreateNodeConfig,
    ) -> impl Future<Output = Result<NodeConfig, CreateNodeConfigError>> + Send;

    /// Retrieves the NodeConfig from the repository.
    fn get_node_config(
        &self,
        req: &GetNodeConfig,
    ) -> impl Future<Output = Result<NodeConfig, GetNodeConfigError>> + Send;

    /// Updates an existing NodeConfig in the repository.
    fn update_node_config(
        &self,
        req: &UpdateNodeConfig,
    ) -> impl Future<Output = Result<(), UpdateNodeConfigError>> + Send;
}
