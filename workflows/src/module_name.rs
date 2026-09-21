use convert_case::{Case, Casing};

/// Derives the Rust module name used for a workflow spec from its file name.
///
/// The spec's *trailing* `.yaml`/`.yml` extension is removed, the remaining stem is converted to
/// `snake_case`, any character outside `[A-Za-z0-9_]` is replaced with `_`, and a leading `_` is
/// added when the result would otherwise be empty or start with an ASCII digit. This guarantees the
/// return value is always a valid Rust identifier, which the code generator relies on when it emits
/// `pub mod <name>;`.
///
/// Only the trailing extension is stripped (rather than every `.yaml`/`.yml` substring), so a file
/// such as `my.yaml.config.yaml` maps to `my_yaml_config` instead of an invalid `my.config`.
pub(crate) fn module_name(file_name: &str) -> String {
    let stem = file_name
        .strip_suffix(".yaml")
        .or_else(|| file_name.strip_suffix(".yml"))
        .unwrap_or(file_name);

    let snake = stem.to_case(Case::Snake);

    let mut sanitized: String = snake
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        sanitized.insert(0, '_');
    }

    sanitized
}

#[cfg(test)]
mod tests {
    use super::module_name;

    #[test]
    fn preserves_existing_snake_case_outputs() {
        // Real spec: specs/cosmwasm/cosmwasm-optimize.yaml
        assert_eq!(module_name("cosmwasm-optimize.yaml"), "cosmwasm_optimize");
        // Real spec: specs/chef/run_cookbook_manually.yml (a `.yml` file)
        assert_eq!(
            module_name("run_cookbook_manually.yml"),
            "run_cookbook_manually"
        );
        // Plain snake identity.
        assert_eq!(module_name("list_directories.yaml"), "list_directories");
    }

    #[test]
    fn strips_only_the_trailing_extension() {
        // Regression: `String::replace(".yaml", "")` used to strip every occurrence and leave the
        // interior dot in place, yielding the invalid module `my.config`.
        assert_eq!(module_name("my.yaml.config.yaml"), "my_yaml_config");
    }

    #[test]
    fn colliding_basenames_map_to_the_same_name() {
        // Documents the collision class the build script now detects and rejects: two specs that
        // differ only in separators normalize to one identifier.
        assert_eq!(
            module_name("collide-test.yaml"),
            module_name("collide_test.yaml")
        );
    }

    #[test]
    fn digit_leading_stem_is_prefixed() {
        // `2fa-setup` snake-cases to `2_fa_setup`; a leading `_` keeps it a valid identifier.
        assert_eq!(module_name("2fa-setup.yaml"), "_2_fa_setup");
    }

    #[test]
    fn output_is_always_a_valid_identifier() {
        let inputs = [
            "cosmwasm-optimize.yaml",
            "run_cookbook_manually.yml",
            "list_directories.yaml",
            "my.yaml.config.yaml",
            "collide-test.yaml",
            "collide_test.yaml",
            "2fa-setup.yaml",
            "weird!!name.yaml",
            ".yaml",
            "123.yml",
        ];

        for input in inputs {
            let name = module_name(input);

            match name.chars().next() {
                None => panic!("empty module name for {input:?}"),
                Some(first) => assert!(
                    first.is_ascii_lowercase() || first == '_',
                    "module name {name:?} for {input:?} starts with an invalid char"
                ),
            }

            assert!(
                name.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "module name {name:?} for {input:?} contains an invalid char"
            );
        }
    }
}
