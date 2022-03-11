export interface Workflow {
  slug: WorkflowSlug;
  name: string;
  command: string;
  tags?: [string];
  description?: string;
  arguments?: [Argument];
  source_url?: string;
  author?: string;
  author_url?: string;
  shells?: [Shell];
  relative_git_url: string;
}

export interface Argument {
  name: string;
  description?: string;
  default_value?: string;
}

export const enum Shell {
  Bash = "bash",
  Fish = "fish",
  Zsh = "zsh",
}

export type WorkflowSlug = string;

function getFileNameOnly(filePath: string): string {
  return filePath.split("/").pop()!.split(".")!.shift()!;
}

export const WORKFLOWS = new Map<WorkflowSlug, Workflow>();

// Using Webpack, dynamically load all the YAML files in the `specs` directory
// as Javascript objects and store the objects within the `WORKFLOWS` map.
const requireContext = require.context(
  "yaml-loader!../specs",
  true,
  /\.ya?ml$/
);

requireContext.keys().forEach((key: string) => {
  const obj = requireContext(key);
  const slug = getFileNameOnly(key);
  const relativeUrl = "/specs" + key.substring(1); // removes '.' from key
  let workflow = {
    ...obj,
    slug,
    relative_git_url: relativeUrl,
  } as Workflow;
  WORKFLOWS.set(slug, workflow);
});
