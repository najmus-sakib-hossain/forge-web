//! Pipeline Execution & Orchestration APIs

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::{Arc, OnceLock};

/// Pipeline execution state
static PIPELINE_STATE: OnceLock<Arc<RwLock<PipelineState>>> = OnceLock::new();

struct PipelineState {
    active_pipeline: Option<String>,
    execution_order: Vec<String>,
    is_suspended: bool,
    override_order: Option<Vec<String>>,
}

impl Default for PipelineState {
    fn default() -> Self {
        Self {
            active_pipeline: None,
            execution_order: Vec::new(),
            is_suspended: false,
            override_order: None,
        }
    }
}

fn get_pipeline_state() -> Arc<RwLock<PipelineState>> {
    PIPELINE_STATE.get_or_init(|| Arc::new(RwLock::new(PipelineState::default()))).clone()
}

/// Executes named pipeline ("default" | "auth" | "deploy" | "ci")
pub fn execute_pipeline(pipeline_name: &str) -> Result<()> {
    let state = get_pipeline_state();
    let mut state = state.write();
    
    if state.is_suspended {
        anyhow::bail!("Pipeline execution is suspended");
    }
    
    tracing::info!("🎼 Executing pipeline: {}", pipeline_name);
    state.active_pipeline = Some(pipeline_name.to_string());
    
    // TODO: Load pipeline configuration and execute tools
    
    Ok(())
}

/// Highest priority execution — bypasses queue and debounce
pub fn execute_tool_immediately(tool_id: &str) -> Result<()> {
    tracing::info!("⚡ Immediate execution: {}", tool_id);
    
    // TODO: Execute tool directly, bypassing normal queue
    
    Ok(())
}

/// Returns final Vec<ToolId> after topology sort
pub fn get_resolved_execution_order() -> Result<Vec<String>> {
    let state = get_pipeline_state();
    let state = state.read();
    
    if let Some(override_order) = &state.override_order {
        Ok(override_order.clone())
    } else {
        Ok(state.execution_order.clone())
    }
}

/// Used by traffic_branching and user experiments
pub fn temporarily_override_pipeline_order(new_order: Vec<String>) -> Result<()> {
    let state = get_pipeline_state();
    let mut state = state.write();
    
    tracing::info!("🔀 Temporarily overriding pipeline order");
    state.override_order = Some(new_order);
    
    Ok(())
}

/// Aborts and restarts active pipeline from scratch
pub fn restart_current_pipeline() -> Result<()> {
    let state = get_pipeline_state();
    let state = state.read();
    
    if let Some(pipeline) = &state.active_pipeline {
        let name = pipeline.clone();
        drop(state);
        
        tracing::info!("🔄 Restarting pipeline: {}", name);
        execute_pipeline(&name)?;
    } else {
        anyhow::bail!("No active pipeline to restart");
    }
    
    Ok(())
}

/// Pauses all tool execution until resumed
pub fn suspend_pipeline_execution() -> Result<()> {
    let state = get_pipeline_state();
    let mut state = state.write();
    
    tracing::info!("⏸️  Pipeline execution suspended");
    state.is_suspended = true;
    
    Ok(())
}

/// Continues from suspended state
pub fn resume_pipeline_execution() -> Result<()> {
    let state = get_pipeline_state();
    let mut state = state.write();
    
    tracing::info!("▶️  Pipeline execution resumed");
    state.is_suspended = false;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_execution() {
        assert!(execute_pipeline("default").is_ok());
    }
    
    #[test]
    fn test_suspend_resume() {
        suspend_pipeline_execution().unwrap();
        assert!(execute_pipeline("test").is_err());
        
        resume_pipeline_execution().unwrap();
        assert!(execute_pipeline("test").is_ok());
    }
}
