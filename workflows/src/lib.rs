mod generated_workflows;

// Shared with `build.rs` via `#[path = "src/module_name.rs"]`. Compiled into the library only
// under `cfg(test)` so its unit tests are exercised by `cargo test` (build scripts have no test
// harness of their own).
#[cfg(test)]
mod module_name;

pub use generated_workflows::workflows;
pub use warp_workflows_types::*;
