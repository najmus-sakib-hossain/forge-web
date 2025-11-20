//! Version control and tool registry
//!
//! Provides semantic versioning, version requirements, and tool registry management.

pub mod types;
pub mod registry;

pub use types::{Version, VersionReq};
pub use registry::{ToolInfo, ToolRegistry, ToolSource};
