use std::any::Any;

use crate::app_config::AppConfig;

/**
 * Trait defining the model dependent logic to optimize the incoming json.
 */
pub trait MetadataOptimizer {
    fn optimize_metadata(&self) -> Result<String, String>;
    fn as_any(&self) -> &dyn Any;
}
