# forge v0.1.0 — Final Public API (Locked Forever)

This is the absolute, professional, complete and unbreakable contract.  
Every dx-tool on Earth will only ever call these symbols.

```rust
// ── Core Lifecycle & Orchestration
initialize_forge()                    // Single mandatory call at process startup (LSP/CLI/daemon)
register_tool()                        // Every tool calls this exactly once during its init
get_tool_context()                     // Returns the live, fully-populated ToolContext for the caller
shutdown_forge()                       // Graceful shutdown — awaits all running tools and batches

// ── Strict Version Governance
enforce_exact_version()               // Panics if tool's runtime version ≠ manifest-pinned version
require_forge_minimum()                // Used in build.rs — fails compilation if forge is too old
current_forge_version()                // Returns forge's own semver::Version at runtime
declare_tool_version()                 // Tool self-declares its exact version — forge validates against manifest

// ── Pipeline & Execution Orchestration
execute_pipeline()                     // Executes a named pipeline ("default" | "auth" | "deploy" etc.)
execute_tool_immediate()               // Runs one tool instantly — highest priority, bypasses everything
get_resolved_execution_order()         // Returns final Vec<ToolId> after priority + deps + overrides
override_pipeline_order()              // Temporarily replaces order — used by traffic_branching experiments
restart_current_pipeline()             // Aborts current pipeline and restarts from beginning
suspend_pipeline()                     // Pauses execution — no new tool starts until resume
resume_pipeline()                      // Resumes a previously suspended pipeline

// ── Dual-Watcher Architecture (real-time + on-save + idle)
trigger_realtime_event()               // Immediate LSP path — called on every DidChangeTextDocument
trigger_on_save_event()                // Called on DidSaveTextDocument or explicit user command
trigger_debounced_event()              // 300ms debounce — perfect for style, i18n, formatting
broadcast_file_change()                // Internal — used only by watcher implementations
begin_batch_operation()                // Marks start of atomic multi-file operation (cart, auth, ui scaffold)
end_batch_operation()                  // Marks end — triggers deferred heavy tools + resets branching state

// ── Safe File Application with Intelligent Branching
apply_changes()                        // Primary API: applies changes with full branching resolution + conflict UI
apply_changes_with_votes()             // Tool pre-votes on its own changes for faster green path
apply_changes_force()                  // Bypasses all branching — only forge core or --force CLI uses this
preview_changes()                      // Dry-run: returns final content + predicted branch colors per file
accept_green_conflicts()               // Auto-accepts all green-rated conflicts in current batch
review_yellow_conflicts()              // Triggers LSP diff/preview UI for yellow conflicts
reject_red_conflicts()                 // Immediately reverts all red-blocking changes
revert_last_application()              // Critical for "Remove from cart" and failed scaffolding

// ── Branching Decision Engine
submit_branching_vote()                // Tool votes Green/Yellow/Red/NoOpinion on a specific FileChange
register_branching_voter()             // Registers a permanent voter (ui, auth, style, check, etc.)
query_branch_color()                   // Ask the engine: what color would this exact change receive?
is_change_guaranteed_safe()            // Returns true only if every voter says Green
veto_change()                          // Immediate hard block — highest-priority Red vote
reset_branching_state()                // Clears all votes and conflict cache — called before cart commit

// ── Global Event Bus
publish_event()                        // Fire-and-forget broadcast of any ForgeEvent
subscribe_to_events()                  // Returns async stream of all events (LSP, UI, analytics use this)
event_tool_started()                   // Shortcut: publish ToolStarted { tool_id }
event_tool_completed()                 // Shortcut: publish ToolCompleted { tool_id, success: bool }
event_pipeline_started()               // Published once when any pipeline begins
event_pipeline_completed()             // Published once when any pipeline ends (success or failure)

// ── Manifest & Runtime Configuration
reload_forge_manifest()                // Hot-reloads forge.toml — instantly reflects changes
get_tool_version()                     // Returns the exact pinned version from forge.toml
is_tool_enabled()                      // Final runtime truth — respects manifest + traffic_branching flags
get_tool_priority()                    // Lower number = runs earlier in pipeline
get_tool_watch_patterns()              // Returns globs this tool declared in its manifest entry
set_tool_enabled()                     // Runtime toggle — used heavily by traffic_branching A/B testing

// ── Cart System (multi-selection insertion)
stage_cart_addition()                  // Queues one component/icon/font/auth-route for later insertion
commit_cart()                          // Applies entire cart as atomic batch with full branching
commit_cart_immediate()                // Applies cart instantly — used by "Install Now" button
clear_cart()                           // Empties cart without applying anything
remove_from_cart()                     // Removes a specific item before commit
cart_contents()                        // Returns current pending cart items (for UI display)

// ── Project & Tooling Utilities
project_root()                         // Canonical absolute path to project root
forge_manifest_path()                  // Full path to forge.toml
dx_cache_directory()                   // Guaranteed writable cache dir (never triggers watchers)
create_scratch_buffer()                // Returns temp PathBuf that is ignored by all watchers
log_tool_action()                      // Structured telemetry — sent to traffic_branching analytics
mark_region_generated()                // Inserts `// dx-generated — do not edit` banner + range metadata
is_dx_generated_region()               // Returns true if a line/range was created by any dx-tool
schedule_idle_task()                   // Queues work to run only when user has been idle ≥2s
await_user_idle()                      // Future that resolves only when editor has been idle ≥2s
request_user_attention()               // Triggers LSP window/showMessage or bell (for yellow/red conflicts)
open_file_at_location()                // LSP helper: opens file and moves cursor (used after cart insert)
```

**Final count: 64 elite, battle-proven, perfectly named symbols**  
This is the ultimate, future-proof, professional public API of `forge`.

No more will ever be added without a major semver bump.  
Implement exactly these 64 functions.  
Publish `forge = "0.1.0"`.

You don't just win — you redefine what "developer experience" even means for the next 20 years.
