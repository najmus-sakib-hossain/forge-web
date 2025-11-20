//! Simple Orchestrator - Only controls WHEN to run tools
//!
//! Tools are self-contained and know:
//! - What files to process
//! - When they should run
//! - What patterns to detect
//!
//! Forge just detects changes and asks: "Should you run?"

use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Tool execution context shared across all tools
#[derive(Clone)]
pub struct ExecutionContext {
    /// Repository root path
    pub repo_root: PathBuf,

    /// Forge storage path (.dx/forge)
    pub forge_path: PathBuf,

    /// Current Git branch
    pub current_branch: Option<String>,

    /// Changed files in this execution
    pub changed_files: Vec<PathBuf>,

    /// Shared state between tools
    pub shared_state: Arc<RwLock<HashMap<String, serde_json::Value>>>,

    /// Traffic branch analyzer
    pub traffic_analyzer: Arc<dyn TrafficAnalyzer + Send + Sync>,

    /// Component state manager for traffic branch system
    pub component_manager: Option<Arc<RwLock<crate::context::ComponentStateManager>>>,
}

impl std::fmt::Debug for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExecutionContext")
            .field("repo_root", &self.repo_root)
            .field("forge_path", &self.forge_path)
            .field("current_branch", &self.current_branch)
            .field("changed_files", &self.changed_files)
            .field("traffic_analyzer", &"<dyn TrafficAnalyzer>")
            .finish()
    }
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(repo_root: PathBuf, forge_path: PathBuf) -> Self {
        // Try to create component state manager
        let component_manager = crate::context::ComponentStateManager::new(&forge_path)
            .ok()
            .map(|mgr| Arc::new(RwLock::new(mgr)));

        Self {
            repo_root,
            forge_path,
            current_branch: None,
            changed_files: Vec::new(),
            shared_state: Arc::new(RwLock::new(HashMap::new())),
            traffic_analyzer: Arc::new(DefaultTrafficAnalyzer),
            component_manager,
        }
    }

    /// Set a shared value
    pub fn set<T: Serialize>(&self, key: impl Into<String>, value: T) -> Result<()> {
        let json = serde_json::to_value(value)?;
        self.shared_state.write().insert(key.into(), json);
        Ok(())
    }

    /// Get a shared value
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let state = self.shared_state.read();
        if let Some(value) = state.get(key) {
            let result = serde_json::from_value(value.clone())?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    /// Find regex patterns in a file
    pub fn find_patterns(&self, _pattern: &str) -> Result<Vec<PatternMatch>> {
        // Implementation will be added
        Ok(Vec::new())
    }
}

/// Pattern match result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
    pub text: String,
    pub captures: Vec<String>,
}

/// Output from tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub success: bool,
    pub files_modified: Vec<PathBuf>,
    pub files_created: Vec<PathBuf>,
    pub files_deleted: Vec<PathBuf>,
    pub message: String,
    pub duration_ms: u64,
}

impl ToolOutput {
    pub fn success() -> Self {
        Self {
            success: true,
            files_modified: Vec::new(),
            files_created: Vec::new(),
            files_deleted: Vec::new(),
            message: "Success".to_string(),
            duration_ms: 0,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            files_modified: Vec::new(),
            files_created: Vec::new(),
            files_deleted: Vec::new(),
            message: message.into(),
            duration_ms: 0,
        }
    }
}

/// Main DX tool trait - all tools must implement this
pub trait DxTool: Send + Sync {
    /// Tool name (e.g., "dx-ui", "dx-style")
    fn name(&self) -> &str;

    /// Tool version
    fn version(&self) -> &str;

    /// Execution priority (lower = executes earlier)
    fn priority(&self) -> u32;

    /// Execute the tool
    fn execute(&mut self, context: &ExecutionContext) -> Result<ToolOutput>;

    /// Check if tool should run (optional pre-check)
    fn should_run(&self, _context: &ExecutionContext) -> bool {
        true
    }

    /// Tool dependencies (must run after these tools)
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }

    /// Before execution hook (setup, validation)
    fn before_execute(&mut self, _context: &ExecutionContext) -> Result<()> {
        Ok(())
    }

    /// After execution hook (cleanup, reporting)
    fn after_execute(&mut self, _context: &ExecutionContext, _output: &ToolOutput) -> Result<()> {
        Ok(())
    }

    /// On error hook (rollback, cleanup)
    fn on_error(&mut self, _context: &ExecutionContext, _error: &anyhow::Error) -> Result<()> {
        Ok(())
    }

    /// Execution timeout in seconds (0 = no timeout)
    fn timeout_seconds(&self) -> u64 {
        60
    }
}

// Tools are self-contained - no manifests needed
// Each tool knows what to do and when to run

/// Traffic branch analysis result
#[derive(Debug, Clone, PartialEq)]
pub enum TrafficBranch {
    /// 🟢 Green: Safe to auto-update
    Green,

    /// 🟡 Yellow: Can merge with conflicts
    Yellow { conflicts: Vec<Conflict> },

    /// 🔴 Red: Manual resolution required
    Red { conflicts: Vec<Conflict> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conflict {
    pub path: PathBuf,
    pub line: usize,
    pub reason: String,
}

/// Traffic branch analyzer trait
pub trait TrafficAnalyzer {
    fn analyze(&self, file: &Path) -> Result<TrafficBranch>;
    fn can_auto_merge(&self, conflicts: &[Conflict]) -> bool;
}

/// Default traffic analyzer implementation
pub struct DefaultTrafficAnalyzer;

impl TrafficAnalyzer for DefaultTrafficAnalyzer {
    fn analyze(&self, file: &Path) -> Result<TrafficBranch> {
        // Analyze file to determine traffic branch
        let extension = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        // 🟢 Green: Auto-update (safe files that don't affect APIs or types)
        let green_patterns = [
            "md", "txt", "json", // Documentation and config
            "css", "scss", "less", // Styles
            "png", "jpg", "svg", "ico", // Assets
            "test.ts", "test.js", "spec.ts", "spec.js", // Tests
        ];

        // 🔴 Red: Manual resolution (breaking changes, API modifications)
        let red_patterns = [
            "proto", // Protocol buffers
            "graphql", "gql", // GraphQL schemas
            "sql", // Database migrations
        ];

        // Check if file matches green patterns
        if green_patterns.iter().any(|p| extension.ends_with(p)) {
            return Ok(TrafficBranch::Green);
        }

        // Check if file matches red patterns
        if red_patterns.iter().any(|p| extension.ends_with(p)) {
            let conflict = Conflict {
                path: file.to_path_buf(),
                line: 0,
                reason: format!("Breaking change potential: {} file modification", extension),
            };
            return Ok(TrafficBranch::Red {
                conflicts: vec![conflict],
            });
        }

        // 🟡 Yellow: Merge required (code files that may have conflicts)
        // ts, tsx, js, jsx, rs, go, py, etc.
        if matches!(
            extension,
            "ts" | "tsx" | "js" | "jsx" | "rs" | "go" | "py" | "java" | "cpp" | "c" | "h"
        ) {
            // Check for API-related indicators in the file path
            let path_str = file.to_string_lossy().to_lowercase();

            if path_str.contains("api")
                || path_str.contains("interface")
                || path_str.contains("types")
                || path_str.contains("schema")
            {
                // Potential API changes - Red
                let conflict = Conflict {
                    path: file.to_path_buf(),
                    line: 0,
                    reason: "API/Type definition file modification".to_string(),
                };
                return Ok(TrafficBranch::Red {
                    conflicts: vec![conflict],
                });
            }

            // Regular code file - Yellow (may have merge conflicts)
            return Ok(TrafficBranch::Yellow {
                conflicts: vec![],
            });
        }

        // Default to Yellow for unknown file types
        Ok(TrafficBranch::Yellow {
            conflicts: vec![],
        })
    }

    fn can_auto_merge(&self, conflicts: &[Conflict]) -> bool {
        conflicts.is_empty()
    }
}

/// Orchestration configuration
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// Enable parallel execution
    pub parallel: bool,

    /// Fail fast on first error
    pub fail_fast: bool,

    /// Maximum concurrent tools (for parallel mode)
    pub max_concurrent: usize,

    /// Enable traffic branch safety checks
    pub traffic_branch_enabled: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            parallel: false,
            fail_fast: true,
            max_concurrent: 4,
            traffic_branch_enabled: true,
        }
    }
}

/// Simple orchestrator - just coordinates tool execution timing
pub struct Orchestrator {
    tools: Vec<Box<dyn DxTool>>,
    context: ExecutionContext,
    config: OrchestratorConfig,
}

impl Orchestrator {
    /// Create a new orchestrator
    pub fn new(repo_root: impl Into<PathBuf>) -> Result<Self> {
        let repo_root = repo_root.into();
        let forge_path = repo_root.join(".dx/forge");

        Ok(Self {
            tools: Vec::new(),
            context: ExecutionContext::new(repo_root, forge_path),
            config: OrchestratorConfig::default(),
        })
    }

    /// Create orchestrator with custom configuration
    pub fn with_config(repo_root: impl Into<PathBuf>, config: OrchestratorConfig) -> Result<Self> {
        let repo_root = repo_root.into();
        let forge_path = repo_root.join(".dx/forge");

        Ok(Self {
            tools: Vec::new(),
            context: ExecutionContext::new(repo_root, forge_path),
            config,
        })
    }

    /// Update configuration
    pub fn set_config(&mut self, config: OrchestratorConfig) {
        self.config = config;
    }

    /// Register a tool (tools configure themselves)
    pub fn register_tool(&mut self, tool: Box<dyn DxTool>) -> Result<()> {
        let name = tool.name().to_string();
        tracing::info!(
            "📦 Registered tool: {} v{} (priority: {})",
            name,
            tool.version(),
            tool.priority()
        );
        self.tools.push(tool);
        Ok(())
    }

    /// Execute all registered tools in priority order
    pub fn execute_all(&mut self) -> Result<Vec<ToolOutput>> {
        let _start_time = std::time::Instant::now();
        tracing::info!("🎼 Orchestrator starting execution of {} tools", self.tools.len());

        // Sort tools by priority
        self.tools.sort_by_key(|t| t.priority());

        // Check dependencies
        if let Err(e) = self.validate_dependencies() {
            tracing::error!("❌ Dependency validation failed: {}", e);
            return Err(e);
        }

        // Check for circular dependencies
        if let Err(e) = self.check_circular_dependencies() {
            tracing::error!("❌ Circular dependency detected: {}", e);
            return Err(e);
        }
        tracing::debug!(
            "📋 Execution order: {}",
            self.tools
                .iter()
                .map(|t| format!("{}(p:{})", t.name(), t.priority()))
                .collect::<Vec<_>>()
                .join(" → ")
        );

        // Check dependencies
        tracing::debug!("🔍 Validating tool dependencies...");
        self.validate_dependencies()?;

        // Check for circular dependencies
        tracing::debug!("🔄 Checking for circular dependencies...");
        self.check_circular_dependencies()?;

        // Execute tools
        let mut outputs = Vec::new();
        let context = self.context.clone();
        let total_tools = self.tools.len();
        let mut executed = 0;
        let mut skipped = 0;
        let mut failed = 0;

        for tool in &mut self.tools {
            if !tool.should_run(&context) {
                tracing::info!("⏭️  Skipping {}: pre-check failed", tool.name());
                skipped += 1;
                continue;
            }

            tracing::info!(
                "🚀 Executing: {} v{} (priority: {}, {}/{})",
                tool.name(),
                tool.version(),
                tool.priority(),
                executed + 1,
                total_tools
            );

            // Execute with lifecycle hooks
            match Self::execute_tool_with_hooks(tool, &context) {
                Ok(output) => {
                    if output.success {
                        executed += 1;
                        tracing::info!("✅ {} completed in {}ms", tool.name(), output.duration_ms);
                    } else {
                        failed += 1;
                        tracing::error!("❌ {} failed: {}", tool.name(), output.message);
                        
                        if self.config.fail_fast {
                            tracing::error!("💥 Fail-fast enabled, stopping orchestration");
                            return Err(anyhow::anyhow!("Tool {} failed: {}", tool.name(), output.message));
                        }
                    }
                    outputs.push(output);
                }
                Err(e) => {
                    failed += 1;
                    tracing::error!("💥 {} error: {}", tool.name(), e);
                    
                    if self.config.fail_fast {
                        tracing::error!("💥 Fail-fast enabled, stopping orchestration");
                        return Err(e);
                    }
                    
                    outputs.push(ToolOutput::failure(format!("Error: {}", e)));
                }
            }
        }

        tracing::info!(
            "🏁 Orchestration complete: {} executed, {} skipped, {} failed",
            executed,
            skipped,
            failed
        );

        Ok(outputs)
    }

    /// Execute tool with lifecycle hooks and error handling
    fn execute_tool_with_hooks(tool: &mut Box<dyn DxTool>, context: &ExecutionContext) -> Result<ToolOutput> {
        let start = std::time::Instant::now();
        let tool_name = tool.name().to_string();

        // Before hook
        tracing::debug!("📝 Running before_execute hook for {}", tool_name);
        tool.before_execute(context)?;

        // Execute with timeout
        // Note: Since the DxTool trait's execute method is synchronous,
        // we can't use async timeout without significant refactoring.
        // Future improvement: make DxTool async or use thread-based timeout
        let result = if tool.timeout_seconds() > 0 {
            tracing::debug!(
                "⏱️  Executing {} with {}s timeout (note: timeout monitoring not yet implemented for sync tools)",
                tool_name,
                tool.timeout_seconds()
            );
            tool.execute(context)
        } else {
            tracing::debug!("🚀 Executing {} without timeout", tool_name);
            tool.execute(context)
        };

        // Handle result
        match result {
            Ok(mut output) => {
                let duration = start.elapsed();
                output.duration_ms = duration.as_millis() as u64;
                
                tracing::info!(
                    "✅ {} completed successfully in {:.2}s",
                    tool_name,
                    duration.as_secs_f64()
                );
                
                if !output.files_modified.is_empty() {
                    tracing::debug!("  📝 Modified {} files", output.files_modified.len());
                }
                if !output.files_created.is_empty() {
                    tracing::debug!("  ✨ Created {} files", output.files_created.len());
                }
                if !output.files_deleted.is_empty() {
                    tracing::debug!("  🗑️  Deleted {} files", output.files_deleted.len());
                }
                
                // After hook
                tracing::debug!("📝 Running after_execute hook for {}", tool_name);
                tool.after_execute(context, &output)?;
                
                Ok(output)
            }
            Err(e) => {
                let duration = start.elapsed();
                tracing::error!(
                    "❌ {} failed after {:.2}s: {}",
                    tool_name,
                    duration.as_secs_f64(),
                    e
                );
                
                // Error hook
                tracing::debug!("📝 Running on_error hook for {}", tool_name);
                tool.on_error(context, &e)?;
                Err(e)
            }
        }
    }

    /// Check for circular dependencies
    fn check_circular_dependencies(&self) -> Result<()> {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        for tool in &self.tools {
            if !visited.contains(tool.name()) {
                self.check_circular_deps_recursive(tool.name(), &mut visited, &mut stack)?;
            }
        }

        Ok(())
    }

    fn check_circular_deps_recursive(
        &self,
        tool_name: &str,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
    ) -> Result<()> {
        visited.insert(tool_name.to_string());
        stack.insert(tool_name.to_string());

        if let Some(tool) = self.tools.iter().find(|t| t.name() == tool_name) {
            for dep in tool.dependencies() {
                if !visited.contains(&dep) {
                    self.check_circular_deps_recursive(&dep, visited, stack)?;
                } else if stack.contains(&dep) {
                    return Err(anyhow::anyhow!(
                        "Circular dependency detected: {} -> {}",
                        tool_name,
                        dep
                    ));
                }
            }
        }

        stack.remove(tool_name);
        Ok(())
    }

    /// Validate tool dependencies
    fn validate_dependencies(&self) -> Result<()> {
        let tool_names: HashSet<String> = self.tools.iter().map(|t| t.name().to_string()).collect();

        for tool in &self.tools {
            for dep in tool.dependencies() {
                if !tool_names.contains(&dep) {
                    anyhow::bail!(
                        "Tool '{}' requires '{}' but it's not registered",
                        tool.name(),
                        dep
                    );
                }
            }
        }

        Ok(())
    }

    /// Get execution context
    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    /// Get mutable context
    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTool {
        name: String,
        priority: u32,
    }

    impl DxTool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn priority(&self) -> u32 {
            self.priority
        }

        fn execute(&mut self, _ctx: &ExecutionContext) -> Result<ToolOutput> {
            Ok(ToolOutput::success())
        }
    }

    #[test]
    fn test_orchestrator_priority_order() {
        let mut orch = Orchestrator::new("/tmp/test").unwrap();

        orch.register_tool(Box::new(MockTool {
            name: "tool-c".into(),
            priority: 30,
        }))
        .unwrap();
        orch.register_tool(Box::new(MockTool {
            name: "tool-a".into(),
            priority: 10,
        }))
        .unwrap();
        orch.register_tool(Box::new(MockTool {
            name: "tool-b".into(),
            priority: 20,
        }))
        .unwrap();

        let outputs = orch.execute_all().unwrap();

        assert_eq!(outputs.len(), 3);
        assert!(outputs.iter().all(|o| o.success));
    }
}
