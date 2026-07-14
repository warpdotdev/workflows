use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;
use warp_workflows_types::Workflow;

// The module-name derivation is shared with the library (see `workflows/src/lib.rs`) so it can be
// unit tested; build scripts are compiled as a separate crate with no test harness.
#[path = "src/module_name.rs"]
mod module_name;
use module_name::module_name;

/// Generates Workflows as rust files from the yaml stored in /specs. Each Workflow is stored within
/// its own mod within the `generated_workflows` module. Additionally, a function called `workflows`
/// is generated that returns a vector of all the `Workflow`s that were created.
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../specs");
    println!("cargo:rerun-if-changed=src/module_name.rs");

    std::fs::create_dir_all("src/generated_workflows")?;
    let parent_module = std::fs::File::create("src/generated_workflows/mod.rs")?;

    let mut workflows_added = Vec::new();
    // Maps each derived module name to the first spec path that produced it, so colliding specs are
    // caught and reported instead of silently overwriting one another's generated file.
    let mut module_names: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new("../specs") {
        let entry = entry?;
        let valid_yaml_file = entry
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .map_or(false, |extension| extension == "yaml" || extension == "yml");

        if valid_yaml_file {
            let file = File::open(entry.path())?;

            let mmap = unsafe { memmap::Mmap::map(&file) }?;
            let yaml_content = std::str::from_utf8(&mmap)?;

            println!("attempting to generate workflow at {:?}", entry.path());
            let workflow: Workflow = serde_yaml::from_str(yaml_content)?;
            println!("generated workflow is {workflow:?}");

            let raw_name = entry.file_name().to_str().with_context(|| {
                format!("spec file name is not valid UTF-8: {:?}", entry.path())
            })?;
            let file_name = module_name(raw_name);
            println!("file name is {file_name:?}");

            // Two specs whose names normalize to the same module would silently overwrite each
            // other's generated file and emit a duplicate `pub mod`, so fail fast with both paths.
            if let Some(existing) =
                module_names.insert(file_name.clone(), entry.path().to_path_buf())
            {
                bail!(
                    "workflow module name collision: {} and {} both normalize to module name `{}`; rename one of the spec files",
                    existing.display(),
                    entry.path().display(),
                    file_name
                );
            }

            // Create a module for each Workflow within the parent module.
            workflows_added.push(file_name.clone());

            write_workflow(workflow, file_name.as_str())?;

            writeln!(&parent_module, "pub mod {file_name};")?;
        }
    }

    write_workflows_function(parent_module, &mut workflows_added)?;

    println!("running cargo fmt...");
    Command::new("cargo")
        .args(["fmt", "-p", "warp-workflows"])
        .output()?;

    Ok(())
}

/// Writes a `workflows` function into the module at path `parent_modules`. The generated function
/// will look approximately like:
/// ```ignore
/// use warp_workflows_metadata::*;
/// pub fn workflows() -> Vec<Workflow> {
///     vec![
///         foo::workflow(),
///         bar::workflow(),
///         bazz:workflow(),
///     ]
/// }
/// ```
fn write_workflows_function(
    mut parent_module: File,
    workflows_added: &mut Vec<String>,
) -> Result<()> {
    writeln!(parent_module, "\nuse warp_workflows_types::*;")?;
    writeln!(parent_module, "pub fn workflows() -> Vec<Workflow> {{")?;
    writeln!(parent_module, "vec![")?;
    for workflows in workflows_added {
        writeln!(parent_module, "{workflows}::workflow(),")?;
    }
    writeln!(parent_module, "]")?;
    writeln!(parent_module, "}}")?;

    Ok(())
}

/// Writes a Workflow into the path at `src/generated_workflows/<file_name>.rs`. The generated Rust
/// will look like:
/// ```ignore
/// use warp_workflows_metadata::*;
/// pub fn workflow -> Workflow {
///    Workflow {
///      ...
///    }
/// }
/// ```
fn write_workflow(workflow: Workflow, file_name: &str) -> Result<()> {
    let module = std::fs::File::create(format!("src/generated_workflows/{file_name}.rs"))?;

    writeln!(&module, "use warp_workflows_types::*;")?;
    writeln!(&module, r#"pub fn workflow() -> Workflow {{"#)?;
    writeln!(&module, "{}", uneval::to_string(workflow)?)?;
    writeln!(&module, "}}")?;

    Ok(())
}
