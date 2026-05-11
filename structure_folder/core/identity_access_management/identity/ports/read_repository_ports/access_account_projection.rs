use async_trait::async_trait;
use event_hex::shared_kernel::domain::EntityId;
use event_hex::shared_kernel::errors::EventHexError;
use uuid::Uuid;

use crate::application::ports::projections::models::access_account::AccessAccountProjection;

#[async_trait]
pub trait AccessAccountReadProjectionRepository: Send + Sync {
    async fn get_projection(&self, id: &EntityId) -> Result<Option<AccessAccountProjection>, EventHexError>;
    async fn find_projection_by_user_id(&self, user_id: Uuid) -> Result<Option<AccessAccountProjection>, EventHexError>;
    async fn find_projection_by_email(&self, email: String) -> Result<Option<AccessAccountProjection>, EventHexError>;
}

//==============================================================================================
// Mock AccessAccountReadProjectionRepository
//==============================================================================================
pub struct MockAccessAccountProjectionRepo {
    _private: (),
}

impl MockAccessAccountProjectionRepo {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for MockAccessAccountProjectionRepo {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AccessAccountReadProjectionRepository for MockAccessAccountProjectionRepo {
    async fn get_projection(&self, _id: &EntityId) -> Result<Option<AccessAccountProjection>, EventHexError> {
        Ok(None)
    }

    async fn find_projection_by_user_id(&self, _user_id: Uuid) -> Result<Option<AccessAccountProjection>, EventHexError> {
        Ok(None)
    }

    async fn find_projection_by_email(&self, _email: String) -> Result<Option<AccessAccountProjection>, EventHexError> {
        Ok(None)
    }
}
