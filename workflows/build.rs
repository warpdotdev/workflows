use anyhow::Result;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;
use warp_workflows_metadata::Workflow;

/**
Workflow
name: String
accepted_shells: [String], optional
tags: [String], optional (necessary for commands.dev)
command: String
description: String, optional
arguments: [Argument], optional
Attribution source: String, optional
Author: String, required

Argument:
name: String
identifier: String
description: String?, optional
default_value: String?, optional

**/


/// Generates completions specs as rust files from the json stored in /json. Each command signature
/// is stored within its own mod within the commands module. Additionally, a function called
/// `signatures` is generated on the commands mod that returns a vector of all the `Signature`s that
/// were created.
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../specs");

    // Create a commands module that will contain all of the signatures.
    std::fs::create_dir_all("src/generated_workflows")?;
    let parent_module = std::fs::File::create("src/generated_workflows/mod.rs")?;

    let mut workflows_added = Vec::new();

    for entry in WalkDir::new("../specs").max_depth(1) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let file = File::open(entry.path())?;

            let mmap = unsafe { memmap::Mmap::map(&file) }?;
            let yaml_content = std::str::from_utf8(&mmap)?;

            let workflow : Workflow = serde_yaml::from_str(yaml_content)?;
            println!("generated workflow is {:?}", workflow);


            let file_name = entry
                .file_name()
                .to_str()
                .expect("OsStr should convert to str")
                .replace(".yaml", "")
                .to_case(Case::Snake);
            println!("file name is {:?}", file_name);

            // Create a module for each signature within the parent commands file.
            workflows_added.push(file_name.clone());

            write_workflow(workflow, file_name.as_str())?;

            writeln!(&parent_module, "pub mod {};", file_name)?;
        }
    }

    write_workflows_function(parent_module, &mut workflows_added)?;

    Ok(())
}

/// Writes a `signatures` function into the module at path `parent_modules`. The generated function
/// will look approximated like:
/// ```ignore
/// use warp_workflows_metadata::*;
/// pub fn signatures() -> Vec<Signature> {
///     vec![
///         foo::signature(),
///         bar::signature(),
///         bazz:signature(),
///     ]
/// }
/// ```
fn write_workflows_function(
    mut parent_module: File,
    signatures_added: &mut Vec<String>,
) -> Result<()> {
    writeln!(parent_module, "\nuse warp_workflows_metadata::*;")?;
    writeln!(parent_module, "pub fn workflows() -> Vec<Workflow> {{")?;
    writeln!(parent_module, "vec![")?;
    for signature in signatures_added {
        writeln!(parent_module, "{}::workflow(),", signature)?;
    }
    writeln!(parent_module, "]")?;
    writeln!(parent_module, "}}")?;

    Ok(())
}

/// Writes a signature into the path at `src/commands/<file_name>.rs`. The generated Rust will look
/// like:
/// ```ignore
/// use warp_workflows_metadata::*;
/// pub fn signature -> Signature {
///    Signature {
///      ...
///    }
/// }
/// ```
fn write_workflow(workflow: Workflow, file_name: &str) -> Result<()> {
    let module = std::fs::File::create(&format!("src/generated_workflows/{}.rs", file_name))?;

    writeln!(&module, "use warp_workflows_metadata::*;")?;
    writeln!(&module, "\n#[allow(clippy::invisible_characters)]")?;
    writeln!(&module, r#"pub fn workflow() -> Workflow {{"#)?;
    writeln!(
        &module,
        "{}",
        uneval::to_string(workflow)?
    )?;
    writeln!(&module, "}}")?;

    Ok(())
}
