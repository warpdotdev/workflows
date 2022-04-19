# Workflows File Format

The workflow file format is a [yaml](https://yaml.org/) file and must have either a `.yml ` or `yaml` extension. If you're new to YAML and want to learn more, see "[Learn YAML in Y minutes](https://learnxinyminutes.com/docs/yaml/)."

_Compatibility Note_: Warp is still in Beta and this format is subject to change.

## File Name

The file name should be clear and concise and should explain what the workflow does. Words should be separated by underscores and not capitalized.

## File Content

### `name`
---
The name of the Workflow. The workflow should not say, for example `Deletes file`. It should instead be `Delete file`. **Required**.

```yaml
name: Delete file/folder
```

### `command`
----
The command that is executed when the Workflow is selected. **Required**.

```yaml
command: rm -rf {{file}}
```

### `tags`
----
An array of tags that are useful to categorize the Workflow. Optional.

```yaml
tags: ["git", "GitHub"]
```

### `description`
----
The description of the Workflow and what it does. It should expand on top of the name, providing extra information. Optional.

```yaml
description: Recursively deletes a file or a folder.
```

### `source_url`
----
The URL from where the Workflow was originally generated from. This is surfaced in [commands.dev](https://www.commands.dev/) for attribution purposes. Optional.

```yaml
source_url: https://example.com
```

### `author`
----
The original author of the Workflow. For example, if this workflow was generated from StackOverflow, the `author` would be the `author` of the StackOverflow post. This is surfaced in [commands.dev](https://www.commands.dev/) for attribution purposes. Optional.

```yaml
author: Jane Doe
```

### `author_url`
----
The URL of original author of the Workflow. For example, if this workflow was generated from StackOverflow, the `author_url` would be the StackOverflow author's profile page. This is surfaced in [commands.dev](https://www.commands.dev/) for attribution purposes. Optional.

```yaml
author_url: https://stackoverflow.com/users/JaneDoe
```

### `shells`
----
The list of shells where this Workflow is valid. If not specified, the Workflow is assumed to be valid in all shells. This must be one of `zsh`, `bash`, or  `fish`.

```yaml
shells:
  - zsh
  - fish
```

## `arguments`
----
A Workflow can have parameterized arguments to specify pieces of the Workflow that need to be filled in by the user.

You can specify which part of the Workflow command maps to an argument by surrounding it with two curly braces (`{{<argument>}}`).

For example the workflow command:
```bash
for {{variable} in {{sequence}}}; do
  {{command}}
done
```
Includes 3 arguments: `variable`, `sequence`, and `command`.

Example: 
```yaml
name: Example workflow
command: echo {{string}}
arguments:
  - name: string
    description: The value to echo
    default_value: Hello, World!
```

## `arguments.name` 
-----
The name of the argument. The argument name is used within the command to specify the ranges of the argument. Required.


## `arguments.description` 
-----
The description of the argument. This is surfaced in both commands.dev and Warp to help users fill in Workflow arguments. Optional


## `arguments.default_value`
-----
The default value for the argument. If specified, the `default_value` replaces the argument name within the command. Optional