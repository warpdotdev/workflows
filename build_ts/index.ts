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

function getFileNameOnly(filePath: string) : string {
  return filePath.split("/").pop()!.split(".")!.shift()!;
}

const requireContext = require.context(
  "yaml-loader!../specs",
  false,
  /\.ya?ml$/
);

export const WORKFLOWS = new Map<WorkflowSlug, Workflow>();

requireContext.keys().forEach((key: string) => {
  const obj = requireContext(key);
  const slug = getFileNameOnly(key);
  let workflow = {
    ...obj,
    slug,
  } as Workflow;
  WORKFLOWS.set(slug, workflow);
});