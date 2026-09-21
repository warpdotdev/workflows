// Type-level regression test for the exported `Workflow` interface.
//
// These declarations do not run; they exist so that `tsc` fails to compile if
// the multi-value fields (`tags`, `arguments`, `shells`) are ever narrowed back
// to single-element tuple types. Every literal below mirrors an example that is
// already documented in FORMAT.md / README.md or shipped under specs/.
//
// Compile with: npm run check-types

import { Argument, Shell, Workflow } from "../index";

// Mirrors FORMAT.md's own for-loop example, which documents a command with the
// three arguments `variable`, `sequence`, and `command`, alongside a two-tag
// list and two valid shells. Under the old `[string]` / `[Argument]` / `[Shell]`
// tuple types this fails to type-check even though it is the documented format.
const multiValued: Workflow = {
  slug: "for-loop",
  name: "For loop",
  command: "for {{variable}} in {{sequence}}; do\n  {{command}}\ndone",
  tags: ["shell", "loops"],
  description: "Iterate over a sequence",
  arguments: [
    { name: "variable" },
    { name: "sequence" },
    { name: "command" },
  ],
  shells: [Shell.Bash, Shell.Zsh],
  relative_git_url: "/specs/shell/for_loop.yaml",
};

// Mirrors the README's example workflow, which uses a single tag/argument and
// an empty `shells: []` (valid in all shells). A single-element tuple type would
// reject `shells: []` because it requires exactly one element.
const singleValued: Workflow = {
  slug: "brew-rmtree",
  name: "Uninstall a Homebrew package and all of its dependencies",
  command: "brew tap beeftornado/rmtree\nbrew rmtree {{package_name}}",
  tags: ["homebrew"],
  arguments: [{ name: "package_name" }],
  shells: [],
  relative_git_url: "/specs/homebrew/brew_rmtree.yaml",
};

// Reference the declarations so `noUnusedLocals`-style checks stay quiet and the
// `Argument` import is exercised.
const _arg: Argument = multiValued.arguments![0];
void _arg;
void singleValued;
