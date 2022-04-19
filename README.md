# Workflows

The repo for all _public_ Workflows that appear within Warp and within [commands.dev](https://www.commands.dev/).

**To learn how to create local or repository workflows, see [our docs](https://github.com/warpdotdev/warp-internal/pull/2205).**

<img width="736" alt="image" src="https://user-images.githubusercontent.com/4110292/164031239-49f0ec9e-f124-44c4-89e6-6facc9bf9a8f.png">


## What are Workflows?

Workflows are an easier way to execute and share commands within Warp. They are searchable by name, description, or command and are easily parameterized. See our documentation for more details: [https://docs.warp.dev/features/workflows](https://docs.warp.dev/features/workflows)

## How Do I Access Workflows within Warp?

Workflows can be accessed directly within Warp, either through the Command Palette or by pressing `ctrl-shift-r`.

All public workflows (i.e. workflows within this repo) are also available at [commands.dev](https://www.commands.dev/).

## Contributing
Contributions are always welcome! If you have a workflow that would be useful to many Warp users, feel free to send a PR to add a Workflow spec.

All workflows are defined as YAML files within the [`specs/`](specs/) directory. 

### File Format
A comprehensive description of the file format is available in [FORMAT.md](FORMAT.md).
Additionally, see the workflow below as an example to quickly get started:

```yaml
---
# The name of the workflow.
name: Uninstall a Homebrew package and all of its dependencies
# The corresponding command for the workflow. Any arguments should be surrounded with two curly braces. E.g `command {{arg}}`.
command: |-
 brew tap beeftornado/rmtree
 brew rmtree {{package_name}}
# Any tags that the workflow should be categorized with.
tags:
  - homebrew
# A description of the workflow.
description: Uses the external command rmtree to remove a Homebrew package and all of its dependencies
# List of arguments within the command.
arguments:
    # Name of the argument within the command. This must exactly match the name of the argument
    # within the command (without the curly braces).
  - name: package_name
    # The description of the argument.
    description: The name of the package that should be removed
    # The default value for the argument.
    default_value: ~
# The source URL for where the workflow was generated from, if any.
source_url: "https://stackoverflow.com/questions/7323261/uninstall-remove-a-homebrew-package-including-all-its-dependencies"
# The author of the workflow.
author: Ory Band
# The URL of original author of the Workflow. For example, if this workflow was generated from StackOverflow, the `author_url` would be the StackOverflow author's profile page.
author_url: "https://stackoverflow.com/users/207894"
# The valid shells where this workflow should be active. If valid for all shells, this can be left empty.
shells: []
```

### Testing
To test a workflow within Warp before submitting, you can use it as a local workflow within warp.

To do this:
1) Copy the workflow to your local `~/.warp/workflows` directory:
    ```bash
    mkdir -p ~/.warp/workflows && cp {{workflow}}.yaml; ~/.warp/workflows/
    ```
2) Open Warp and open workflows by pressing `ctrl-shift-r` or using the command palette.
3) Click on "My Workflows" on the left to filter for local workflows.
![README md — workflows 2022-04-19 at 11 52 53 AM](https://user-images.githubusercontent.com/4110292/164045025-8eb3dd66-260a-4b12-8a4b-beae563db8ee.jpg)
4) Click on the workflow you've added and ensure all the information is correct.

To quickly test if a workflow file format is valid, you can also build workflows locally to validate the schema is correct:
```
# Download the rust toolchain, if not already installed.
brew install rust-up
rustup-init

# Ensure the workflows can successfully be converted into Rust.
cargo build
```


### What Makes a Useful Workflow?
A good workflow is one that includes a command with many flags or arguments or one that is hard to remember.

Additionally, a workflow _must_ include:

* A descriptive title that includes the name of the command--this is useful for improving the experience of searching for workflows in Warp or [commands.dev](https://www.commands.dev/).
* A tag that accurately categorizes the workflows. Avoid many repetitive tags to improve searchability of workflows within Warp.
* A description for the workflow and each of its arguments, if applicable.
* A default value for each argument, if applicable.
